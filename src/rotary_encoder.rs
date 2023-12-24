use core::ops::DerefMut;
use embedded_hal::digital::v2::InputPin;
use crate::rgbcontroller::*;

const COLOR_INCREMENT: u8 = 12;

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
                    if let Some(rgb) = RGB.borrow(cs).borrow_mut().deref_mut() {
                        rgb.sub_color(&self.color, COLOR_INCREMENT);
                    }
                });
        } else if dt && !self.dt_prev && clk && self.clk_prev {
            cortex_m::interrupt::free(|cs| {
                if let Some(rgb) = RGB.borrow(cs).borrow_mut().deref_mut() {
                    rgb.add_color(&self.color, COLOR_INCREMENT);
                }
            });
        }

        self.clk_prev = clk;
        self.dt_prev = dt;
    }
}
