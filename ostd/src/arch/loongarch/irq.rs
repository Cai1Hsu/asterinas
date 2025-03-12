// SPDX-License-Identifier: MPL-2.0

//! Interrupts.

use alloc::vec::Vec;

use id_alloc::IdAlloc;
use spin::Once;

use crate::{
    cpu::CpuId,
    sync::{PreemptDisabled, SpinLock, SpinLockGuard},
};

/// The global allocator for software defined IRQ lines.
pub(crate) static IRQ_ALLOCATOR: Once<SpinLock<IdAlloc>> = Once::new();

pub(crate) static IRQ_LIST: Once<Vec<IrqLine>> = Once::new();

/// An interrupt request (IRQ) line.
#[derive(Debug)]
pub(crate) struct IrqLine {}

impl IrqLine {
    /// Acquire an interrupt request line.
    ///
    /// # Safety
    ///
    /// This function is marked unsafe as manipulating interrupt lines is
    /// considered a dangerous operation.
    #[expect(clippy::redundant_allocation)]
    pub unsafe fn acquire(_irq_num: u8) -> alloc::sync::Arc<&'static Self> {
        todo!()
    }

    /// Get the IRQ number.
    pub fn num(&self) -> u8 {
        todo!()
    }

    pub fn callback_list(
        &self,
    ) -> SpinLockGuard<alloc::vec::Vec<CallbackElement>, PreemptDisabled> {
        todo!()
    }
}

/// The handle to a registered callback for a IRQ line.
///
/// When the handle is dropped, the callback will be unregistered automatically.
#[must_use]
#[derive(Debug)]
pub struct IrqCallbackHandle {}

pub struct CallbackElement {}

pub(crate) fn init() {
    // TODO
}

pub(crate) fn enable_local() {
    // TODO
}

pub(crate) fn disable_local() {
    // TODO
}

pub(crate) fn is_local_enabled() -> bool {
    // TODO
    false
}

/// Sends a general inter-processor interrupt (IPI) to the specified CPU.
///
/// # Safety
///
/// The caller must ensure that the CPU ID and the interrupt number corresponds
/// to a safe function to call.
pub(crate) unsafe fn send_ipi(_cpu_id: CpuId, _irq_num: u8) {
    unimplemented!()
}
