//! Section containing the multi fx parameters

use iced::{
    Color, Element, Length, Padding,
    widget::{column, container, text},
};

use crate::messages::Message;
use crate::params::{GetValue, MultiParameter, MultiParameterValues, Parameter};
use crate::ui::elements::{fx_mode_list::fx_mode_list, slider::slider_with_labels};
use crate::ui::style;

pub struct FXSection;

impl FXSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &MultiParameterValues) -> Element<'_, Message> {
        container(
            column![
                text("FX").size(style::SECTION_LABEL_TEXT_SIZE),
                fx_mode_list(
                    "Mode",
                    Parameter::Multi(MultiParameter::FXMode),
                    params.get_value(MultiParameter::FXMode),
                ),
                slider_with_labels(
                    "Length",
                    Parameter::Multi(MultiParameter::FXLength),
                    params.get_value(MultiParameter::FXLength),
                ),
                slider_with_labels(
                    "Feedback",
                    Parameter::Multi(MultiParameter::FXFeedback),
                    params.get_value(MultiParameter::FXFeedback),
                ),
                slider_with_labels(
                    "Mix",
                    Parameter::Multi(MultiParameter::FXMix),
                    params.get_value(MultiParameter::FXMix),
                ),
                slider_with_labels(
                    "Speed",
                    Parameter::Multi(MultiParameter::FXSpeed),
                    params.get_value(MultiParameter::FXSpeed),
                ),
                slider_with_labels(
                    "Depth",
                    Parameter::Multi(MultiParameter::FXDepth),
                    params.get_value(MultiParameter::FXDepth),
                )
            ]
            .height(Length::Fill)
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING),
        )
        .style(|_| style::section(Color::from_rgb8(0x65, 0xA4, 0x7E)))
        .into()
    }
}
