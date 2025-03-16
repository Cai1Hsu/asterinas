use core::{fmt::Debug, ops::Range};

use ostd_pod::Pod;

use crate::{
    mm::{
        page_table::PageTableEntryTrait, CachePolicy, Paddr, PageFlags, PageProperty,
        PagingConstsTrait, PagingLevel, PodOnce, PrivilegedPageFlags as PrivFlags, Vaddr,
        PAGE_SIZE,
    },
    util::SameSizeAs,
};

pub(crate) const NR_ENTRIES_PER_PAGE: usize = 512;

#[derive(Clone, Debug, Default)]
pub struct PagingConsts {}

impl PagingConstsTrait for PagingConsts {
    const BASE_PAGE_SIZE: usize = 4096;
    const NR_LEVELS: PagingLevel = 4;
    const ADDRESS_WIDTH: usize = 48;
    const HIGHEST_TRANSLATION_LEVEL: PagingLevel = 4;
    const PTE_SIZE: usize = core::mem::size_of::<u64>();
}

bitflags::bitflags! {
    #[derive(Pod)]
    #[repr(C)]
    /// Possible flags for a page table entry.
    pub struct PageTableFlags: usize {
        const VALID = 1 << 0;
        const DIRTY = 1 << 1;
        // Refer to loongarch reference manual.
        // 0b00 for PLV0, 0b01 for PLV1, 0b10 for PLV2, 0b11 for PLV3
        const PLV_LOW = 1 << 2;
        const PLV_HIGH = 1 << 3;
        // Refer to loongarch reference manual.
        // 0b00 for strongly ordered uncached, 0b01 for coherent cached, 0b10 for weakly ordered uncached
        const MAT_LOW = 1 << 4;
        const MAT_HIGH = 1 << 5;
        // this bit is global for basic pages and huge for huge pages
        const GLOBAL_OR_HUGE = 1 << 6;
        const PHYSICAL = 1 << 7;
        const WRITABLE = 1 << 8;
        // this bit only applies to huge pages, for basic pages, use GLOBAL_HUGE flag
        const GLOBAL = 1 << 12;
        const NO_READ = 1 << 61;
        const NO_EXECUTE = 1 << 62;
        const RPLV = 1 << 63;
    }
}

/// Parse a bit-flag bits `val` in the representation of `from` to `to` in bits.
macro_rules! parse_flags {
    ($val:expr, $from:expr, $to:expr) => {
        ($val as usize & $from.bits() as usize) >> $from.bits().ilog2() << $to.bits().ilog2()
    };
}

pub(crate) fn tlb_flush_addr(vaddr: Vaddr) {
    unsafe {
        core::arch::asm!(
            "
                dbar 0
                // TODO: 0x5 requires G = 0, ensure this matches expectation
                invtlb 0x5, $zero, {vaddr}
            ",
            vaddr = in(reg) vaddr,
        );
    }
}

pub(crate) fn tlb_flush_addr_range(range: &Range<Vaddr>) {
    for vaddr in range.clone().step_by(PAGE_SIZE) {
        tlb_flush_addr(vaddr);
    }
}

pub(crate) fn tlb_flush_all_excluding_global() {
    unsafe {
        core::arch::asm!(
            "
                dbar 0
                // 0x3: invalidate all entries with G = 0
                invtlb 0x3, $zero, $zero
            "
        );
    }
}

pub(crate) fn tlb_flush_all_including_global() {
    unsafe {
        core::arch::asm!(
            "
                dbar 0
                // 0x0: invalidate all entries
                invtlb 0x0, $zero, $zero
            "
        );
    }
}

#[derive(Clone, Copy, Pod, Default)]
#[repr(C)]
pub struct PageTableEntry(usize);

// SAFETY: `PageTableEntry` has the same size as `usize`
unsafe impl SameSizeAs<usize> for PageTableEntry {}

impl PodOnce for PageTableEntry {}

impl PageTableEntry {
    const PHYS_ADDR_MASK: usize = 0x0000_ffff_ffff_f000;

    fn new_paddr(paddr: Paddr) -> Self {
        Self(paddr & Self::PHYS_ADDR_MASK)
    }
}

impl PageTableEntryTrait for PageTableEntry {
    fn is_present(&self) -> bool {
        let paddr = self.paddr();
        let flags = self.0 & (!Self::PHYS_ADDR_MASK);

        paddr != 0 && (flags == 0 || (flags & PageTableFlags::VALID.bits() != 0))
    }

    fn new_page(
        paddr: crate::prelude::Paddr,
        level: PagingLevel,
        prop: crate::mm::PageProperty,
    ) -> Self {
        let mut pte = Self::new_paddr(paddr);

        if level > 1 {
            pte = Self(pte.0 | PageTableFlags::GLOBAL_OR_HUGE.bits());
        }

        pte = Self(pte.0 | PageTableFlags::VALID.bits());

        pte.set_prop(prop);
        pte
    }

    fn new_pt(paddr: crate::prelude::Paddr) -> Self {
        Self::new_paddr(paddr)
    }

    fn paddr(&self) -> crate::prelude::Paddr {
        self.0 & Self::PHYS_ADDR_MASK
    }

    fn prop(&self) -> PageProperty {
        let flags = (parse_flags!(!self.0, PageTableFlags::NO_READ, PageFlags::R))
            | (parse_flags!(self.0, PageTableFlags::WRITABLE, PageFlags::W))
            | (parse_flags!(!self.0, PageTableFlags::NO_EXECUTE, PageFlags::X))
            | (parse_flags!(self.0, PageTableFlags::DIRTY, PageFlags::DIRTY))
            | PageFlags::ACCESSED.bits() as usize;

        let priv_flags = (
            // TODO: this only allows PLV3, the most common plv for user mode
            (parse_flags!(self.0, PageTableFlags::PLV_LOW, PrivFlags::USER))
                & (parse_flags!(self.0, PageTableFlags::PLV_HIGH, PrivFlags::USER))
        ) | (parse_flags!(self.0, PageTableFlags::GLOBAL, PrivFlags::GLOBAL))
            // TODO: this only appiles to basic pages
            | (parse_flags!(self.0, PageTableFlags::GLOBAL_OR_HUGE, PrivFlags::GLOBAL));

        let cache = if self.0 & PageTableFlags::MAT_LOW.bits() != 0 {
            // coherent cached
            CachePolicy::Writeback
        } else if self.0 & PageTableFlags::MAT_HIGH.bits() != 0 {
            // weakly ordered uncached
            CachePolicy::WriteCombining
        } else {
            // strongly ordered uncached
            CachePolicy::Uncacheable
        };

        PageProperty {
            flags: PageFlags::from_bits(flags as u8).unwrap(),
            cache,
            priv_flags: PrivFlags::from_bits(priv_flags as u8).unwrap(),
        }
    }

    fn set_prop(&mut self, prop: PageProperty) {
        // skips if the entry that points to a sub page table
        if !self.is_present() || self.0 & (!Self::PHYS_ADDR_MASK) == 0 {
            // According to the interface of `PageTableEntryTrait`,
            // setting the property of a non-present entry is a no-op.
            return;
        }

        let flags = PageTableFlags::VALID.bits()
            | PageTableFlags::PHYSICAL.bits()
            | PageTableFlags::DIRTY.bits()
            | parse_flags!(!prop.flags.bits(), PageFlags::R, PageTableFlags::NO_READ)
            | parse_flags!(prop.flags.bits(), PageFlags::W, PageTableFlags::WRITABLE)
            | parse_flags!(!prop.flags.bits(), PageFlags::X, PageTableFlags::NO_EXECUTE)
            | parse_flags!(
                prop.priv_flags.bits(),
                PrivFlags::USER,
                PageTableFlags::PLV_LOW
            )
            | parse_flags!(
                prop.priv_flags.bits(),
                PrivFlags::USER,
                PageTableFlags::PLV_HIGH
            );
        // TODO: handle global flag

        self.0 = self.0 & Self::PHYS_ADDR_MASK | flags;
    }

    fn is_last(&self, level: PagingLevel) -> bool {
        level == 1 || self.0 & PageTableFlags::GLOBAL_OR_HUGE.bits() != 0
    }
}

impl Debug for PageTableEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut f = f.debug_struct("PageTableEntry");
        f.field("raw", &format_args!("{:#x}", self.0))
            .field("paddr", &format_args!("{:#x}", self.paddr()))
            .field("present", &self.is_present())
            .field("flags", &PageTableFlags::from_bits_truncate(self.0))
            .field("prop", &self.prop())
            .finish()
    }
}

/// Activate the given level 4 page table.
///
/// # Safety
///
/// Changing the level 4 page table is unsafe, because it's possible to violate memory safety by
/// changing the page mapping.
pub unsafe fn activate_page_table(root_paddr: Paddr, _root_pt_cache: CachePolicy) {
    assert!(root_paddr % PagingConsts::BASE_PAGE_SIZE == 0);

    loongArch64::register::pgdh::set_base(root_paddr);
}

pub fn current_page_table_paddr() -> Paddr {
    // TODO: we return pgdh for now because the boot page table initialization
    // uses this function. We should change this once we have a better way to
    // get the boot page table.
    // Same as above for activate_page_table.
    loongArch64::register::pgdh::read().raw()
}

pub(crate) fn __memcpy_fallible(dst: *mut u8, src: *const u8, size: usize) -> usize {
    // TODO: implement fallible
    unsafe { core::ptr::copy(src, dst, size) };
    0
}

pub(crate) fn __memset_fallible(dst: *mut u8, value: u8, size: usize) -> usize {
    // TODO: implement fallible
    unsafe { core::ptr::write_bytes(dst, value, size) };
    0
}
