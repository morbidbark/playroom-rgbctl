use crate::imu::*;
use crate::rgbcontroller::*;
use vek::Vec3;

use super::ModeRun;

const X: Vec3<f32> = Vec3::new(1.0, 0.0, 0.0);
const Y: Vec3<f32> = Vec3::new(0.0, 1.0, 0.0);
const Z: Vec3<f32> = Vec3::new(0.0, 0.0, 1.0);

fn abs(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & (i32::MAX as u32))
}

pub(super) struct TiltMode;
impl ModeRun for TiltMode {
    fn run(&mut self) {
        if let Ok(q) = cortex_m::interrupt::free(|cs| {
            IMUReader
                .borrow(cs)
                .borrow_mut()
                .as_mut()
                .unwrap()
                .orientation_quat()
        }) {
            cortex_m::interrupt::free(|cs| {
                RGB.borrow(cs).borrow_mut().as_mut().unwrap().set_color(
                    &Color::Red,
                    (255.0 * abs((q * Z).dot(Z))) as u8,
                    true,
                );
                RGB.borrow(cs).borrow_mut().as_mut().unwrap().set_color(
                    &Color::Green,
                    (255.0 * abs((q * Y).dot(Z))) as u8,
                    true,
                );
                RGB.borrow(cs).borrow_mut().as_mut().unwrap().set_color(
                    &Color::Blue,
                    (255.0 * abs((q * X).dot(Z))) as u8,
                    true,
                );
            });
        }
    }
}
