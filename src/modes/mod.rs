pub mod manager;

mod audio;
mod dial;
mod shake;
mod tilt;

enum Mode {
    Dial(dial::DialMode),
    Tilt(tilt::TiltMode),
    Audio(audio::AudioMode),
    Shake(shake::ShakeMode),
}

trait ModeRun {
    fn run(&mut self);
}
