use iced::{slider, Column, Container, Element, Text};

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::slider::slider_with_labels;
use crate::ui::style;

pub struct MiscSection {
    bend_slider: slider::State,
    tune_slider: slider::State,
}

impl MiscSection {
    pub fn new() -> Self {
        Self {
            bend_slider: slider::State::new(),
            tune_slider: slider::State::new(),
        }
    }

    pub fn view(&mut self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Misc"))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(slider_with_labels(
                "Bend Range",
                &mut self.bend_slider,
                SoundParameter::BendRange,
                params.get_value(SoundParameter::BendRange),
            ))
            .push(slider_with_labels(
                "Tune",
                &mut self.tune_slider,
                SoundParameter::Tune,
                params.get_value(SoundParameter::Tune),
            ));
        Container::new(content).style(style::MiscSection).into()
    }
}
