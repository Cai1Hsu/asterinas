// SPDX-License-Identifier: MPL-2.0

//! The LoongArch boot module defines the entrypoints of Asterinas.

pub mod smp;

use core::arch::global_asm;

use loongArch64::register::{pwch, pwcl, stlbps, tlbidx, tlbrehi};

use crate::{boot::call_ostd_main, early_println};

global_asm!(include_str!("boot.S"));

#[no_mangle]
extern "C" fn setup_tlb() {
    const PS_4K: usize = 0x0c;

    tlbidx::set_ps(PS_4K);
    stlbps::set_ps(PS_4K);
    tlbrehi::set_ps(PS_4K);

    // Set Page table entry width
    pwcl::set_pte_width(8);
    // Set Page table width and offset
    pwcl::set_ptbase(12);
    pwcl::set_ptwidth(9);
    pwcl::set_dir1_base(21);
    pwcl::set_dir1_width(9);
    pwcl::set_dir2_base(30);
    pwcl::set_dir2_width(9);
    pwch::set_dir3_base(39);
    pwch::set_dir3_width(9);
}

#[no_mangle]
pub extern "C" fn loongarch_boot(_cpu_id: usize, _device_tree_paddr: usize) -> ! {
    // TODO

    call_ostd_main();
}
