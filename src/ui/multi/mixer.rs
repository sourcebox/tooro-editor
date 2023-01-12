//! Section containing the mixer parameters

use iced::widget::{Column, Container, Text};
use iced::Element;

use crate::messages::Message;
use crate::params::{GetValue, MultiParameter, MultiParameterValues};
use crate::ui::elements::slider::multi_slider_with_labels;
use crate::ui::style;

pub struct MixerSection {}

impl MixerSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &MultiParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Mix").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(multi_slider_with_labels(
                "Part 1 Vol",
                MultiParameter::VolumePart1,
                params.get_value(MultiParameter::VolumePart1),
            ))
            .push(multi_slider_with_labels(
                "Part 1 Bal",
                MultiParameter::BalancePart1,
                params.get_value(MultiParameter::BalancePart1),
            ))
            .push(multi_slider_with_labels(
                "Part 2 Vol",
                MultiParameter::VolumePart2,
                params.get_value(MultiParameter::VolumePart2),
            ))
            .push(multi_slider_with_labels(
                "Part 2 Bal",
                MultiParameter::BalancePart2,
                params.get_value(MultiParameter::BalancePart2),
            ))
            .push(multi_slider_with_labels(
                "Part 3 Vol",
                MultiParameter::VolumePart3,
                params.get_value(MultiParameter::VolumePart3),
            ))
            .push(multi_slider_with_labels(
                "Part 3 Bal",
                MultiParameter::BalancePart3,
                params.get_value(MultiParameter::BalancePart3),
            ))
            .push(multi_slider_with_labels(
                "Part 4 Vol",
                MultiParameter::VolumePart4,
                params.get_value(MultiParameter::VolumePart4),
            ))
            .push(multi_slider_with_labels(
                "Part 4 Bal",
                MultiParameter::BalancePart4,
                params.get_value(MultiParameter::BalancePart4),
            ));
        Container::new(content).style(style::MixerSection).into()
    }
}
