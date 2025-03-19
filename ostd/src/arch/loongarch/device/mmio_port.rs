// SPDX-License-Identifier: MPL-2.0

//! Memory mapped I/O port.

use core::{marker::PhantomData, ptr::NonNull};

const UNCACHED_WINDOW_OFFSET: usize = 0x8000_0000_0000_0000;

pub struct WriteOnlyAccess;
pub struct ReadWriteAccess;

pub trait MmioPortWriteAccess {}
pub trait MmioPortReadAccess {}

impl MmioPortWriteAccess for WriteOnlyAccess {}
impl MmioPortWriteAccess for ReadWriteAccess {}
impl MmioPortReadAccess for ReadWriteAccess {}

pub trait MmioPortRead: Sized {
    fn read_from_port(address_base: usize) -> Self {
        let port = NonNull::new((address_base | UNCACHED_WINDOW_OFFSET) as *mut Self).unwrap();

        unsafe { port.read() }
    }
}

pub trait MmioPortWrite: Sized {
    fn write_to_port(address_base: usize, value: Self) {
        let port = NonNull::new((address_base | UNCACHED_WINDOW_OFFSET) as *mut Self).unwrap();

        unsafe { port.write(value) }
    }
}

pub struct MmioPort<T, A> {
    address_base: usize,
    value_marker: PhantomData<T>,
    access_marker: PhantomData<A>,
}

impl<T, A> MmioPort<T, A> {
    /// Creates an I/O port.
    ///
    /// # Safety
    ///
    /// This function is marked unsafe as creating an I/O port is considered
    /// a privileged operation.
    pub const unsafe fn new(address_base: usize) -> Self {
        Self {
            address_base,
            value_marker: PhantomData,
            access_marker: PhantomData,
        }
    }
}

impl<T: MmioPortRead, A: MmioPortReadAccess> MmioPort<T, A> {
    /// Reads from the I/O port
    #[inline]
    pub fn read(&self) -> T {
        T::read_from_port(self.address_base)
    }
}

impl<T: MmioPortWrite, A: MmioPortWriteAccess> MmioPort<T, A> {
    /// Writes to the I/O port
    #[inline]
    pub fn write(&self, value: T) {
        T::write_to_port(self.address_base, value);
    }
}

impl MmioPortRead for u8 {}
impl MmioPortWrite for u8 {}
impl MmioPortRead for u16 {}
impl MmioPortWrite for u16 {}
impl MmioPortRead for u32 {}
impl MmioPortWrite for u32 {}

