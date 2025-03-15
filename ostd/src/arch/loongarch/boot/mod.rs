// SPDX-License-Identifier: MPL-2.0

//! The LoongArch boot module defines the entrypoints of Asterinas.

pub mod smp;

use core::arch::global_asm;

use fdt::Fdt;
use spin::Once;

use crate::{
    boot::{
        memory_region::{MemoryRegion, MemoryRegionArray, MemoryRegionType},
        BootloaderAcpiArg, BootloaderFramebufferArg,
    }, console::early_print, early_println, mm::paddr_to_vaddr
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

        // The dtb qemu generated is(with 2G memory specified):
        // memory@80000000 {
        //     device_type = "memory";
        //     reg = <0x02 0x80000000 0x02 0x70000000>;
        // };
        // memory@0 {
        //     device_type = "memory";
        //     reg = <0x02 0x00 0x02 0x10000000>;
        // };

        // But the FDT interprets it as:
        // memory@80000000 {
        //     device_type = "memory"
        //     reg = <0x280000000 0x2f0000000>
        // };

        // memory@0 {
        //     device_type = "memory"
        //     reg = <0x200000000 0x210000000>
        // };
        // I don't know what does the 0x02 come from, but ignoring it provides the correct result.

        if region_size > 0 {
            regions.push(MemoryRegion::new(
                region_address,
                region_size,
                MemoryRegionType::Usable,
            ));
        }
    }

    // TODO: add reserved region.

    // add the kernel region.
    regions.push(MemoryRegion::kernel());

    // TODO: add initramfs region.

    regions.into_non_overlapping()
}

#[no_mangle]
pub extern "C" fn loongarch_boot(_cpu_id: usize, device_tree_paddr: usize) -> ! {
    // TODO
    let device_tree_ptr = paddr_to_vaddr(device_tree_paddr) as *const u8;
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
