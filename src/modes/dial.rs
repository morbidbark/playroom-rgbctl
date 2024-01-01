use crate::rgbcontroller::*;
use crate::rotary_encoder::*;

use super::ModeRun;

pub(super) struct DialMode;
impl ModeRun for DialMode {
    fn run(&mut self) {
        cortex_m::interrupt::free(|cs| {
            RENCODER.borrow(cs).borrow_mut().as_mut().unwrap().process();
            GENCODER.borrow(cs).borrow_mut().as_mut().unwrap().process();
            BENCODER.borrow(cs).borrow_mut().as_mut().unwrap().process();
            RGB.borrow(cs).borrow_mut().as_mut().unwrap().update_all();
        });
    }
}
