//! Abstraction for each device.

use self::sealed::{Buffer, BufferMut};
use super::BufferDataExt;
use super::Device;
use super::{Input, Main, Output};

pub(crate) mod sealed {
    pub trait Buffer<'a, 'b>
    where
        'b: 'a,
    {
        fn as_slice(&'a self) -> &'a [&'b [f32]];
        fn len(&'a self) -> usize {
            self.as_slice().len()
        }
    }
    pub trait BufferMut<'a, 'b>
    where
        'b: 'a,
    {
        fn as_mut_slice(&mut self) -> &mut [&'b mut [f32]];
        fn len(&'a self) -> usize;
    }
}

impl<'a, 'b: 'a, const N: usize> Buffer<'a, 'b> for [&'b [f32]; N] {
    fn as_slice(&'a self) -> &'a [&'b [f32]] {
        self
    }

    fn len(&'a self) -> usize {
        N
    }
}

impl<'a, 'b: 'a> Buffer<'a, 'b> for &'a [&'b [f32]] {
    fn as_slice(&'a self) -> &'a [&'b [f32]] {
        *self
    }
}

impl<'a, 'b: 'a, const N: usize> BufferMut<'a, 'b> for [&'b mut [f32]; N] {
    fn as_mut_slice(&mut self) -> &mut [&'b mut [f32]] {
        self
    }

    fn len(&'a self) -> usize {
        N
    }
}

impl<'a, 'b: 'a> BufferMut<'a, 'b> for &'a mut [&'b mut [f32]] {
    fn as_mut_slice(&mut self) -> &mut [&'b mut [f32]] {
        *self
    }
    fn len(&'a self) -> usize {
        self.as_ref().len()
    }
}

/// A devices buffer.
pub enum DeviceBuffer<T> {
    /// Device does not exist
    None,
    /// Device buffer
    Buffer(T),
}

impl<T> Default for DeviceBuffer<T> {
    fn default() -> Self {
        DeviceBuffer::None
    }
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

    /// Returns `true` if the device buffer is [`None`].
    ///
    /// [`None`]: DeviceBuffer::None
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    /// Returns `true` if the device buffer is [`Buffer`].
    ///
    /// [`Buffer`]: DeviceBuffer::Buffer
    pub fn is_buffer(&self) -> bool {
        matches!(self, Self::Buffer(..))
    }
}

// mutable buffer impls
impl<'a, 'b: 'a, B> DeviceBuffer<B>
where
    B: BufferMut<'a, 'b>,
{
    /// Make the buffer into a mutable slice
    pub fn to_mut_slice(&'a mut self) -> &'a mut [&'b mut [f32]] {
        match self {
            Self::None => &mut [],
            Self::Buffer(b) => b.as_mut_slice(),
        }
    }

    /// Get the buffer as a mutable slice
    pub fn as_mut_slice(&mut self) -> DeviceBuffer<&mut [&'b mut [f32]]> {
        match self {
            Self::None => DeviceBuffer::None,
            Self::Buffer(b) => DeviceBuffer::Buffer(b.as_mut_slice()),
        }
    }
    /// Given a device, apply a specific function on all channels
    ///
    /// The function is given the current channel as the first argument,
    /// the samples in the read buffer as the second argument, and the write buffer as the third argument.
    pub fn apply<F, B2: Buffer<'a, 'b>>(&'a mut self, read: &'a DeviceBuffer<B2>, mut f: F)
    where
        F: FnMut(usize, &[f32], &mut [f32]),
    {
        // FIXME: Assert that the sizes are equal for optimization
        for (channel, (read, write)) in read
            .to_slice()
            .iter()
            .zip(self.to_mut_slice().iter_mut())
            .enumerate()
        {
            f(channel, read, write);
        }
    }

    /// Given a device, apply a specific function on all channels and their samples.
    ///
    /// The function is given the current channel as the first argument,
    /// the current sample in the read buffer as the second argument, and the current sample in the write buffer as the third argument.
    pub fn apply_all_samples<F, B2: Buffer<'a, 'b>>(
        &'a mut self,
        read: &'a DeviceBuffer<B2>,
        mut f: F,
    ) where
        F: FnMut(usize, &f32, &mut f32),
    {
        // FIXME: Assert that the sizes are equal for optimization
        for (channel, (read, write)) in read
            .to_slice()
            .iter()
            .zip(self.to_mut_slice().iter_mut())
            .enumerate()
        {
            for (i, sample) in write.iter_mut().enumerate() {
                f(channel, &read[i], sample);
            }
        }
    }
}

impl<'a, 'b: 'a, B> DeviceBuffer<B>
where
    B: Buffer<'a, 'b>,
{
    /// Make the buffer into a slice
    pub fn to_slice(&'a self) -> &'a [&'b [f32]] {
        match self {
            Self::None => &mut [],
            Self::Buffer(b) => b.as_slice(),
        }
    }

    /// Get the buffer as a mutable slice
    pub fn as_slice(&'a self) -> DeviceBuffer<&'a [&'b [f32]]> {
        match self {
            Self::None => DeviceBuffer::None,
            Self::Buffer(b) => DeviceBuffer::Buffer(b.as_slice()),
        }
    }
}

/// Main mode
pub mod main {
    use super::*;

    /// Read interface for main mode
    #[derive(Default)]
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
        pub(crate) unsafe fn new(buffer: &'_ mut Main) -> Self {
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
        /// Grab the device buffer for a specific device
        pub fn device(&'a self, device: &Device) -> DeviceBuffer<&'a [&'b [f32]]> {
            match device {
                Device::Strip1 => self.strip1.as_slice(),
                Device::Strip2 => self.strip2.as_slice(),
                Device::Strip3 => self.strip3.as_slice(),
                Device::Strip4 => self.strip4.as_slice(),
                Device::Strip5 => self.strip5.as_slice(),
                Device::OutputA1 => self.output_a1.as_slice(),
                Device::OutputA2 => self.output_a2.as_slice(),
                Device::OutputA3 => self.output_a3.as_slice(),
                Device::OutputA4 => self.output_a4.as_slice(),
                Device::OutputA5 => self.output_a5.as_slice(),
                Device::VirtualOutputB1 => self.virtual_output_b1.as_slice(),
                Device::VirtualOutputB2 => self.virtual_output_b2.as_slice(),
                Device::VirtualOutputB3 => self.virtual_output_b3.as_slice(),
                Device::VirtualInput => self.virtual_input.as_slice(),
                Device::VirtualInputAux => self.virtual_input_aux.as_slice(),
                Device::VirtualInput8 => self.virtual_input8.as_slice(),
            }
        }
    }

    /// Write interface for main mode
    #[derive(Default)]
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
        pub(crate) unsafe fn new(buffer: &'_ mut Main) -> Self {
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
                let DeviceBuffer::Buffer(write) = self.device_mut(device) else {
                    continue;
                };
                let DeviceBuffer::Buffer(read) = read.device(device) else {
                    continue;
                };
                assert_eq!(read.len(), write.len());
                for (read, write) in read.iter().zip(write.iter_mut()) {
                    write.copy_from_slice(read)
                }
            }
        }

        /// Grab the device buffer for a specific device
        pub fn device_mut(&mut self, device: &Device) -> DeviceBuffer<&mut [&'b mut [f32]]> {
            match device {
                Device::OutputA1 => self.output_a1.as_mut_slice(),
                Device::OutputA2 => self.output_a2.as_mut_slice(),
                Device::OutputA3 => self.output_a3.as_mut_slice(),
                Device::OutputA4 => self.output_a4.as_mut_slice(),
                Device::OutputA5 => self.output_a5.as_mut_slice(),
                Device::VirtualOutputB1 => self.virtual_output_b1.as_mut_slice(),
                Device::VirtualOutputB2 => self.virtual_output_b2.as_mut_slice(),
                Device::VirtualOutputB3 => self.virtual_output_b3.as_mut_slice(),
                _ => DeviceBuffer::None,
            }
        }
    }
}

/// Output mode
pub mod output {
    use super::*;

    /// Read interface for output mode
    #[derive(Default)]
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
        pub(crate) unsafe fn new(buffer: &'_ mut Output) -> Self {
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
        /// Grab the device buffer for a specific device
        pub fn device(&'a self, device: &Device) -> DeviceBuffer<&'a [&'b [f32]]> {
            match device {
                Device::OutputA1 => self.output_a1.as_slice(),
                Device::OutputA2 => self.output_a2.as_slice(),
                Device::OutputA3 => self.output_a3.as_slice(),
                Device::OutputA4 => self.output_a4.as_slice(),
                Device::OutputA5 => self.output_a5.as_slice(),
                Device::VirtualOutputB1 => self.virtual_output_b1.as_slice(),
                Device::VirtualOutputB2 => self.virtual_output_b2.as_slice(),
                Device::VirtualOutputB3 => self.virtual_output_b3.as_slice(),
                _ => DeviceBuffer::None,
            }
        }
    }

    /// Write interface for output mode
    #[derive(Default)]
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
        pub(crate) unsafe fn new(buffer: &'_ mut Output) -> Self {
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
                let DeviceBuffer::Buffer(write) = self.device_mut(device) else {
                    continue;
                };
                let DeviceBuffer::Buffer(read) = read.device(device) else {
                    continue;
                };
                assert_eq!(read.len(), write.len());
                for (read, write) in read.iter().zip(write.iter_mut()) {
                    write.copy_from_slice(read)
                }
            }
        }
        /// Grab the device buffer for a specific device
        pub fn device_mut(&mut self, device: &Device) -> DeviceBuffer<&mut [&'b mut [f32]]> {
            match device {
                Device::OutputA1 => self.output_a1.as_mut_slice(),
                Device::OutputA2 => self.output_a2.as_mut_slice(),
                Device::OutputA3 => self.output_a3.as_mut_slice(),
                Device::OutputA4 => self.output_a4.as_mut_slice(),
                Device::OutputA5 => self.output_a5.as_mut_slice(),
                Device::VirtualOutputB1 => self.virtual_output_b1.as_mut_slice(),
                Device::VirtualOutputB2 => self.virtual_output_b2.as_mut_slice(),
                Device::VirtualOutputB3 => self.virtual_output_b3.as_mut_slice(),
                _ => DeviceBuffer::None,
            }
        }
    }
}

/// Input mode
pub mod input {
    use super::*;

    /// Read interface for input mode
    #[derive(Default)]
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
        pub(crate) unsafe fn new(buffer: &'_ mut Input) -> Self {
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

        /// Grab the device buffer for a specific device
        pub fn device(&'a self, device: &Device) -> DeviceBuffer<&'a [&'b [f32]]> {
            match device {
                Device::Strip1 => self.strip1.as_slice(),
                Device::Strip2 => self.strip2.as_slice(),
                Device::Strip3 => self.strip3.as_slice(),
                Device::Strip4 => self.strip4.as_slice(),
                Device::Strip5 => self.strip5.as_slice(),
                Device::VirtualInput => self.virtual_input.as_slice(),
                Device::VirtualInputAux => self.virtual_input_aux.as_slice(),
                Device::VirtualInput8 => self.virtual_input8.as_slice(),
                _ => DeviceBuffer::None,
            }
        }
    }

    /// Write interface for input mode
    #[derive(Default)]
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
        pub(crate) unsafe fn new(buffer: &'_ mut Input) -> Self {
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
        pub fn copy_device_from<'i>(
            &mut self,
            read: &ReadDevices<'_, '_>,
            devices: impl IntoIterator<Item = &'i Device>,
        ) {
            for device in devices {
                let DeviceBuffer::Buffer(write) = self.device_mut(device) else {
                    continue;
                };
                let DeviceBuffer::Buffer(read) = read.device(device) else {
                    continue;
                };
                assert_eq!(read.len(), write.len());
                for (read, write) in read.iter().zip(write.iter_mut()) {
                    write.copy_from_slice(read)
                }
            }
        }
        /// Grab the device buffer for a specific device
        pub fn device_mut(&mut self, device: &Device) -> DeviceBuffer<&mut [&'b mut [f32]]> {
            match device {
                Device::Strip1 => self.strip1.as_mut_slice(),
                Device::Strip2 => self.strip2.as_mut_slice(),
                Device::Strip3 => self.strip3.as_mut_slice(),
                Device::Strip4 => self.strip4.as_mut_slice(),
                Device::Strip5 => self.strip5.as_mut_slice(),
                Device::VirtualInput => self.virtual_input.as_mut_slice(),
                Device::VirtualInputAux => self.virtual_input_aux.as_mut_slice(),
                Device::VirtualInput8 => self.virtual_input8.as_mut_slice(),
                _ => DeviceBuffer::None,
            }
        }
    }
}
