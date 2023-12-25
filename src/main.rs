#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;

use crate::hal::{pac, prelude::*};
use stm32f4xx_hal as hal;

use playroom_rgbctl::console::Console;
use playroom_rgbctl::consoleio::ConsoleIO;
use playroom_rgbctl::context::Context;
use playroom_rgbctl::imu::IMU;
use playroom_rgbctl::rgbcontroller::*;
use playroom_rgbctl::rotary_encoder::*;
use playroom_rgbctl::modes::manager::ModeManager;

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
    RGBController::init(gpioa.pa6, gpioa.pa7, gpiob.pb0, dp.TIM3, &clocks);

    // Configure USART1
    let io = ConsoleIO::init(
        gpioa.pa9,  // TX pin
        gpioa.pa10, // RX pin
        dp.USART1, &clocks,
    );

    // Configure debug LED
    let debug_led = gpioc.pc13.into_push_pull_output();

    // Configure I2C1
    IMU::init(gpiob.pb6, gpiob.pb7, dp.I2C1, &clocks);
    let mut ctx = Context {
        counter,
        debug_led,
        io,
    };

    let mut console = Console::init(&mut ctx);

    // Setup RGB rotary encoders
    cortex_m::interrupt::free(|cs| {
        RENCODER.borrow(cs).replace(Some(Encoder::init(
            gpioa.pa0.into_pull_down_input(),
            gpioa.pa1.into_pull_down_input(),
            Color::Red,
        )));
        GENCODER.borrow(cs).replace(Some(Encoder::init(
            gpioa.pa2.into_pull_down_input(),
            gpioa.pa3.into_pull_down_input(),
            Color::Green,
        )));
        BENCODER.borrow(cs).replace(Some(Encoder::init(
            gpioa.pa4.into_pull_down_input(),
            gpioa.pa5.into_pull_down_input(),
            Color::Blue,
        )));
    });

    let mut mode_manager = ModeManager::new();
    mode_manager.next();
    loop {
        mode_manager.process();
        // Process console IO
        console.process(&mut ctx);
    }
}
