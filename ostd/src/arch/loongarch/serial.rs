// SPDX-License-Identifier: MPL-2.0

//! The console I/O.

use alloc::fmt;
use core::fmt::Write;

use super::device::serial::SerialPort;

// TODO: probe the serial port by walking fdt
static UART_PORT: SerialPort = unsafe { SerialPort::new(0x1fe001e0) };

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
        for &c in s.as_bytes() {
            send(c);
        }
        Ok(())
    }
}

/// Initializes the serial port.
pub(crate) fn init() {
    UART_PORT.init();
}

/// Sends a byte on the serial port.
pub fn send(data: u8) {
    UART_PORT.send(data);
}
