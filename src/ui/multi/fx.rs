//! Section containing the multi fx parameters

use iced::widget::{Column, Container, Text};
use iced::Element;

use crate::messages::Message;
use crate::params::{GetValue, MultiParameter, MultiParameterValues};
use crate::ui::elements::{fx_mode_list::fx_mode_list, slider::multi_slider_with_labels};
use crate::ui::style;

pub struct FXSection {}

impl FXSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &MultiParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("FX").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(fx_mode_list(
                "Mode",
                MultiParameter::FXMode,
                params.get_value(MultiParameter::FXMode),
            ))
            .push(multi_slider_with_labels(
                "Length",
                MultiParameter::FXLength,
                params.get_value(MultiParameter::FXLength),
            ))
            .push(multi_slider_with_labels(
                "Feedback",
                MultiParameter::FXFeedback,
                params.get_value(MultiParameter::FXFeedback),
            ))
            .push(multi_slider_with_labels(
                "Mix",
                MultiParameter::FXMix,
                params.get_value(MultiParameter::FXMix),
            ))
            .push(multi_slider_with_labels(
                "Speed",
                MultiParameter::FXSpeed,
                params.get_value(MultiParameter::FXSpeed),
            ))
            .push(multi_slider_with_labels(
                "Depth",
                MultiParameter::FXDepth,
                params.get_value(MultiParameter::FXDepth),
            ));
        Container::new(content)
            // .style(style::FXSection)
            .into()
    }
}
