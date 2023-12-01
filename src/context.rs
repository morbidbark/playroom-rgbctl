use stm32f4xx_hal::{
    pac::TIM2,
    timer::counter::CounterMs,
    gpio::*,
};

use crate::{
    consoleio::ConsoleIO,
    imu::IMU,
};

pub struct Context {
    pub counter: CounterMs<TIM2>,
    pub debug_led: Pin<'C', 13, Output>,
    pub io: ConsoleIO,
    pub imu: IMU,
}
