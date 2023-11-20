use stm32f4xx_hal::gpio::*;

use crate::{
    consoleio::ConsoleIO,
    imu::IMU,
};

pub struct Context {
    pub debug_led: Pin<'C', 13, Output>,
    pub io: ConsoleIO,
    pub imu: IMU,
}
