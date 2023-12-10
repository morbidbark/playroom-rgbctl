use stm32f4xx_hal as hal;
use stm32f4xx_hal::{pac::TIM3, gpio::Pin, prelude::*, timer::pwm::*, timer::Channel};

pub enum Color {
    Red,
    Green,
    Blue,
}
pub struct RGBController {
    pwm: Pwm<TIM3, (Channel1<TIM3>, Channel2<TIM3>, Channel3<TIM3>), 1000>,
}
impl RGBController {
    pub fn init(
        red_pin: Pin<'A', 6>,
        green_pin: hal::gpio::Pin<'A', 7>,
        blue_pin: hal::gpio::Pin<'B', 0>,
        timer: TIM3,
        clocks: &hal::rcc::Clocks,
    ) -> Self {
        let pwm = timer.pwm(
            (
                Channel1::new(red_pin),
                Channel2::new(green_pin),
                Channel3::new(blue_pin),
            ),
            10.millis::<1, 1000>(),
            &clocks
        );
        let mut ctl = Self { pwm };
        ctl.on();
        ctl
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
    pub fn set_color(&mut self, color: Color, value: u8) {
        let channel = match color {
            Color::Red => Channel::C1,
            Color::Green => Channel::C2,
            Color::Blue => Channel::C3,
        };
        let duty_cycle = (self.pwm.get_max_duty() * value as u16) / 255;
        self.pwm.set_duty(channel, duty_cycle);
    }
}
