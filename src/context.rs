use stm32f4xx_hal::gpio::*;

use crate::consoleio::ConsoleIO;

pub struct Context {
    pub debug_led: Pin<'C', 13, Output>,
    pub io: ConsoleIO,
}
