use super::mem_port::{MemPort, ReadWriteAccess, WriteOnlyAccess};

/// A serial port.
///
/// Serial ports are a legacy communications port common on IBM-PC compatible computers.
/// Ref: <https://wiki.osdev.org/Serial_Ports>
pub struct SerialPort {
    /// Data Register
    data: MemPort<u8, ReadWriteAccess>,
    /// Interrupt Enable Register
    int_en: MemPort<u8, WriteOnlyAccess>,
    /// First In First Out Control Register
    fifo_ctrl: MemPort<u8, WriteOnlyAccess>,
    /// Line control Register
    line_ctrl: MemPort<u8, WriteOnlyAccess>,
    /// Modem Control Register
    modem_ctrl: MemPort<u8, WriteOnlyAccess>,
    /// Line status Register
    line_status: MemPort<u8, ReadWriteAccess>,
    /// Modem Status Register
    modem_status: MemPort<u8, ReadWriteAccess>,
}

impl SerialPort {
    pub const unsafe fn new(uart_base: usize) -> Self {
        let data = MemPort::new(uart_base);
        let int_en = MemPort::new(uart_base + 1);
        let fifo_ctrl = MemPort::new(uart_base + 2);
        let line_ctrl = MemPort::new(uart_base + 3);
        let modem_ctrl = MemPort::new(uart_base + 4);
        let line_status = MemPort::new(uart_base + 5);
        let modem_status = MemPort::new(uart_base + 6);

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
        // TODO: this works for now, so not adding actual initialization
    }

    pub fn send(&self, data: u8) {
        const TX_IDEL: u8 = 1u8 << 5;

        while self.line_status() & TX_IDEL == 0 {}
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
