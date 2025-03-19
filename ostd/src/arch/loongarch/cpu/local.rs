// SPDX-License-Identifier: MPL-2.0

//! Architecture dependent CPU-local information utilities.

pub(crate) unsafe fn set_base(addr: u64) {
    core::arch::asm!(
        "move $r21, {addr}",
        addr = in(reg) addr,
        options(preserves_flags, nostack)
    );
}

pub(crate) fn get_base() -> u64 {
    let mut addr;
    unsafe {
        core::arch::asm!(
            "move {addr}, $r21",
            addr = out(reg) addr,
            options(preserves_flags, nostack)
        );
    }
    addr
}
