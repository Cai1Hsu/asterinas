// SPDX-License-Identifier: MPL-2.0

//! CPU

pub mod local;

use core::fmt::Debug;

use loongArch64::register::estat::{Exception, Trap};

pub use super::trap::GeneralRegs as RawGeneralRegs;
use super::trap::{TrapFrame, UserContext as RawUserContext};
use crate::user::{ReturnReason, UserContextApi, UserContextApiInternal};

/// The FPU state of user task.
///
/// This could be used for saving both legacy and modern state format.
#[derive(Debug, Clone, Copy)]
pub struct FpuState {}

impl FpuState {
    /// Initializes a new instance.
    pub fn init() -> Self {
        Self {}
    }

    /// Returns whether the instance can contains valid state.
    pub fn is_valid(&self) -> bool {
        // TODO
        true
    }

    /// Save CPU's current FPU state into this instance.
    pub fn save(&self) {
        // TODO
    }

    /// Restores CPU's FPU state from this instance.
    pub fn restore(&self) {
        // TODO
    }

    pub fn clear(&self) {
        // TODO
    }
}

/// Cpu context, including both general-purpose registers and FPU state.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct UserContext {
    user_context: RawUserContext,
    trap: Trap,
    fpu_state: FpuState, // TODO
    cpu_exception_info: CpuExceptionInfo,
}

/// CPU exception information.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct CpuExceptionInfo {
    /// The type of the exception.
    pub code: Exception,
    /// The error code associated with the exception.
    pub page_fault_addr: usize,
    pub error_code: usize, // TODO
}

impl Default for UserContext {
    fn default() -> Self {
        UserContext {
            user_context: RawUserContext::default(),
            trap: Trap::Exception(Exception::Breakpoint),
            fpu_state: FpuState {},
            cpu_exception_info: CpuExceptionInfo::default(),
        }
    }
}

impl Default for CpuExceptionInfo {
    fn default() -> Self {
        CpuExceptionInfo {
            code: Exception::Breakpoint,
            page_fault_addr: 0,
            error_code: 0,
        }
    }
}

impl CpuExceptionInfo {
    /// Get corresponding CPU exception
    pub fn cpu_exception(&self) -> CpuException {
        self.code
    }
}

impl UserContext {
    /// Returns a reference to the general registers.
    pub fn general_regs(&self) -> &RawGeneralRegs {
        &self.user_context.general
    }

    /// Returns a mutable reference to the general registers
    pub fn general_regs_mut(&mut self) -> &mut RawGeneralRegs {
        &mut self.user_context.general
    }

    /// Returns the trap information.
    pub fn trap_information(&self) -> &CpuExceptionInfo {
        &self.cpu_exception_info
    }

    /// Returns a reference to the FPU state.
    pub fn fpu_state(&self) -> &FpuState {
        &self.fpu_state
    }

    /// Returns a mutable reference to the FPU state.
    pub fn fpu_state_mut(&mut self) -> &mut FpuState {
        &mut self.fpu_state
    }

    /// Sets thread-local storage pointer.
    pub fn set_tls_pointer(&mut self, tls: usize) {
        self.set_tp(tls)
    }

    /// Gets thread-local storage pointer.
    pub fn tls_pointer(&self) -> usize {
        self.tp()
    }

    /// Activates thread-local storage pointer on the current CPU.
    pub fn activate_tls_pointer(&self) {
        // No-op
    }
}

impl UserContextApiInternal for UserContext {
    fn execute<F>(&mut self, mut has_kernel_event: F) -> ReturnReason
    where
        F: FnMut() -> bool,
    {
        let ret = loop {
            self.user_context.run();
            match loongArch64::register::estat::read().cause() {
                Trap::Interrupt(_) => todo!(),
                Trap::Exception(Exception::Syscall) => {
                    self.user_context.era += 4;
                    break ReturnReason::UserSyscall;
                }
                Trap::Exception(e) => {
                    let badv = self.user_context.badv;
                    log::trace!("Exception, cause: {e:?}, badv: {badv:#x?}");
                    self.cpu_exception_info = CpuExceptionInfo {
                        code: e,
                        page_fault_addr: badv,
                        error_code: 0,
                    };
                    break ReturnReason::UserException;
                }
                Trap::MachineError(machine_error) => todo!(),
                Trap::Unknown => todo!(),
            }

            if has_kernel_event() {
                break ReturnReason::KernelEvent;
            }
        };

        crate::arch::irq::enable_local();
        ret
    }

    fn as_trap_frame(&self) -> TrapFrame {
        TrapFrame {
            general: self.user_context.general,
            prmd: self.user_context.prmd,
            era: self.user_context.era,
            badv: self.user_context.badv,
        }
    }
}

impl UserContextApi for UserContext {
    fn trap_number(&self) -> usize {
        todo!()
    }

    fn trap_error_code(&self) -> usize {
        todo!()
    }

    fn instruction_pointer(&self) -> usize {
        self.user_context.era
    }

    fn set_instruction_pointer(&mut self, ip: usize) {
        self.user_context.set_ip(ip);
    }

    fn stack_pointer(&self) -> usize {
        self.user_context.get_sp()
    }

    fn set_stack_pointer(&mut self, sp: usize) {
        self.user_context.set_sp(sp);
    }
}

macro_rules! cpu_context_impl_getter_setter {
    ( $( [ $field: ident, $setter_name: ident] ),*) => {
        impl UserContext {
            $(
                #[doc = concat!("Gets the value of ", stringify!($field))]
                #[inline(always)]
                pub fn $field(&self) -> usize {
                    self.user_context.general.$field
                }

                #[doc = concat!("Sets the value of ", stringify!($field))]
                #[inline(always)]
                pub fn $setter_name(&mut self, $field: usize) {
                    self.user_context.general.$field = $field;
                }
            )*
        }
    };
}

cpu_context_impl_getter_setter!(
    [ra, set_ra],
    [tp, set_tp],
    [sp, set_sp],
    [a0, set_a0],
    [a1, set_a1],
    [a2, set_a2],
    [a3, set_a3],
    [a4, set_a4],
    [a5, set_a5],
    [a6, set_a6],
    [a7, set_a7],
    [t0, set_t0],
    [t1, set_t1],
    [t2, set_t2],
    [t3, set_t3],
    [t4, set_t4],
    [t5, set_t5],
    [t6, set_t6],
    [t7, set_t7],
    [t8, set_t8],
    [r21, set_r21],
    [fp, set_fp],
    [s0, set_s0],
    [s1, set_s1],
    [s2, set_s2],
    [s3, set_s3],
    [s4, set_s4],
    [s5, set_s5],
    [s6, set_s6],
    [s7, set_s7],
    [s8, set_s8]
);

/// CPU exception.
pub type CpuException = Exception;
