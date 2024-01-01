use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

use stm32f4xx_hal::{
    adc::{config::*, Adc},
    gpio::{Analog, Pin},
    pac::ADC1,
};

pub static MIC: Mutex<RefCell<Option<Mic>>> = Mutex::new(RefCell::new(None));

pub struct Mic {
    pin: Pin<'B', 1, Analog>,
    adc: Adc<ADC1>,
}
impl Mic {
    pub fn init(pin: Pin<'B', 1, Analog>, adc: ADC1) {
        let adc_config = AdcConfig::default()
            .continuous(Continuous::Single)
            .clock(Clock::Pclk2_div_2)
            .align(Align::Right)
            .resolution(Resolution::Ten);
        cortex_m::interrupt::free(|cs| {
            MIC.borrow(cs).replace(Some(Self {
                pin,
                adc: Adc::adc1(adc, true, adc_config),
            }));
        });
    }

    pub fn amplitude(&mut self) -> f32 {
        let mut min = u16::MAX;
        let mut max = 0;
        let mut value;
        for _ in 0..10 {
            value = self.adc.convert(&self.pin, SampleTime::Cycles_480);
            if value < min {
                min = value;
            }
            if value > max {
                max = value;
            }
        }
        (max - min) as f32 / 1024.
    }
}
