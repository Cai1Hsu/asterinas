// SPDX-License-Identifier: MPL-2.0

//! The architecture support of context switch.

core::arch::global_asm!(include_str!("switch.S"));

use crate::task::TaskContextApi;

#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub(crate) struct TaskContext {
    pub regs: CalleeRegs,
    pub pc: usize,
    pub fsbase: usize,
}

/// Callee-saved registers.
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct CalleeRegs {
    /// sp
    pub sp: u64,
    /// fp
    pub fp: u64,
    /// s0
    pub s0: u64,
    /// s1
    pub s1: u64,
    /// s2
    pub s2: u64,
    /// s3
    pub s3: u64,
    /// s4
    pub s4: u64,
    /// s5
    pub s5: u64,
    /// s6
    pub s6: u64,
    /// s7
    pub s7: u64,
    /// s8
    pub s8: u64,
}

impl CalleeRegs {
    /// Creates new `CalleeRegs`
    pub const fn new() -> Self {
        Self {
            sp: 0,
            fp: 0,
            s0: 0,
            s1: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
        }
    }
}

impl TaskContext {
    pub const fn new() -> Self {
        TaskContext {
            regs: CalleeRegs::new(),
            pc: 0,
            fsbase: 0,
        }
    }

    /// Sets thread-local storage pointer.
    pub fn set_tls_pointer(&mut self, tls: usize) {
        self.fsbase = tls;
    }

    /// Gets thread-local storage pointer.
    pub fn tls_pointer(&self) -> usize {
        self.fsbase
    }
}

impl TaskContextApi for TaskContext {
    fn set_instruction_pointer(&mut self, ip: usize) {
        self.pc = ip;
    }

    fn instruction_pointer(&self) -> usize {
        self.pc
    }

    fn set_stack_pointer(&mut self, sp: usize) {
        self.regs.sp = sp as u64;
    }

    fn stack_pointer(&self) -> usize {
        self.regs.sp as usize
    }
}

extern "C" {
    pub(crate) fn context_switch(cur: *mut TaskContext, nxt: *const TaskContext);
}
