pub mod device;
pub mod qemu;
pub mod serial;

#[cfg(feature = "cvm_guest")]
pub(crate) fn init_cvm_guest() {
    // Unimplemented, no-op
}

pub(crate) fn enable_cpu_features() {
    // enable float point
    loongArch64::register::euen::set_fpe(true);
}
