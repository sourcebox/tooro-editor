//! Section containing the mixer parameters

use iced::{
    Color, Element, Padding,
    widget::{column, container, text},
};

use crate::messages::Message;
use crate::params::{GetValue, MultiParameter, MultiParameterValues, Parameter};
use crate::ui::elements::slider::slider_with_labels;
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
                slider_with_labels(
                    "Part 1 Vol",
                    Parameter::Multi(MultiParameter::VolumePart1),
                    params.get_value(MultiParameter::VolumePart1),
                ),
                slider_with_labels(
                    "Part 1 Bal",
                    Parameter::Multi(MultiParameter::BalancePart1),
                    params.get_value(MultiParameter::BalancePart1),
                ),
                slider_with_labels(
                    "Part 2 Vol",
                    Parameter::Multi(MultiParameter::VolumePart2),
                    params.get_value(MultiParameter::VolumePart2),
                ),
                slider_with_labels(
                    "Part 2 Bal",
                    Parameter::Multi(MultiParameter::BalancePart2),
                    params.get_value(MultiParameter::BalancePart2),
                ),
                slider_with_labels(
                    "Part 3 Vol",
                    Parameter::Multi(MultiParameter::VolumePart3),
                    params.get_value(MultiParameter::VolumePart3),
                ),
                slider_with_labels(
                    "Part 3 Bal",
                    Parameter::Multi(MultiParameter::BalancePart3),
                    params.get_value(MultiParameter::BalancePart3),
                ),
                slider_with_labels(
                    "Part 4 Vol",
                    Parameter::Multi(MultiParameter::VolumePart4),
                    params.get_value(MultiParameter::VolumePart4),
                ),
                slider_with_labels(
                    "Part 4 Bal",
                    Parameter::Multi(MultiParameter::BalancePart4),
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
