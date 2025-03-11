// SPDX-License-Identifier: MPL-2.0

//! Memory mapped I/O port.

use core::{marker::PhantomData, ptr::NonNull};

const UNCACHED_WINDOW_OFFSET: usize = 0x8000_0000_0000_0000;

pub struct WriteOnlyAccess;
pub struct ReadWriteAccess;

pub trait MemPortWriteAccess {}
pub trait MemPortReadAccess {}

impl MemPortWriteAccess for WriteOnlyAccess {}
impl MemPortWriteAccess for ReadWriteAccess {}
impl MemPortReadAccess for ReadWriteAccess {}

pub trait MemPortRead: Sized {
    fn read_from_port(address_base: usize) -> Self {
        let port = NonNull::new((address_base | UNCACHED_WINDOW_OFFSET) as *mut Self).unwrap();

        unsafe { port.read() }
    }
}

pub trait MemPortWrite: Sized {
    fn write_to_port(address_base: usize, value: Self) {
        let port = NonNull::new((address_base | UNCACHED_WINDOW_OFFSET) as *mut Self).unwrap();

        unsafe { port.write(value) }
    }
}

pub struct MemPort<T, A> {
    address_base: usize,
    value_marker: PhantomData<T>,
    access_marker: PhantomData<A>,
}

impl<T, A> MemPort<T, A> {
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

impl<T: MemPortRead, A: MemPortReadAccess> MemPort<T, A> {
    /// Reads from the I/O port
    #[inline]
    pub fn read(&self) -> T {
        T::read_from_port(self.address_base)
    }
}

impl<T: MemPortWrite, A: MemPortWriteAccess> MemPort<T, A> {
    /// Writes to the I/O port
    #[inline]
    pub fn write(&self, value: T) {
        T::write_to_port(self.address_base, value);
    }
}

impl MemPortRead for u8 {}
impl MemPortWrite for u8 {}
impl MemPortRead for u16 {}
impl MemPortWrite for u16 {}
impl MemPortRead for u32 {}
impl MemPortWrite for u32 {}

