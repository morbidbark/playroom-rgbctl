use cortex_m::interrupt::Mutex;
use core::cell::RefCell;
use stm32f4xx_hal as hal;
use stm32f4xx_hal::{pac::TIM3, gpio::Pin, prelude::*, timer::pwm::*, timer::Channel};

pub static RGB: Mutex<RefCell<Option<RGBController>>> = Mutex::new(RefCell::new(None));

pub enum Color {
    Red,
    Green,
    Blue,
}
pub struct RGBController {
    pwm: Pwm<TIM3, (Channel1<TIM3>, Channel2<TIM3>, Channel3<TIM3>), 1_000_000>,
    r: u8,
    g: u8,
    b: u8,
}
impl RGBController {
    pub fn init(
        red_pin: Pin<'A', 6>,
        green_pin: hal::gpio::Pin<'A', 7>,
        blue_pin: hal::gpio::Pin<'B', 0>,
        timer: TIM3,
        clocks: &hal::rcc::Clocks,
    ) {
        let pwm = timer.pwm(
            (
                Channel1::new(red_pin),
                Channel2::new(green_pin),
                Channel3::new(blue_pin),
            ),
            1.millis::<1, 1_000_000>(),
            &clocks
        );
        let mut ctl = Self { pwm, r: 0, g: 0, b: 0 };
        ctl.on();
        
        cortex_m::interrupt::free(|cs| RGB.borrow(cs).replace(Some(ctl)) );
    }

    pub fn on(&mut self) {
        self.pwm.enable(Channel::C1);
        self.pwm.enable(Channel::C2);
        self.pwm.enable(Channel::C3);
    }
    pub fn off(&mut self) {
        self.pwm.disable(Channel::C1);
        self.pwm.disable(Channel::C2);
        self.pwm.disable(Channel::C3);
    }
    fn update_duty(&mut self, color: &Color) {
        let (channel, value) = match color {
            Color::Red => (Channel::C1, self.r),
            Color::Green => (Channel::C2, self.g),
            Color::Blue => (Channel::C3, self.b),
        };
        let duty_cycle = (self.pwm.get_max_duty() as u32) * (value as u32) / 255;
        self.pwm.set_duty(channel, duty_cycle as u16);
    }
    pub fn set_color(&mut self, color: &Color, value: u8) {
        match color {
            Color::Red => self.r = value,
            Color::Green => self.g = value,
            Color::Blue => self.b = value,
        }
        self.update_duty(color);
    }

    pub fn add_color(&mut self, color: &Color, value: u8) {
        match color {
            Color::Red => self.r = self.r.saturating_add(value),
            Color::Green => self.g = self.g.saturating_add(value),
            Color::Blue => self.b = self.b.saturating_add(value),
        }
        self.update_duty(color);
    }

    pub fn sub_color(&mut self, color: &Color, value: u8) {
        match color {
            Color::Red => self.r = self.r.saturating_sub(value),
            Color::Green => self.g = self.g.saturating_sub(value),
            Color::Blue => self.b = self.b.saturating_sub(value),
        }
        self.update_duty(color);
    }
}
