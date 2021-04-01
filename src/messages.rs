use iced_native;

use crate::params::SoundParameter;

#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(iced_native::Event),
    SoundParameterChange(SoundParameter, i32),
    MidiReceived(Vec<u8>),
    Tick,
}
