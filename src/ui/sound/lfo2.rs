//! Section containing the LFO 2 parameters

use iced::widget::{Column, Container, Rule, Text};
use iced::Element;

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::{
    lfo_phase_list::lfo_phase_list, lfo_shape_list::lfo_shape_list,
    mod_target_list::mod_target_list, slider::slider_with_labels,
};
use crate::ui::style;

pub struct LFO2Section {}

impl LFO2Section {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("LFO 2").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(lfo_shape_list(
                "Shape",
                SoundParameter::LFO2Shape,
                params.get_value(SoundParameter::LFO2Shape),
            ))
            .push(slider_with_labels(
                "Speed",
                SoundParameter::LFO2Speed,
                params.get_value(SoundParameter::LFO2Speed),
            ))
            .push(slider_with_labels(
                "Rise",
                SoundParameter::LFO2Rise,
                params.get_value(SoundParameter::LFO2Rise),
            ))
            .push(lfo_phase_list(
                "Phase",
                SoundParameter::LFO2Phase,
                params.get_value(SoundParameter::LFO2Phase),
            ))
            .push(Rule::horizontal(10))
            .push(mod_target_list(
                "Mod Target",
                SoundParameter::ModLFO2Target,
                params.get_value(SoundParameter::ModLFO2Target),
            ))
            .push(slider_with_labels(
                "Mod Amt",
                SoundParameter::ModLFO2Amount,
                params.get_value(SoundParameter::ModLFO2Amount),
            ));
        Container::new(content).style(style::LFOSection).into()
    }
}
