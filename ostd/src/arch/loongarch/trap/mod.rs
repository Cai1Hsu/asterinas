// SPDX-License-Identifier: MPL-2.0

//! Handles trap.

mod trap;

pub use trap::{GeneralRegs, TrapFrame, UserContext};

use crate::cpu_local_cell;

cpu_local_cell! {
    static IS_KERNEL_INTERRUPTED: bool = false;
}

/// Initialize interrupt handling on RISC-V.
pub unsafe fn init(_on_bsp: bool) {
    self::trap::init();
}

/// Returns true if this function is called within the context of an IRQ handler
/// and the IRQ occurs while the CPU is executing in the kernel mode.
/// Otherwise, it returns false.
pub fn is_kernel_interrupted() -> bool {
    IS_KERNEL_INTERRUPTED.load()
}

/// Handle traps (only from kernel).
#[no_mangle]
extern "C" fn trap_handler(f: &mut TrapFrame) {
    use loongArch64::register::estat::Trap;
    use loongArch64::register::badv;

    match loongArch64::register::estat::read().cause() {
        Trap::Interrupt(_) => {
            IS_KERNEL_INTERRUPTED.store(true);
            todo!();
            IS_KERNEL_INTERRUPTED.store(false);
        }
        Trap::Exception(e) => {
            let badv = badv::read().vaddr();

            panic!(
                "Cannot handle kernel cpu exception: {e:?}. badv: {badv:#x}, trapframe: {f:#x?}.",
            );
        }
        _ => todo!(),
    }
}