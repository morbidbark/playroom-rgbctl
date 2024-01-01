use crate::mic::*;
use crate::rgbcontroller::*;
use crate::rotary_encoder::*;

use super::ModeRun;

const LERP_FACTOR: f32 = 0.3;
pub(super) struct AudioMode(pub(super) f32);
impl AudioMode {
    fn lerp(&mut self, target: f32) -> f32 {
        self.0 + (target - self.0) * LERP_FACTOR
    }
}
impl ModeRun for AudioMode {
    fn run(&mut self) {
        cortex_m::interrupt::free(|cs| {
            RENCODER.borrow(cs).borrow_mut().as_mut().unwrap().process();
            GENCODER.borrow(cs).borrow_mut().as_mut().unwrap().process();
            BENCODER.borrow(cs).borrow_mut().as_mut().unwrap().process();

            let value = MIC.borrow(cs).borrow_mut().as_mut().unwrap().amplitude();
            RGB.borrow(cs)
                .borrow_mut()
                .as_mut()
                .unwrap()
                .scale_all(self.lerp(value));
        });
    }
}
