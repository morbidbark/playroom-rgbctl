pub mod manager;

mod audio;
mod dial;
mod tilt;
mod shake;

enum Mode {
    Dial(dial::DialMode),
    Tilt(tilt::TiltMode),
    Audio(audio::AudioMode),
    Shake(shake::ShakeMode),
}

trait ModeRun {
    fn run(&mut self);
}
