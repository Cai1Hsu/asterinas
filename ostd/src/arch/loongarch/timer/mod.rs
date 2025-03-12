// SPDX-License-Identifier: MPL-2.0

//! The timer support.

use core::sync::atomic::{AtomicU64, Ordering};

/// The timer frequency (Hz). Here we choose 1000Hz since 1000Hz is easier for unit conversion and
/// convenient for timer. What's more, the frequency cannot be set too high or too low, 1000Hz is
/// a modest choice.
///
/// For system performance reasons, this rate cannot be set too high, otherwise most of the time
/// is spent executing timer code.
pub const TIMER_FREQ: u64 = 1000;

pub(crate) static TIMEBASE_FREQ: AtomicU64 = AtomicU64::new(1);

pub(super) fn init() {
    use loongArch64::cpu::CPUCFG;

    let cc_freq = CPUCFG::read(0x4).get_bits(0, 31) as u64;
    let cc_mul = CPUCFG::read(0x5).get_bits(0, 15) as u64;
    let cc_div = CPUCFG::read(0x5).get_bits(15, 31) as u64;

    let timer_freq = cc_freq * cc_mul / cc_div;

    TIMEBASE_FREQ.store(timer_freq, Ordering::Relaxed);

    // TODO: add abstraction for ls7a-rtc
}
