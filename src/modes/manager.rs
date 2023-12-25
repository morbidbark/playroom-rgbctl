
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
use vek::Vec3;

const X: Vec3<f32> = Vec3::new(1.0, 0.0, 0.0);
const Y: Vec3<f32> = Vec3::new(0.0, 1.0, 0.0);
const Z: Vec3<f32> = Vec3::new(0.0, 0.0, 1.0);

fn abs(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & (i32::MAX as u32))
}

struct TiltMode;
impl ModeRun for TiltMode {
    fn run(&self) {
        if let Ok(q) = cortex_m::interrupt::free(|cs| {
            IMUReader.borrow(cs).borrow_mut().as_mut().unwrap().orientation_quat()
        }) {
            cortex_m::interrupt::free(|cs| {
                RGB.borrow(cs).borrow_mut().as_mut().unwrap().set_color(
                    &Color::Red, (255.0 * abs((q * Z).dot(Z))) as u8
                );
                RGB.borrow(cs).borrow_mut().as_mut().unwrap().set_color(
                    &Color::Green, (255.0 * abs((q * Y).dot(Z))) as u8
                );
                RGB.borrow(cs).borrow_mut().as_mut().unwrap().set_color(
                    &Color::Blue, (255.0 * abs((q * X).dot(Z))) as u8
                );
            });
        }
    }
}
