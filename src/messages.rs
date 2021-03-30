use crate::params::SoundParameter;

#[derive(Debug, Clone)]
pub enum Message {
    SoundParameterChange(SoundParameter, i32),
    MidiReceived(Vec<u8>),
    Tick,
}
