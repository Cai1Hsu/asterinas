// SPDX-License-Identifier: MPL-2.0

//! A memory-mapped UART. Copied from uart_16550.

use super::mmio_port::{MmioPort, ReadWriteAccess, WriteOnlyAccess};

/// A serial port.
///
/// Serial ports are a legacy communications port common on IBM-PC compatible computers.
/// Ref: <https://wiki.osdev.org/Serial_Ports>
pub struct SerialPort {
    /// Data Register
    data: MmioPort<u8, ReadWriteAccess>,
    #[expect(dead_code)]
    /// Interrupt Enable Register
    int_en: MmioPort<u8, WriteOnlyAccess>,
    #[expect(dead_code)]
    /// First In First Out Control Register
    fifo_ctrl: MmioPort<u8, WriteOnlyAccess>,
    #[expect(dead_code)]
    /// Line control Register
    line_ctrl: MmioPort<u8, WriteOnlyAccess>,
    #[expect(dead_code)]
    /// Modem Control Register
    modem_ctrl: MmioPort<u8, WriteOnlyAccess>,
    /// Line status Register
    line_status: MmioPort<u8, ReadWriteAccess>,
    #[expect(dead_code)]
    /// Modem Status Register
    modem_status: MmioPort<u8, ReadWriteAccess>,
}

impl SerialPort {
    /// Creates a serial port.
    ///
    /// # Safety
    ///
    /// User must ensure the `port` is valid serial port.
    pub const unsafe fn new(uart_base: usize) -> Self {
        let data = MmioPort::new(uart_base);
        let int_en = MmioPort::new(uart_base + 1);
        let fifo_ctrl = MmioPort::new(uart_base + 2);
        let line_ctrl = MmioPort::new(uart_base + 3);
        let modem_ctrl = MmioPort::new(uart_base + 4);
        let line_status = MmioPort::new(uart_base + 5);
        let modem_status = MmioPort::new(uart_base + 6);

        Self {
            data,
            int_en,
            fifo_ctrl,
            line_ctrl,
            modem_ctrl,
            line_status,
            modem_status,
        }
    }

    /// Initializes the serial port.
    pub fn init(&self) {
        // TODO
    }

    /// Sends data to the data port
    pub fn send(&self, data: u8) {
        const TX_IDLE: u8 = 1u8 << 5;

        while self.line_status() & TX_IDLE == 0 {}
        self.data.write(data);
    }

    /// Receives data from the data port
    #[inline]
    pub fn recv(&self) -> u8 {
        const RX_READY: u8 = 1u8 << 0;
        while self.line_status() & RX_READY == 0 {}
        self.data.read()
    }

    /// Gets line status
    #[inline]
    pub fn line_status(&self) -> u8 {
        self.line_status.read()
    }
}
