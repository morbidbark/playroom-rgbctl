use stm32f4xx_hal::{gpio::*, pac::TIM2, timer::counter::CounterMs};

use crate::consoleio::ConsoleIO;

pub struct Context {
    pub counter: CounterMs<TIM2>,
    pub debug_led: Pin<'C', 13, Output>,
    pub io: ConsoleIO,
}
