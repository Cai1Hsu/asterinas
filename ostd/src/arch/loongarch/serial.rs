// SPDX-License-Identifier: MPL-2.0

//! The console I/O.

use alloc::fmt;
use core::fmt::Write;

use spin::Once;

use super::{boot::DEVICE_TREE, device::serial::SerialPort};

static UART_PORT: Once<SerialPort> = Once::new();

/// Prints the formatted arguments to the standard output using the serial port.
#[inline]
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// The callback function for console input.
pub type InputCallback = dyn Fn(u8) + Send + Sync + 'static;

/// Registers a callback function to be called when there is console input.
pub fn register_console_input_callback(_f: &'static InputCallback) {
    todo!()
}

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let port = UART_PORT.get().unwrap();
        for &c in s.as_bytes() {
            port.send(c);
        }
        Ok(())
    }
}

/// Initializes the serial port.
pub(crate) fn init() {
    let chosen = DEVICE_TREE.get().unwrap().find_node("/serial").unwrap();
    if let Some(compatible) = chosen.compatible()
        && compatible.all().any(|c| c == "ns16550a")
    {
        let base_paddr = chosen.reg().unwrap().next().unwrap().starting_address as usize;
        let uart_port = unsafe { SerialPort::new(base_paddr) };

        UART_PORT.call_once(|| uart_port);
    }
}

/// Sends a byte on the serial port.
pub fn send(data: u8) {
    UART_PORT.get().unwrap().send(data);
}
