//! Section containing the LFO 1 parameters

use iced::widget::{Column, Container, Rule, Text};
use iced::Element;

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::{
    lfo_phase_list::lfo_phase_list, lfo_shape_list::lfo_shape_list,
    mod_target_list::mod_target_list, slider::slider_with_labels,
};
use crate::ui::style;

pub struct LFO1Section {}

impl LFO1Section {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("LFO 1").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(lfo_shape_list(
                "Shape",
                SoundParameter::LFO1Shape,
                params.get_value(SoundParameter::LFO1Shape),
            ))
            .push(slider_with_labels(
                "Speed",
                SoundParameter::LFO1Speed,
                params.get_value(SoundParameter::LFO1Speed),
            ))
            .push(slider_with_labels(
                "Rise",
                SoundParameter::LFO1Rise,
                params.get_value(SoundParameter::LFO1Rise),
            ))
            .push(lfo_phase_list(
                "Phase",
                SoundParameter::LFO1Phase,
                params.get_value(SoundParameter::LFO1Phase),
            ))
            .push(Rule::horizontal(10))
            .push(mod_target_list(
                "Mod Target",
                SoundParameter::ModLFO1Target,
                params.get_value(SoundParameter::ModLFO1Target),
            ))
            .push(slider_with_labels(
                "Mod Amt",
                SoundParameter::ModLFO1Amount,
                params.get_value(SoundParameter::ModLFO1Amount),
            ));
        Container::new(content).style(style::LFOSection).into()
    }
}
