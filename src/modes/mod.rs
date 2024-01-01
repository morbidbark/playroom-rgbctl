pub mod manager;

mod audio;
mod dial;
mod tilt;

enum Mode {
    Dial(dial::DialMode),
    Tilt(tilt::TiltMode),
    Audio(audio::AudioMode),
}

trait ModeRun {
    fn run(&mut self);
}
