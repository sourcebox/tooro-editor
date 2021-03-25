use crate::params::SoundParameter;
#[derive(Debug, Clone, Copy)]
pub enum Message {
    SoundParameterChange(SoundParameter, i32),
    MidiCCReceived(u8, u8, u8),
    MidiUnknownReceived,
    Tick,
}
