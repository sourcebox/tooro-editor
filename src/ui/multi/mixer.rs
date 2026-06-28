//! Section containing the mixer parameters

use iced::{
    Color, Element, Padding,
    widget::{column, container, text},
};

use crate::messages::Message;
use crate::params::{GetValue, MultiParameter, MultiParameterValues};
use crate::ui::elements::slider::multi_slider_with_labels;
use crate::ui::style;

pub struct MixerSection;

impl MixerSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &MultiParameterValues) -> Element<'_, Message> {
        container(
            column![
                text("Mix").size(style::SECTION_LABEL_TEXT_SIZE),
                multi_slider_with_labels(
                    "Part 1 Vol",
                    MultiParameter::VolumePart1,
                    params.get_value(MultiParameter::VolumePart1),
                ),
                multi_slider_with_labels(
                    "Part 1 Bal",
                    MultiParameter::BalancePart1,
                    params.get_value(MultiParameter::BalancePart1),
                ),
                multi_slider_with_labels(
                    "Part 2 Vol",
                    MultiParameter::VolumePart2,
                    params.get_value(MultiParameter::VolumePart2),
                ),
                multi_slider_with_labels(
                    "Part 2 Bal",
                    MultiParameter::BalancePart2,
                    params.get_value(MultiParameter::BalancePart2),
                ),
                multi_slider_with_labels(
                    "Part 3 Vol",
                    MultiParameter::VolumePart3,
                    params.get_value(MultiParameter::VolumePart3),
                ),
                multi_slider_with_labels(
                    "Part 3 Bal",
                    MultiParameter::BalancePart3,
                    params.get_value(MultiParameter::BalancePart3),
                ),
                multi_slider_with_labels(
                    "Part 4 Vol",
                    MultiParameter::VolumePart4,
                    params.get_value(MultiParameter::VolumePart4),
                ),
                multi_slider_with_labels(
                    "Part 4 Bal",
                    MultiParameter::BalancePart4,
                    params.get_value(MultiParameter::BalancePart4),
                )
            ]
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING),
        )
        .style(|_| style::section(Color::from_rgb8(0xC0, 0xC0, 0xC0)))
        .into()
    }
}
