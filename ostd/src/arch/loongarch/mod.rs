pub mod device;
pub mod qemu;
pub mod serial;

pub(crate) fn enable_cpu_features() {
    // enable float point
    loongArch64::register::euen::set_fpe(true);
}
