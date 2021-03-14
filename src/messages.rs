use crate::params::SoundParameter;
#[derive(Debug, Clone, Copy)]
pub enum Message {
    SoundParameterChange(SoundParameter, i32),
    Tick(chrono::DateTime<chrono::Local>),
}
