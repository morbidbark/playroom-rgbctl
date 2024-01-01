use hal::serial::*;
use stm32f4xx_hal as hal;
use stm32f4xx_hal::{pac, prelude::*};

use core::fmt::Write;

pub struct ConsoleIO {
    tx: Tx<pac::USART1, u8>,
    rx: Rx<pac::USART1, u8>,
}
impl ConsoleIO {
    pub fn init(
        tx_pin: hal::gpio::Pin<'A', 9>,
        rx_pin: hal::gpio::Pin<'A', 10>,
        usart: pac::USART1,
        clocks: &hal::rcc::Clocks,
    ) -> Self {
        // 9600 Baud, 1 stopbit, no parity, 8-bit word length
        let config = hal::serial::Config::default().baudrate(9600.bps());
        let (tx, rx) = usart
            .serial((tx_pin, rx_pin), config, clocks)
            .unwrap()
            .split();

        Self { tx, rx }
    }

    pub fn receive(&mut self, buf: &mut [u8]) -> usize {
        let mut received = 0;
        while self.rx.is_rx_not_empty() && received < buf.len() {
            buf[received] = match self.rx.read() {
                Ok(byte) => byte,
                Err(_) => continue,
            };
            received += 1;
        }
        received
    }

    pub fn write(&mut self, buf: &str) {
        write!(self.tx, "{}", buf).unwrap();
    }
}
