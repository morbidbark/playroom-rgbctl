#![no_std]
#![no_main]

use crate::hal::{
    gpio::{Edge, Input, PC15},
    pac::{self, interrupt},
    prelude::*,
    timer::counter::CounterMs,
};
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_probe as _;
use stm32f4xx_hal as hal;

use playroom_rgbctl::console::Console;
use playroom_rgbctl::consoleio::ConsoleIO;
use playroom_rgbctl::context::Context;
use playroom_rgbctl::imu::IMU;
use playroom_rgbctl::modes::manager::ModeManager;
use playroom_rgbctl::rgbcontroller::*;
use playroom_rgbctl::rotary_encoder::*;
use playroom_rgbctl::mic::*;

static MODE_MANAGER: Mutex<RefCell<Option<ModeManager>>> = Mutex::new(RefCell::new(None));
static MODE_SELECT: Mutex<RefCell<Option<PC15<Input>>>> = Mutex::new(RefCell::new(None));
static DEBOUNCE: Mutex<RefCell<Option<CounterMs<pac::TIM4>>>> = Mutex::new(RefCell::new(None));
const DEBOUNCE_TIME: u32 = 20;

#[interrupt]
fn EXTI15_10() {
    cortex_m::interrupt::free(|cs| {
        let mut debounce_borrow = DEBOUNCE.borrow(cs).borrow_mut();
        let debounce = debounce_borrow.as_mut().unwrap();
        if debounce.wait().is_ok() {
            MODE_MANAGER
                .borrow(cs)
                .borrow_mut()
                .as_mut()
                .unwrap()
                .next();
            let _ = debounce.start(DEBOUNCE_TIME.millis());
        }
        MODE_SELECT
            .borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .clear_interrupt_pending_bit();
    });
}

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();

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

    // Setup ADC for Microphone input
    Mic::init(
        gpiob.pb1.into_analog(),
        dp.ADC1,
    );

    // Configure mode switch interrupt pin
    let mut syscfg = dp.SYSCFG.constrain();
    let mut mode_select = gpioc.pc15.into_pull_down_input();
    mode_select.make_interrupt_source(&mut syscfg);
    mode_select.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
    mode_select.enable_interrupt(&mut dp.EXTI);
    unsafe {
        cortex_m::peripheral::NVIC::unmask(mode_select.interrupt());
    }
    cortex_m::interrupt::free(|cs| {
        MODE_SELECT.borrow(cs).replace(Some(mode_select));
    });
    // Create global counter for button debouncing
    let mut debounce = dp.TIM4.counter_ms(&clocks);
    let _ = debounce.start(DEBOUNCE_TIME.millis());
    cortex_m::interrupt::free(|cs| {
        DEBOUNCE.borrow(cs).replace(Some(debounce));
    });

    cortex_m::interrupt::free(|cs| {
        MODE_MANAGER.borrow(cs).replace(Some(ModeManager::new()));
    });
    loop {
        cortex_m::interrupt::free(|cs| {
            MODE_MANAGER.borrow(cs).borrow().as_ref().unwrap().process();
        });
        // Process console IO
        console.process(&mut ctx);
    }
}
