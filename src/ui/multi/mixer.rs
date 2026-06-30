//! Section containing the mixer parameters

use iced::{
    Color, Element, Padding,
    widget::{column, container, text},
};

use crate::messages::Message;
use crate::params::{MultiParameter, Parameter, ParameterValues};
use crate::ui::elements::slider::slider_with_labels;
use crate::ui::style;

pub struct MixerSection;

impl MixerSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view<'a>(&'a self, values: &'a ParameterValues) -> Element<'a, Message> {
        container(
            column![
                text("Mix").size(style::SECTION_LABEL_TEXT_SIZE),
                slider_with_labels(
                    "Part 1 Vol",
                    Parameter::Multi(MultiParameter::VolumePart1),
                    values
                ),
                slider_with_labels(
                    "Part 1 Bal",
                    Parameter::Multi(MultiParameter::BalancePart1),
                    values
                ),
                slider_with_labels(
                    "Part 2 Vol",
                    Parameter::Multi(MultiParameter::VolumePart2),
                    values
                ),
                slider_with_labels(
                    "Part 2 Bal",
                    Parameter::Multi(MultiParameter::BalancePart2),
                    values
                ),
                slider_with_labels(
                    "Part 3 Vol",
                    Parameter::Multi(MultiParameter::VolumePart3),
                    values
                ),
                slider_with_labels(
                    "Part 3 Bal",
                    Parameter::Multi(MultiParameter::BalancePart3),
                    values
                ),
                slider_with_labels(
                    "Part 4 Vol",
                    Parameter::Multi(MultiParameter::VolumePart4),
                    values
                ),
                slider_with_labels(
                    "Part 4 Bal",
                    Parameter::Multi(MultiParameter::BalancePart4),
                    values
                )
            ]
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING),
        )
        .style(|_| style::section(Color::from_rgb8(0xC0, 0xC0, 0xC0)))
        .into()
    }
}
