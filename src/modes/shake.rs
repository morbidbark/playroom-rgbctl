use crate::imu::*;
use crate::rgbcontroller::*;

use super::ModeRun;

const MAX_ACCEL: f32 = 30.;
const LERP_FACTOR: f32 = 0.1;
fn lerp(current: f32, target: f32) -> f32 {
    current + (target - current) * LERP_FACTOR
}
pub(super) struct ShakeMode(pub f32, pub f32, pub f32);
impl ModeRun for ShakeMode {
    fn run(&mut self) {
        if let Ok((x, y, z)) = cortex_m::interrupt::free(|cs| {
            IMUReader.borrow(cs).borrow_mut().as_mut().unwrap().accel()
        }) {
            self.0 = lerp(self.0, x.abs() as f32 / MAX_ACCEL);
            self.1 = lerp(self.1, y.abs() as f32 / MAX_ACCEL);
            self.2 = lerp(self.2, z.abs() as f32 / MAX_ACCEL);
            cortex_m::interrupt::free(|cs| {
                if let Some(rgb) = RGB.borrow(cs).borrow_mut().as_mut() {
                    rgb.set_color(&Color::Red, 255, false);
                    rgb.set_color(&Color::Green, 255, false);
                    rgb.set_color(&Color::Blue, 255, false);
                    rgb.scale(&Color::Red, self.0);
                    rgb.scale(&Color::Green, self.1);
                    rgb.scale(&Color::Blue, self.2);
                }
            });
        }
    }
}
