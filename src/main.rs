#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_probe as _;

use crate::hal::{pac, prelude::*};
use stm32f4xx_hal as hal;

use playroom_rgbctl::console::Console;
use playroom_rgbctl::consoleio::ConsoleIO;
use playroom_rgbctl::context::Context;
use playroom_rgbctl::imu::IMU;
use playroom_rgbctl::rgbcontroller::RGBController;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.MHz()).freeze();

    // Enable gpio ports
    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpioc = dp.GPIOC.split();

    // Create delay timer
    let counter = dp.TIM2.counter_ms(&clocks);

    // Setup PWM generator pins
    let rgb = RGBController::init(gpioa.pa6, gpioa.pa7, gpiob.pb0, dp.TIM3, &clocks);

    // Configure USART1
    let io = ConsoleIO::init(
        gpioa.pa9,  // TX pin
        gpioa.pa10, // RX pin
        dp.USART1, &clocks,
    );

    // Configure debug LED
    let debug_led = gpioc.pc13.into_push_pull_output();

    // Configure I2C1
    let imu = IMU::init(gpiob.pb6, gpiob.pb7, dp.I2C1, &clocks);
    let mut ctx = Context {
        counter,
        debug_led,
        io,
        imu,
        rgb,
    };

    let mut console = Console::init(&mut ctx);

    loop {
        console.process(&mut ctx);
    }
}
