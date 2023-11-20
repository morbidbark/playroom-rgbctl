#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;

use stm32f4xx_hal as hal;
use crate::hal::{pac, prelude::*};

use playroom_rgbctl::console::Console;
use playroom_rgbctl::consoleio::ConsoleIO;
use playroom_rgbctl::context::Context;
use playroom_rgbctl::imu::IMU;

#[entry]
fn main() -> ! {
    
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.MHz()).freeze();

    // Configure USART1
    let gpioa = dp.GPIOA.split();
    let io = ConsoleIO::init(
        gpioa.pa9, // TX pin
        gpioa.pa10, // RX pin
        dp.USART1,
        &clocks,
    );

    // Configure debug LED
    let gpioc = dp.GPIOC.split();
    let debug_led = gpioc.pc13.into_push_pull_output();

    // Configure I2C1
    let gpiob = dp.GPIOB.split();
    let imu = IMU::init(
        gpiob.pb6,
        gpiob.pb7,
        dp.I2C1,
        &clocks,
    );
    let mut ctx = Context { debug_led, io, imu };

    let mut console = Console::init(&mut ctx);

    loop {
        console.process(&mut ctx);
    }
}
