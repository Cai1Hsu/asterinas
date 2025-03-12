// SPDX-License-Identifier: MPL-2.0

//! The LoongArch boot module defines the entrypoints of Asterinas.

pub mod smp;

use core::arch::global_asm;

use crate::boot::call_ostd_main;

global_asm!(include_str!("boot.S"));

#[no_mangle]
pub extern "C" fn loongarch_boot(_cpu_id: usize, _device_tree_paddr: usize) -> ! {
    // TODO

    call_ostd_main();
}
