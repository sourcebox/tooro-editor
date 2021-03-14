use iced::{pick_list, Container, Length, PickList, Row, Text};

use crate::messages::Message;
use crate::params::SoundParameter;
use crate::style;

pub fn lfo_shape_list<'a>(
    label: &'a str,
    state: &'a mut pick_list::State<LFOShape>,
    sound_param: SoundParameter,
    value: i32,
) -> Container<'a, Message> {
    let value = match value {
        0 => Some(LFOShape::Triangle),
        1 => Some(LFOShape::RampUp),
        2 => Some(LFOShape::RampDown),
        3 => Some(LFOShape::Square),
        4 => Some(LFOShape::MWave),
        5 => Some(LFOShape::Random),
        6 => Some(LFOShape::Slew),
        7 => Some(LFOShape::AM),
        _ => None,
    };
    let pick_list = PickList::new(state, &LFOShape::ALL[..], value, move |v| {
        Message::SoundParameterChange(sound_param, v as i32)
    })
    .style(style::PickList)
    .text_size(16);

    Container::new(
        Row::new()
            .push(Text::new(label).size(16).width(Length::Units(80)))
            .push(pick_list),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LFOShape {
    Triangle,
    RampUp,
    RampDown,
    Square,
    MWave,
    Random,
    Slew,
    AM,
}

impl LFOShape {
    const ALL: [LFOShape; 8] = [
        LFOShape::Triangle,
        LFOShape::RampUp,
        LFOShape::RampDown,
        LFOShape::Square,
        LFOShape::MWave,
        LFOShape::Random,
        LFOShape::Slew,
        LFOShape::AM,
    ];
}

impl std::fmt::Display for LFOShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LFOShape::Triangle => "Triangle",
                LFOShape::RampUp => "Ramp Up",
                LFOShape::RampDown => "Ramp Down",
                LFOShape::Square => "Square",
                LFOShape::MWave => "M-Wave",
                LFOShape::Random => "Random",
                LFOShape::Slew => "Slew",
                LFOShape::AM => "AM",
            }
        )
    }
}
