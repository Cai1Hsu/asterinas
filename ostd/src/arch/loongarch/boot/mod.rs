// SPDX-License-Identifier: MPL-2.0

//! The LoongArch boot module defines the entrypoints of Asterinas.

pub mod smp;

use core::arch::global_asm;

use fdt::Fdt;
use spin::Once;

use crate::boot::{
    memory_region::{MemoryRegion, MemoryRegionArray, MemoryRegionType},
    BootloaderAcpiArg, BootloaderFramebufferArg,
};

global_asm!(include_str!("boot.S"));

/// The Flattened Device Tree of the platform.
pub static DEVICE_TREE: Once<Fdt> = Once::new();

fn parse_bootloader_name() -> &'static str {
    "unknown"
}

fn parse_kernel_commandline() -> &'static str {
    DEVICE_TREE.get().unwrap().chosen().bootargs().unwrap_or("")
}

fn parse_initramfs() -> Option<&'static [u8]> {
    // TODO
    None
}

fn parse_acpi_arg() -> BootloaderAcpiArg {
    BootloaderAcpiArg::NotProvided
}

fn parse_framebuffer_info() -> Option<BootloaderFramebufferArg> {
    None
}

fn parse_memory_regions() -> MemoryRegionArray {
    let mut regions = MemoryRegionArray::new();

    // add memory region
    for region in DEVICE_TREE.get().unwrap().memory().regions() {
        let region_address = region.starting_address as usize;
        let region_size = region.size.unwrap_or(0);

        let region_address = region_address & 0xffff_ffff;
        let region_size = region_size & 0xffff_ffff;

        if region_size > 0 {
            regions
                .push(MemoryRegion::new(
                    region_address,
                    region_size,
                    MemoryRegionType::Usable,
                ))
                .unwrap();
        }
    }

    // TODO: add reserved region.

    // add the kernel region.
    regions.push(MemoryRegion::kernel()).unwrap();

    // TODO: add initramfs region.

    regions.into_non_overlapping()
}

/// The entry point of the Rust code portion of Asterinas.
#[no_mangle]
pub extern "C" fn loongarch_boot(_cpu_id: usize, device_tree_paddr: usize) -> ! {
    // We have to use DMW1 here as the linear mapping window only covers the physical memory for now
    let device_tree_ptr = (device_tree_paddr + 0x9000_0000_0000_0000) as *const u8;
    let fdt = unsafe { Fdt::from_ptr(device_tree_ptr).unwrap() };
    DEVICE_TREE.call_once(|| fdt);

    use crate::boot::{call_ostd_main, EarlyBootInfo, EARLY_INFO};

    EARLY_INFO.call_once(|| EarlyBootInfo {
        bootloader_name: parse_bootloader_name(),
        kernel_cmdline: parse_kernel_commandline(),
        initramfs: parse_initramfs(),
        acpi_arg: parse_acpi_arg(),
        framebuffer_arg: parse_framebuffer_info(),
        memory_regions: parse_memory_regions(),
    });

    call_ostd_main();
}
