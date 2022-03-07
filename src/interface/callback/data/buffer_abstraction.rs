//! Abstraction for each device.
use super::BufferDataExt;
use super::Device;

/// Main mode
pub mod main {
    use super::*;
    use crate::interface::callback::BufferMainData;

    /// Read interface for main mode
    #[derive(Debug)]
    pub struct ReadDevices<'a, 'b> {
        /// Channel read buffer for [`Strip1`](Device::Strip1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip1: Option<[&'b [f32]; 2]>,
        /// Channel read buffer for [`Strip2`](Device::Strip2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip2: Option<[&'b [f32]; 2]>,
        /// Channel read buffer for [`Strip3`](Device::Strip3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip3: Option<[&'b [f32]; 2]>,
        /// Channel read buffer for [`Strip4`](Device::Strip4).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip4: Option<[&'b [f32]; 2]>,
        /// Channel read buffer for [`Strip5`](Device::Strip5).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip5: Option<[&'b [f32]; 2]>,
        /// Channel read buffer for [`OutputA1`](Device::OutputA1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a1: Option<[&'b [f32]; 8]>,
        /// Channel read buffer for [`OutputA2`](Device::OutputA2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a2: Option<[&'b [f32]; 8]>,
        /// Channel read buffer for [`OutputA3`](Device::OutputA3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a3: Option<[&'b [f32]; 8]>,
        /// Channel read buffer for [`OutputA4`](Device::OutputA4).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a4: Option<[&'b [f32]; 8]>,
        /// Channel read buffer for [`OutputA5`](Device::OutputA5).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a5: Option<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualOutputB1`](Device::VirtualOutputB1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b1: Option<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualOutputB2`](Device::VirtualOutputB2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b2: Option<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualOutputB3`](Device::VirtualOutputB3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b3: Option<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualInput`](Device::VirtualInput).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_input: Option<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualInputAux`](Device::VirtualInputAux).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_input_aux: Option<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualInput8`](Device::VirtualInput8).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_input8: Option<[&'b [f32]; 8]>,
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
    #[derive(Debug)]
    pub struct WriteDevices<'a, 'b> {
        /// Channel write buffer for [`OutputA1`](Device::OutputA1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a1: Option<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`OutputA2`](Device::OutputA2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a2: Option<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`OutputA3`](Device::OutputA3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a3: Option<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`OutputA4`](Device::OutputA4).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a4: Option<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`OutputA5`](Device::OutputA5).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a5: Option<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`VirtualOutputB1`](Device::VirtualOutputB1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b1: Option<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`VirtualOutputB2`](Device::VirtualOutputB2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b2: Option<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`VirtualOutputB3`](Device::VirtualOutputB3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b3: Option<[&'b mut [f32]; 8]>,
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
                if let (Some(read), Some(write)) = (read, write) {
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
    #[derive(Debug)]
    pub struct ReadDevices<'a, 'b> {
        /// Channel read buffer for [`OutputA1`](Device::OutputA1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a1: Option<[&'b [f32]; 8]>,
        /// Channel read buffer for [`OutputA2`](Device::OutputA2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a2: Option<[&'b [f32]; 8]>,
        /// Channel read buffer for [`OutputA3`](Device::OutputA3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a3: Option<[&'b [f32]; 8]>,
        /// Channel read buffer for [`OutputA4`](Device::OutputA4).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a4: Option<[&'b [f32]; 8]>,
        /// Channel read buffer for [`OutputA5`](Device::OutputA5).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a5: Option<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualOutputB1`](Device::VirtualOutputB1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b1: Option<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualOutputB2`](Device::VirtualOutputB2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b2: Option<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualOutputB3`](Device::VirtualOutputB3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b3: Option<[&'b [f32]; 8]>,
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
    #[derive(Debug)]
    pub struct WriteDevices<'a, 'b> {
        /// Channel write buffer for [`OutputA1`](Device::OutputA1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a1: Option<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`OutputA2`](Device::OutputA2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a2: Option<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`OutputA3`](Device::OutputA3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a3: Option<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`OutputA4`](Device::OutputA4).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a4: Option<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`OutputA5`](Device::OutputA5).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub output_a5: Option<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`VirtualOutputB1`](Device::VirtualOutputB1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b1: Option<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`VirtualOutputB2`](Device::VirtualOutputB2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b2: Option<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`VirtualOutputB3`](Device::VirtualOutputB3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_output_b3: Option<[&'b mut [f32]; 8]>,
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
                if let (Some(read), Some(write)) = (read, write) {
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
    #[derive(Debug)]
    pub struct ReadDevices<'a, 'b> {
        /// Channel read buffer for [`Strip1`](Device::Strip1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip1: Option<[&'b [f32]; 2]>,
        /// Channel read buffer for [`Strip2`](Device::Strip2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip2: Option<[&'b [f32]; 2]>,
        /// Channel read buffer for [`Strip3`](Device::Strip3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip3: Option<[&'b [f32]; 2]>,
        /// Channel read buffer for [`Strip4`](Device::Strip4).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip4: Option<[&'b [f32]; 2]>,
        /// Channel read buffer for [`Strip5`](Device::Strip5).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip5: Option<[&'b [f32]; 2]>,
        /// Channel read buffer for [`VirtualInput`](Device::VirtualInput).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_input: Option<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualInputAux`](Device::VirtualInputAux).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_input_aux: Option<[&'b [f32]; 8]>,
        /// Channel read buffer for [`VirtualInput8`](Device::VirtualInput8).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_input8: Option<[&'b [f32]; 8]>,
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
    #[derive(Debug)]
    pub struct WriteDevices<'a, 'b> {
        /// Channel write buffer for [`Strip1`](Device::Strip1).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip1: Option<[&'b mut [f32]; 2]>,
        /// Channel write buffer for [`Strip2`](Device::Strip2).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip2: Option<[&'b mut [f32]; 2]>,
        /// Channel write buffer for [`Strip3`](Device::Strip3).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip3: Option<[&'b mut [f32]; 2]>,
        /// Channel write buffer for [`Strip4`](Device::Strip4).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip4: Option<[&'b mut [f32]; 2]>,
        /// Channel write buffer for [`Strip5`](Device::Strip5).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub strip5: Option<[&'b mut [f32]; 2]>,
        /// Channel write buffer for [`VirtualInput`](Device::VirtualInput).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_input: Option<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`VirtualInputAux`](Device::VirtualInputAux).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_input_aux: Option<[&'b mut [f32]; 8]>,
        /// Channel write buffer for [`VirtualInput8`](Device::VirtualInput8).
        ///
        /// Is [`None`](Option::None) if the device is not available.
        pub virtual_input8: Option<[&'b mut [f32]; 8]>,
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
                    Device::Strip1 => (
                        read.strip1.as_ref().map(|b| b.as_slice()),
                        self.strip1.as_mut().map(|b| b.as_mut_slice()),
                    ),
                    Device::Strip2 => (
                        read.strip2.as_ref().map(|b| b.as_slice()),
                        self.strip2.as_mut().map(|b| b.as_mut_slice()),
                    ),
                    Device::Strip3 => (
                        read.strip3.as_ref().map(|b| b.as_slice()),
                        self.strip3.as_mut().map(|b| b.as_mut_slice()),
                    ),
                    Device::Strip4 => (
                        read.strip4.as_ref().map(|b| b.as_slice()),
                        self.strip4.as_mut().map(|b| b.as_mut_slice()),
                    ),
                    Device::Strip5 => (
                        read.strip5.as_ref().map(|b| b.as_slice()),
                        self.strip5.as_mut().map(|b| b.as_mut_slice()),
                    ),
                    Device::VirtualInput => (
                        read.virtual_input.as_ref().map(|b| b.as_slice()),
                        self.virtual_input.as_mut().map(|b| b.as_mut_slice()),
                    ),
                    Device::VirtualInputAux => (
                        read.virtual_input_aux.as_ref().map(|b| b.as_slice()),
                        self.virtual_input_aux.as_mut().map(|b| b.as_mut_slice()),
                    ),
                    Device::VirtualInput8 => (
                        read.virtual_input8.as_ref().map(|b| b.as_slice()),
                        self.virtual_input8.as_mut().map(|b| b.as_mut_slice()),
                    ),
                    _ => continue,
                };
                if let (Some(read), Some(write)) = (read, write) {
                    for (read, write) in read.iter().zip(write.iter_mut()) {
                        write.copy_from_slice(read)
                    }
                }
            }
        }
    }
}
