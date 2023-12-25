
enum Mode {
    Dial(DialMode),
    Tilt(TiltMode),
}

trait ModeRun {
    fn run(&self);
}

pub struct ModeManager {
    mode: Mode,
}
impl ModeManager {
    pub fn new() -> Self {
        Self {
            mode: Mode::Dial(DialMode),
        }
    }
    pub fn next(&mut self) {
        self.mode = match self.mode {
            Mode::Dial(_) => Mode::Tilt(TiltMode),
            Mode::Tilt(_) => Mode::Dial(DialMode),
        };
    }
    pub fn process(&self) {
        match &self.mode {
            Mode::Dial(m) => m.run(),
            Mode::Tilt(m) => m.run(),
        }
    }
}

use crate::rotary_encoder::*;
pub struct DialMode;
impl ModeRun for DialMode {
    fn run(&self) {
        cortex_m::interrupt::free(|cs| {
            RENCODER.borrow(cs).borrow_mut().as_mut().unwrap().process();
            GENCODER.borrow(cs).borrow_mut().as_mut().unwrap().process();
            BENCODER.borrow(cs).borrow_mut().as_mut().unwrap().process();
        });
    }
}

use crate::imu::*;
use crate::rgbcontroller::*;
struct TiltMode;
impl ModeRun for TiltMode {
    fn run(&self) {
        if let Ok((pitch, yaw, roll)) = cortex_m::interrupt::free(|cs| {
            IMUReader.borrow(cs).borrow_mut().as_mut().unwrap().orientation()
        }) {
            // pitch is -180 -  180 starting at 0. pitch down is positive
            // yaw is 0 - 360 starting at 0. clockwise is positive
            // roll is -90 - 90 starting at 0. roll left is positive
            cortex_m::interrupt::free(|cs| {
                RGB.borrow(cs).borrow_mut().as_mut().unwrap().set_color(
                    &Color::Red, (127 + pitch) as u8
                );
                RGB.borrow(cs).borrow_mut().as_mut().unwrap().set_color(
                    &Color::Green, (127 + (yaw - 180)) as u8
                );
                RGB.borrow(cs).borrow_mut().as_mut().unwrap().set_color(
                    &Color::Blue, (127 + roll) as u8
                );
            });
        }
    }
}
