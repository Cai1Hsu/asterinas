// SPDX-License-Identifier: MPL-2.0

//! The Loongson LS7A RTC device.

use spin::Once;

use crate::{
    arch::{
        boot::DEVICE_TREE,
        device::mmio_port::{MmioPort, ReadOnlyAccess, ReadWriteAccess, WriteOnlyAccess},
    },
    mm::Paddr,
};

pub struct Ls7aRtc {
    pub toy_trim: MmioPort<u32, ReadWriteAccess>,
    pub toy_writel: MmioPort<u32, WriteOnlyAccess>,
    pub toy_writeh: MmioPort<u32, WriteOnlyAccess>,
    pub toy_readl: MmioPort<u32, ReadOnlyAccess>,
    pub toy_readh: MmioPort<u32, ReadOnlyAccess>,
    pub toy_match0: MmioPort<u32, ReadWriteAccess>,
    pub toy_match1: MmioPort<u32, ReadWriteAccess>,
    pub toy_match2: MmioPort<u32, ReadWriteAccess>,
    pub rtc_ctrl: MmioPort<u32, ReadWriteAccess>,
    pub rtc_trim: MmioPort<u32, ReadWriteAccess>,
    pub rtc_write: MmioPort<u32, WriteOnlyAccess>,
    pub rtc_read: MmioPort<u32, ReadOnlyAccess>,
    pub rtc_match0: MmioPort<u32, ReadWriteAccess>,
    pub rtc_match1: MmioPort<u32, ReadWriteAccess>,
    pub rtc_match2: MmioPort<u32, ReadWriteAccess>,
}

impl Ls7aRtc {
    pub const unsafe fn new(base_paddr: Paddr) -> Self {
        // defined in <https://github.com/qemu/qemu/blob/b876e721f1c939f3e83ac85bd3c1c2821e12b3fa/hw/rtc/ls7a_rtc.c#L21-L35>
        Self {
            toy_trim: MmioPort::new(base_paddr + 0x20),
            toy_writel: MmioPort::new(base_paddr + 0x24),
            toy_writeh: MmioPort::new(base_paddr + 0x28),
            toy_readl: MmioPort::new(base_paddr + 0x2c),
            toy_readh: MmioPort::new(base_paddr + 0x30),
            toy_match0: MmioPort::new(base_paddr + 0x34),
            toy_match1: MmioPort::new(base_paddr + 0x38),
            toy_match2: MmioPort::new(base_paddr + 0x3c),
            rtc_ctrl: MmioPort::new(base_paddr + 0x40),
            rtc_trim: MmioPort::new(base_paddr + 0x60),
            rtc_write: MmioPort::new(base_paddr + 0x64),
            rtc_read: MmioPort::new(base_paddr + 0x68),
            rtc_match0: MmioPort::new(base_paddr + 0x6c),
            rtc_match1: MmioPort::new(base_paddr + 0x70),
            rtc_match2: MmioPort::new(base_paddr + 0x74),
        }
    }

    pub fn init(&mut self, enable_toy: bool, enable_rtc: bool) {
        const EO_ENABLE: u32 = 1 << 8;
        const TOY_ENABLE: u32 = 1 << 11;
        const RTC_ENABLE: u32 = 1 << 13;

        let mut ctrl = EO_ENABLE;

        if enable_toy {
            ctrl |= TOY_ENABLE;
        }

        if enable_rtc {
            ctrl |= RTC_ENABLE;
        }

        self.rtc_ctrl.write(ctrl);
    }
}

/// The LS7A RTC device.
pub static LS7A_RTC: Once<Ls7aRtc> = Once::new();

pub(crate) fn init() {
    let chosen = DEVICE_TREE.get().unwrap().find_node("/rtc").unwrap();
    if let Some(compatible) = chosen.compatible()
        && compatible.all().any(|c| c == "loongson,ls7a-rtc")
    {
        let base_paddr = chosen.reg().unwrap().next().unwrap().starting_address as usize;

        let mut ls7a_rtc = unsafe { Ls7aRtc::new(base_paddr) };

        ls7a_rtc.init(true, true);

        LS7A_RTC.call_once(|| ls7a_rtc);
    }
}
