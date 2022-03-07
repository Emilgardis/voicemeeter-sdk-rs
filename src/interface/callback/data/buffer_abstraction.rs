//! Abstraction for each device.

use self::sealed::{Buffer, BufferMut};
use super::BufferDataExt;
use super::Device;

pub(crate) mod sealed {
    pub trait Buffer<'b> {
        const N: usize;
        fn as_slice(&self) -> &[&'b [f32]];
    }
    pub trait BufferMut<'b> {
        const N: usize;

        fn as_mut_slice(&mut self) -> &mut [&'b mut [f32]];
    }
}

impl<'b, const N: usize> Buffer<'b> for [&'b [f32]; N] {
    const N: usize = N;

    fn as_slice(&self) -> &[&'b [f32]] {
        self
    }
}

impl<'b, const N: usize> BufferMut<'b> for [&'b mut [f32]; N] {
    const N: usize = N;

    fn as_mut_slice(&mut self) -> &mut [&'b mut [f32]] {
        self
    }
}

/// A devices buffer.
pub enum DeviceBuffer<T> {
    /// Device does not exist
    None,
    /// Device buffer
    Buffer(T),
}

impl<T> DeviceBuffer<T> {
    /// Converts from `&DeviceBuffer<T>` to `Option<&T>`.
    pub fn as_opt_ref(&self) -> Option<&T> {
        match self {
            Self::None => None,
            Self::Buffer(t) => Some(t),
        }
    }

    /// Converts from `&mut DeviceBuffer<T>` to `Option<&mut T>`.
    pub fn as_opt_mut(&mut self) -> Option<&mut T> {
        match self {
            Self::None => None,
            Self::Buffer(t) => Some(t),
        }
    }

    /// Converts from `&DeviceBuffer<T>` to `DeviceBuffer<&T>`.
    pub fn as_ref(&self) -> DeviceBuffer<&T> {
        match self {
            Self::None => DeviceBuffer::None,
            Self::Buffer(t) => DeviceBuffer::Buffer(t),
        }
    }

    /// Converts from `&mut DeviceBuffer<T>` to `DeviceBuffer<&mut T>`.
    pub fn as_mut(&mut self) -> DeviceBuffer<&mut T> {
        match self {
            Self::None => DeviceBuffer::None,
            Self::Buffer(t) => DeviceBuffer::Buffer(t),
        }
    }
}

// mutable buffer impls
impl<'b, B> DeviceBuffer<B>
where
    B: BufferMut<'b>,
{
    /// Get the buffer as a mutable slice
    pub fn as_mut_slice(&mut self) -> &mut [&'b mut [f32]] {
        match self {
            Self::None => &mut [],
            Self::Buffer(b) => b.as_mut_slice(),
        }
    }

    /// Given a device, apply a specific function on all channels of the device.
    ///
    /// The function is given the current channel as the first argument
    pub fn apply<'b2, F, B2: Buffer<'b2>>(&mut self, read: &DeviceBuffer<B2>, mut f: F)
    where
        F: FnMut(usize, &'b2 f32) -> f32,
    {
        assert_eq!(B::N, B2::N);
        for (channel, (read, write)) in read
            .as_slice()
            .iter()
            .zip(self.as_mut_slice().iter_mut())
            .enumerate()
        {
            write
                .iter_mut()
                .enumerate()
                .map(|(i, w)| *w = f(channel, &read[i]))
                .for_each(drop)
        }
    }
}

impl<'b, B> DeviceBuffer<B>
where
    B: Buffer<'b>,
{
    /// Get the buffer as a slice
    pub fn as_slice(&self) -> &[&'b [f32]] {
        match self {
            Self::None => &mut [],
            Self::Buffer(b) => b.as_slice(),
        }
    }
}

/// Main mode
pub mod main {
    use super::*;
    use crate::interface::callback::BufferMainData;

    /// Read interface for main mode
    pub struct ReadDevices<'a, 'b> {
        /// Channel read buffer for [`Strip1`](Device::Strip1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip1: DeviceBuffer<[&'b [f32]; 2]>,
        /// Channel read buffer for [`Strip2`](Device::Strip2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip2: DeviceBuffer<[&'b [f32]; 2]>,
        /// Channel read buffer for [`Strip3`](Device::Strip3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip3: DeviceBuffer<[&'b [f32]; 2]>,
        /// Channel read buffer for [`Strip4`](Device::Strip4).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip4: DeviceBuffer<[&'b [f32]; 2]>,
        /// Channel read buffer for [`Strip5`](Device::Strip5).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip5: DeviceBuffer<[&'b [f32]; 2]>,
        /// Channel read buffer for [`OutputA1`](Device::OutputA1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a1: DeviceBuffer<[&'b [f32]; 8]>,
        /// Channel read buffer for [`OutputA2`](Device::OutputA2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a2: DeviceBuffer<[&'b [f32]; 8]>,
        /// Channel read buffer for [`OutputA3`](Device::OutputA3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a3: DeviceBuffer<[&'b [f32]; 8]>,
        /// Channel read buffer for [`OutputA4`](Device::OutputA4).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a4: DeviceBuffer<[&'b [f32]; 8]>,
        /// Channel read buffer for [`OutputA5`](Device::OutputA5).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a5: DeviceBuffer<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualOutputB1`](Device::VirtualOutputB1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b1: DeviceBuffer<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualOutputB2`](Device::VirtualOutputB2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b2: DeviceBuffer<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualOutputB3`](Device::VirtualOutputB3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b3: DeviceBuffer<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualInput`](Device::VirtualInput).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_input: DeviceBuffer<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualInputAux`](Device::VirtualInputAux).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_input_aux: DeviceBuffer<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualInput8`](Device::VirtualInput8).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_input8: DeviceBuffer<[&'b [f32]; 8]>,
        _pd: std::marker::PhantomData<&'a ()>,
    }
    impl<'a, 'b> ReadDevices<'a, 'b> {
        /// Create a new buffer for this mode
        pub(crate) fn new(buffer: &'a BufferMainData) -> Self {
            unsafe {
                Self {
                    strip1: buffer.device_read(&Device::Strip1),
                    strip2: buffer.device_read(&Device::Strip2),
                    strip3: buffer.device_read(&Device::Strip3),
                    strip4: buffer.device_read(&Device::Strip4),
                    strip5: buffer.device_read(&Device::Strip5),
                    output_a1: buffer.device_read(&Device::OutputA1),
                    output_a2: buffer.device_read(&Device::OutputA2),
                    output_a3: buffer.device_read(&Device::OutputA3),
                    output_a4: buffer.device_read(&Device::OutputA4),
                    output_a5: buffer.device_read(&Device::OutputA5),
                    virtual_output_b1: buffer.device_read(&Device::VirtualOutputB1),
                    virtual_output_b2: buffer.device_read(&Device::VirtualOutputB2),
                    virtual_output_b3: buffer.device_read(&Device::VirtualOutputB3),
                    virtual_input: buffer.device_read(&Device::VirtualInput),
                    virtual_input_aux: buffer.device_read(&Device::VirtualInputAux),
                    virtual_input8: buffer.device_read(&Device::VirtualInput8),
                    _pd: <_>::default(),
                }
            }
        }
    }

    /// Write interface for main mode
    pub struct WriteDevices<'a, 'b> {
        /// Channel write buffer for [`OutputA1`](Device::OutputA1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a1: DeviceBuffer<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`OutputA2`](Device::OutputA2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a2: DeviceBuffer<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`OutputA3`](Device::OutputA3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a3: DeviceBuffer<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`OutputA4`](Device::OutputA4).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a4: DeviceBuffer<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`OutputA5`](Device::OutputA5).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a5: DeviceBuffer<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`VirtualOutputB1`](Device::VirtualOutputB1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b1: DeviceBuffer<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`VirtualOutputB2`](Device::VirtualOutputB2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b2: DeviceBuffer<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`VirtualOutputB3`](Device::VirtualOutputB3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b3: DeviceBuffer<[&'b mut [f32]; 8]>,
        _pd: std::marker::PhantomData<&'a ()>,
    }
    impl<'a, 'b> WriteDevices<'a, 'b> {
        /// Create a new buffer for this mode
        pub(crate) fn new(buffer: &'a BufferMainData) -> Self {
            unsafe {
                Self {
                    output_a1: buffer.device_write(&Device::OutputA1),
                    output_a2: buffer.device_write(&Device::OutputA2),
                    output_a3: buffer.device_write(&Device::OutputA3),
                    output_a4: buffer.device_write(&Device::OutputA4),
                    output_a5: buffer.device_write(&Device::OutputA5),
                    virtual_output_b1: buffer.device_write(&Device::VirtualOutputB1),
                    virtual_output_b2: buffer.device_write(&Device::VirtualOutputB2),
                    virtual_output_b3: buffer.device_write(&Device::VirtualOutputB3),
                    _pd: <_>::default(),
                }
            }
        }

        /// Copies data from a read buffer into the output.
        pub fn copy_device_from<'i>(
            &mut self,
            read: &ReadDevices<'_, '_>,
            devices: impl IntoIterator<Item = &'i Device>,
        ) {
            for device in devices {
                // TODO: when stable use let_else.
                let (read, write) = match device {
                    Device::OutputA1 => (read.output_a1.as_ref(), self.output_a1.as_mut()),
                    Device::OutputA2 => (read.output_a2.as_ref(), self.output_a2.as_mut()),
                    Device::OutputA3 => (read.output_a3.as_ref(), self.output_a3.as_mut()),
                    Device::OutputA4 => (read.output_a4.as_ref(), self.output_a4.as_mut()),
                    Device::OutputA5 => (read.output_a5.as_ref(), self.output_a5.as_mut()),
                    Device::VirtualOutputB1 => (
                        read.virtual_output_b1.as_ref(),
                        self.virtual_output_b1.as_mut(),
                    ),
                    Device::VirtualOutputB2 => (
                        read.virtual_output_b2.as_ref(),
                        self.virtual_output_b2.as_mut(),
                    ),
                    Device::VirtualOutputB3 => (
                        read.virtual_output_b3.as_ref(),
                        self.virtual_output_b3.as_mut(),
                    ),
                    _ => continue,
                };
                if let (DeviceBuffer::Buffer(read), DeviceBuffer::Buffer(write)) = (read, write) {
                    for (read, write) in read.iter().zip(write.iter_mut()) {
                        write.copy_from_slice(read)
                    }
                }
            }
        }
    }
}

/// Output mode
pub mod output {
    use super::*;
    use crate::interface::callback::BufferOutData;

    /// Read interface for output mode
    pub struct ReadDevices<'a, 'b> {
        /// Channel read buffer for [`OutputA1`](Device::OutputA1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a1: DeviceBuffer<[&'b [f32]; 8]>,
        /// Channel read buffer for [`OutputA2`](Device::OutputA2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a2: DeviceBuffer<[&'b [f32]; 8]>,
        /// Channel read buffer for [`OutputA3`](Device::OutputA3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a3: DeviceBuffer<[&'b [f32]; 8]>,
        /// Channel read buffer for [`OutputA4`](Device::OutputA4).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a4: DeviceBuffer<[&'b [f32]; 8]>,
        /// Channel read buffer for [`OutputA5`](Device::OutputA5).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a5: DeviceBuffer<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualOutputB1`](Device::VirtualOutputB1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b1: DeviceBuffer<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualOutputB2`](Device::VirtualOutputB2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b2: DeviceBuffer<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualOutputB3`](Device::VirtualOutputB3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b3: DeviceBuffer<[&'b [f32]; 8]>,
        _pd: std::marker::PhantomData<&'a ()>,
    }
    impl<'a, 'b> ReadDevices<'a, 'b> {
        /// Create a new buffer for this mode
        pub(crate) fn new(buffer: &'a BufferOutData) -> Self {
            unsafe {
                Self {
                    output_a1: buffer.device_read(&Device::OutputA1),
                    output_a2: buffer.device_read(&Device::OutputA2),
                    output_a3: buffer.device_read(&Device::OutputA3),
                    output_a4: buffer.device_read(&Device::OutputA4),
                    output_a5: buffer.device_read(&Device::OutputA5),
                    virtual_output_b1: buffer.device_read(&Device::VirtualOutputB1),
                    virtual_output_b2: buffer.device_read(&Device::VirtualOutputB2),
                    virtual_output_b3: buffer.device_read(&Device::VirtualOutputB3),
                    _pd: <_>::default(),
                }
            }
        }
    }

    /// Write interface for output mode
    pub struct WriteDevices<'a, 'b> {
        /// Channel write buffer for [`OutputA1`](Device::OutputA1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a1: DeviceBuffer<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`OutputA2`](Device::OutputA2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a2: DeviceBuffer<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`OutputA3`](Device::OutputA3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a3: DeviceBuffer<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`OutputA4`](Device::OutputA4).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a4: DeviceBuffer<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`OutputA5`](Device::OutputA5).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a5: DeviceBuffer<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`VirtualOutputB1`](Device::VirtualOutputB1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b1: DeviceBuffer<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`VirtualOutputB2`](Device::VirtualOutputB2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b2: DeviceBuffer<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`VirtualOutputB3`](Device::VirtualOutputB3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b3: DeviceBuffer<[&'b mut [f32]; 8]>,
        _pd: std::marker::PhantomData<&'a ()>,
    }
    impl<'a, 'b> WriteDevices<'a, 'b> {
        /// Create a new buffer for this mode
        pub(crate) fn new(buffer: &'a BufferOutData) -> Self {
            unsafe {
                Self {
                    output_a1: buffer.device_write(&Device::OutputA1),
                    output_a2: buffer.device_write(&Device::OutputA2),
                    output_a3: buffer.device_write(&Device::OutputA3),
                    output_a4: buffer.device_write(&Device::OutputA4),
                    output_a5: buffer.device_write(&Device::OutputA5),
                    virtual_output_b1: buffer.device_write(&Device::VirtualOutputB1),
                    virtual_output_b2: buffer.device_write(&Device::VirtualOutputB2),
                    virtual_output_b3: buffer.device_write(&Device::VirtualOutputB3),
                    _pd: <_>::default(),
                }
            }
        }

        /// Copies data from a read buffer into the output.
        pub fn copy_device_from(
            &mut self,
            read: &ReadDevices<'_, '_>,
            devices: impl IntoIterator<Item = Device>,
        ) {
            for device in devices {
                // TODO: when stable use let_else.
                let (read, write) = match device {
                    Device::OutputA1 => (read.output_a1.as_ref(), self.output_a1.as_mut()),
                    Device::OutputA2 => (read.output_a2.as_ref(), self.output_a2.as_mut()),
                    Device::OutputA3 => (read.output_a3.as_ref(), self.output_a3.as_mut()),
                    Device::OutputA4 => (read.output_a4.as_ref(), self.output_a4.as_mut()),
                    Device::OutputA5 => (read.output_a5.as_ref(), self.output_a5.as_mut()),
                    Device::VirtualOutputB1 => (
                        read.virtual_output_b1.as_ref(),
                        self.virtual_output_b1.as_mut(),
                    ),
                    Device::VirtualOutputB2 => (
                        read.virtual_output_b2.as_ref(),
                        self.virtual_output_b2.as_mut(),
                    ),
                    Device::VirtualOutputB3 => (
                        read.virtual_output_b3.as_ref(),
                        self.virtual_output_b3.as_mut(),
                    ),
                    _ => continue,
                };
                if let (DeviceBuffer::Buffer(read), DeviceBuffer::Buffer(write)) = (read, write) {
                    for (read, write) in read.iter().zip(write.iter_mut()) {
                        write.copy_from_slice(read)
                    }
                }
            }
        }
    }
}

/// Input mode
pub mod input {
    use super::*;
    use crate::interface::callback::BufferInData;

    /// Read interface for input mode
    pub struct ReadDevices<'a, 'b> {
        /// Channel read buffer for [`Strip1`](Device::Strip1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip1: DeviceBuffer<[&'b [f32]; 2]>,
        /// Channel read buffer for [`Strip2`](Device::Strip2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip2: DeviceBuffer<[&'b [f32]; 2]>,
        /// Channel read buffer for [`Strip3`](Device::Strip3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip3: DeviceBuffer<[&'b [f32]; 2]>,
        /// Channel read buffer for [`Strip4`](Device::Strip4).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip4: DeviceBuffer<[&'b [f32]; 2]>,
        /// Channel read buffer for [`Strip5`](Device::Strip5).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip5: DeviceBuffer<[&'b [f32]; 2]>,
        /// Channel read buffer for [`VirtualInput`](Device::VirtualInput).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_input: DeviceBuffer<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualInputAux`](Device::VirtualInputAux).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_input_aux: DeviceBuffer<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualInput8`](Device::VirtualInput8).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_input8: DeviceBuffer<[&'b [f32]; 8]>,
        _pd: std::marker::PhantomData<&'a ()>,
    }
    impl<'a, 'b> ReadDevices<'a, 'b> {
        /// Create a new buffer for this mode
        pub(crate) fn new(buffer: &'a BufferInData) -> Self {
            unsafe {
                Self {
                    strip1: buffer.device_read(&Device::Strip1),
                    strip2: buffer.device_read(&Device::Strip2),
                    strip3: buffer.device_read(&Device::Strip3),
                    strip4: buffer.device_read(&Device::Strip4),
                    strip5: buffer.device_read(&Device::Strip5),
                    virtual_input: buffer.device_read(&Device::VirtualInput),
                    virtual_input_aux: buffer.device_read(&Device::VirtualInputAux),
                    virtual_input8: buffer.device_read(&Device::VirtualInput8),
                    _pd: <_>::default(),
                }
            }
        }
    }

    /// Write interface for input mode
    pub struct WriteDevices<'a, 'b> {
        /// Channel write buffer for [`Strip1`](Device::Strip1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip1: DeviceBuffer<[&'b mut [f32]; 2]>,
        /// Channel write buffer for [`Strip2`](Device::Strip2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip2: DeviceBuffer<[&'b mut [f32]; 2]>,
        /// Channel write buffer for [`Strip3`](Device::Strip3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip3: DeviceBuffer<[&'b mut [f32]; 2]>,
        /// Channel write buffer for [`Strip4`](Device::Strip4).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip4: DeviceBuffer<[&'b mut [f32]; 2]>,
        /// Channel write buffer for [`Strip5`](Device::Strip5).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip5: DeviceBuffer<[&'b mut [f32]; 2]>,
        /// Channel write buffer for [`VirtualInput`](Device::VirtualInput).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_input: DeviceBuffer<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`VirtualInputAux`](Device::VirtualInputAux).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_input_aux: DeviceBuffer<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`VirtualInput8`](Device::VirtualInput8).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_input8: DeviceBuffer<[&'b mut [f32]; 8]>,
        _pd: std::marker::PhantomData<&'a ()>,
    }
    impl<'a, 'b> WriteDevices<'a, 'b> {
        /// Create a new buffer for this mode
        pub(crate) fn new(buffer: &'a BufferInData) -> Self {
            unsafe {
                Self {
                    strip1: buffer.device_write(&Device::Strip1),
                    strip2: buffer.device_write(&Device::Strip2),
                    strip3: buffer.device_write(&Device::Strip3),
                    strip4: buffer.device_write(&Device::Strip4),
                    strip5: buffer.device_write(&Device::Strip5),
                    virtual_input: buffer.device_write(&Device::VirtualInput),
                    virtual_input_aux: buffer.device_write(&Device::VirtualInputAux),
                    virtual_input8: buffer.device_write(&Device::VirtualInput8),
                    _pd: <_>::default(),
                }
            }
        }

        /// Copies data from a read buffer into the output.
        pub fn copy_device_from(
            &mut self,
            read: &ReadDevices<'_, '_>,
            devices: impl IntoIterator<Item = Device>,
        ) {
            for device in devices {
                // TODO: when stable use let_else.
                let (read, write) = match device {
                    Device::Strip1 => (read.strip1.as_slice(), self.strip1.as_mut_slice()),
                    Device::Strip2 => (read.strip2.as_slice(), self.strip2.as_mut_slice()),
                    Device::Strip3 => (read.strip3.as_slice(), self.strip3.as_mut_slice()),
                    Device::Strip4 => (read.strip4.as_slice(), self.strip4.as_mut_slice()),
                    Device::Strip5 => (read.strip5.as_slice(), self.strip5.as_mut_slice()),
                    Device::VirtualInput => (
                        read.virtual_input.as_slice(),
                        self.virtual_input.as_mut_slice(),
                    ),
                    Device::VirtualInputAux => (
                        read.virtual_input_aux.as_slice(),
                        self.virtual_input_aux.as_mut_slice(),
                    ),
                    Device::VirtualInput8 => (
                        read.virtual_input8.as_slice(),
                        self.virtual_input8.as_mut_slice(),
                    ),
                    _ => continue,
                };
                for (read, write) in read.iter().zip(write.iter_mut()) {
                    write.copy_from_slice(read)
                }
            }
        }
    }
}
