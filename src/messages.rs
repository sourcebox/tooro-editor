use iced_native;

use crate::params::{MultiParameter, SoundParameter};

#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(iced_native::Event),
    SoundParameterChange(SoundParameter, i32),
    MultiParameterChange(MultiParameter, i32),
    PartChange(u8),
    UpdateFromDevice,
    LoadSysexFile,
    Tick,
    FastTick,
}
