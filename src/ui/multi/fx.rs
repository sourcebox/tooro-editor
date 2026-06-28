//! Section containing the multi fx parameters

use iced::{
    Color, Element, Padding,
    widget::{column, container, text},
};

use crate::messages::Message;
use crate::params::{GetValue, MultiParameter, MultiParameterValues};
use crate::ui::elements::{fx_mode_list::fx_mode_list, slider::multi_slider_with_labels};
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
                    MultiParameter::FXMode,
                    params.get_value(MultiParameter::FXMode),
                ),
                multi_slider_with_labels(
                    "Length",
                    MultiParameter::FXLength,
                    params.get_value(MultiParameter::FXLength),
                ),
                multi_slider_with_labels(
                    "Feedback",
                    MultiParameter::FXFeedback,
                    params.get_value(MultiParameter::FXFeedback),
                ),
                multi_slider_with_labels(
                    "Mix",
                    MultiParameter::FXMix,
                    params.get_value(MultiParameter::FXMix),
                ),
                multi_slider_with_labels(
                    "Speed",
                    MultiParameter::FXSpeed,
                    params.get_value(MultiParameter::FXSpeed),
                ),
                multi_slider_with_labels(
                    "Depth",
                    MultiParameter::FXDepth,
                    params.get_value(MultiParameter::FXDepth),
                )
            ]
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING),
        )
        .style(|_| style::section(Color::from_rgb8(0x65, 0xA4, 0x7E)))
        .into()
    }
}
