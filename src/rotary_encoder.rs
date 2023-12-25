use cortex_m::interrupt::Mutex;
use core::cell::RefCell;
use embedded_hal::digital::v2::InputPin;
use stm32f4xx_hal::gpio::{Pin, Input};
use crate::rgbcontroller::*;

const COLOR_INCREMENT: u8 = 12;

pub static RENCODER: Mutex<RefCell<Option<Encoder<Pin<'A', 0, Input>, Pin<'A', 1, Input>>>>> = Mutex::new(RefCell::new(None));
pub static GENCODER: Mutex<RefCell<Option<Encoder<Pin<'A', 2, Input>, Pin<'A', 3, Input>>>>> = Mutex::new(RefCell::new(None));
pub static BENCODER: Mutex<RefCell<Option<Encoder<Pin<'A', 4, Input>, Pin<'A', 5, Input>>>>> = Mutex::new(RefCell::new(None));

pub struct Encoder<T: InputPin, V: InputPin> {
    clk: T,
    dt: V,
    color: Color,
    clk_prev: bool,
    dt_prev: bool,
}
impl<T: InputPin, V: InputPin> Encoder<T, V> {
    pub fn init(clk: T, dt: V, color: Color) -> Self {
        Self {
            clk,
            dt,
            color,
            clk_prev: false,
            dt_prev: false,
        }
    }
    pub fn process(&mut self) {
        let clk = self.clk.is_low().is_ok_and(|x| x);
        let dt = self.dt.is_low().is_ok_and(|x| x);

        if clk && !self.clk_prev && dt && self.dt_prev {
                cortex_m::interrupt::free(|cs| {
                    if let Some(rgb) = RGB.borrow(cs).borrow_mut().as_mut() {
                        rgb.sub_color(&self.color, COLOR_INCREMENT);
                    }
                });
        } else if dt && !self.dt_prev && clk && self.clk_prev {
            cortex_m::interrupt::free(|cs| {
                if let Some(rgb) = RGB.borrow(cs).borrow_mut().as_mut() {
                    rgb.add_color(&self.color, COLOR_INCREMENT);
                }
            });
        }

        self.clk_prev = clk;
        self.dt_prev = dt;
    }
}
