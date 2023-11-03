#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;

use stm32f4xx_hal as hal;
use crate::hal::{pac, prelude::*};

use playroom_rgbctl::console::Console;

// FTDI Pinout
// Black - GND
// Yellow - RX
// Orange - TX


#[entry]
fn main() -> ! {
    
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.MHz()).freeze();

    let gpioa = dp.GPIOA.split();
    let mut console = Console::init(
        gpioa.pa9,
        gpioa.pa10,
        dp.USART1,
        &clocks,
    );

    loop {
        console.process();
    }
}
