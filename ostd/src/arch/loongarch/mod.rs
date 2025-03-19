// SPDX-License-Identifier: MPL-2.0

//! Platform-specific code for the LoongArch platform.

pub mod boot;
pub mod device;
pub mod iommu;
pub(crate) mod irq;
pub(crate) mod mm;
pub mod qemu;
pub mod serial;
pub mod task;
pub mod timer;
