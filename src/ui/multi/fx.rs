//! Section containing the multi fx parameters

use iced::{
    Color, Element, Length, Padding,
    widget::{column, container, text},
};

use crate::messages::Message;
use crate::params::{MultiParameter, Parameter, ParameterValues};
use crate::ui::elements::{fx_mode_list::fx_mode_list, slider::slider_with_labels};
use crate::ui::style;

pub struct FXSection;

impl FXSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view<'a>(&self, values: &'a ParameterValues) -> Element<'a, Message> {
        container(
            column![
                text("FX").size(style::SECTION_LABEL_TEXT_SIZE),
                fx_mode_list("Mode", Parameter::Multi(MultiParameter::FXMode), values),
                slider_with_labels("Length", Parameter::Multi(MultiParameter::FXLength), values),
                slider_with_labels(
                    "Feedback",
                    Parameter::Multi(MultiParameter::FXFeedback),
                    values
                ),
                slider_with_labels("Mix", Parameter::Multi(MultiParameter::FXMix), values),
                slider_with_labels("Speed", Parameter::Multi(MultiParameter::FXSpeed), values),
                slider_with_labels("Depth", Parameter::Multi(MultiParameter::FXDepth), values)
            ]
            .height(Length::Fill)
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING),
        )
        .style(|_| style::section(Color::from_rgb8(0x65, 0xA4, 0x7E)))
        .into()
    }
}
