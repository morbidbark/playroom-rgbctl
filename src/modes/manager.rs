use super::*;

pub struct ModeManager {
    mode: Mode,
}
impl ModeManager {
    pub fn new() -> Self {
        Self {
            mode: Mode::Dial(dial::DialMode),
        }
    }
    pub fn next(&mut self) {
        self.mode = match self.mode {
            Mode::Dial(_) => Mode::Tilt(tilt::TiltMode),
            Mode::Tilt(_) => Mode::Audio(audio::AudioMode(0.)),
            Mode::Audio(_) => Mode::Shake(shake::ShakeMode(0.,0.,0.)),
            Mode::Shake(_) => Mode::Dial(dial::DialMode),
        };
    }
    pub fn process(&mut self) {
        match &mut self.mode {
            Mode::Dial(m) => m.run(),
            Mode::Tilt(m) => m.run(),
            Mode::Audio(m) => m.run(),
            Mode::Shake(m) => m.run(),
        }
    }
}
