use iced::{slider, Column, Container, Element, Text};

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::slider::slider_with_labels;
use crate::ui::style;

pub struct AmpSection {
    level_slider: slider::State,
    pan_slider: slider::State,
}

impl AmpSection {
    pub fn new() -> Self {
        Self {
            level_slider: slider::State::new(),
            pan_slider: slider::State::new(),
        }
    }

    pub fn view(&mut self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Amp").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(slider_with_labels(
                "Level",
                &mut self.level_slider,
                SoundParameter::AmpLevel,
                params.get_value(SoundParameter::AmpLevel),
            ))
            .push(slider_with_labels(
                "Pan",
                &mut self.pan_slider,
                SoundParameter::AmpPan,
                params.get_value(SoundParameter::AmpPan),
            ));
        Container::new(content).style(style::AmpSection).into()
    }
}
