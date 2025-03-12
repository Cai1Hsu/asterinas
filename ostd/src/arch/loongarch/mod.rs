pub mod cpu;
pub mod device;
pub(crate) mod irq;
pub mod qemu;
pub mod serial;
pub mod timer;

#[cfg(feature = "cvm_guest")]
pub(crate) fn init_cvm_guest() {
    // Unimplemented, no-op
}

pub(crate) fn enable_cpu_features() {
    // enable float point
    loongArch64::register::euen::set_fpe(true);
}
