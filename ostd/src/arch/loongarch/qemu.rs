// SPDX-License-Identifier: MPL-2.0

//! Providing the ability to exit QEMU and return a value as debug result.

use crate::arch::device::mem_port::{MemPort, WriteOnlyAccess};

/// The exit code of QEMU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QemuExitCode {
    /// The code that indicates a successful exit.
    Success,
    /// The code that indicates a failed exit.
    Failed,
}

/// Exit QEMU with the given exit code.
pub fn exit_qemu(_exit_code: QemuExitCode) -> ! {
    let port = unsafe { MemPort::<u8, WriteOnlyAccess>::new(0x100e001c) };

    port.write(0x34);

    unreachable!("qemu did not exit");
}
