pub mod cpu;
pub mod device;
pub mod iommu;
pub(crate) mod irq;
pub(crate) mod mm;
pub(crate) mod pci;
pub mod qemu;
pub mod serial;
pub mod task;
pub mod timer;

use core::sync::atomic::Ordering;

#[cfg(feature = "cvm_guest")]
pub(crate) fn init_cvm_guest() {
    // Unimplemented, no-op
}

/// Return the frequency of TSC. The unit is Hz.
pub fn tsc_freq() -> u64 {
    timer::TIMEBASE_FREQ.load(Ordering::Relaxed)
}

/// Reads the current value of the processor’s time-stamp counter (TSC).
pub fn read_tsc() -> u64 {
    let mut pmc: usize;
    unsafe {
        core::arch::asm!(
            "rdtime.d {}, $zero",
            out(reg) pmc,
        );
    }
    pmc as u64
}

pub(crate) fn enable_cpu_features() {
    // enable float point
    loongArch64::register::euen::set_fpe(true);
}
