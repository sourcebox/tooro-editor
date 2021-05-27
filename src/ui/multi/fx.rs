//! Section containing the multi fx parameters

use iced::{pick_list, slider, Column, Container, Element, Text};

use crate::messages::Message;
use crate::params::{GetValue, MultiParameter, MultiParameterValues};
use crate::ui::elements::{
    fx_mode_list::{fx_mode_list, FXMode},
    slider::multi_slider_with_labels,
};
use crate::ui::style;

pub struct FXSection {
    mode_list: pick_list::State<FXMode>,
    length_slider: slider::State,
    feedback_slider: slider::State,
    mix_slider: slider::State,
    speed_slider: slider::State,
    depth_slider: slider::State,
}

impl FXSection {
    pub fn new() -> Self {
        Self {
            mode_list: pick_list::State::<FXMode>::default(),
            length_slider: slider::State::new(),
            feedback_slider: slider::State::new(),
            mix_slider: slider::State::new(),
            speed_slider: slider::State::new(),
            depth_slider: slider::State::new(),
        }
    }

    pub fn view(&mut self, params: &MultiParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("FX").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(fx_mode_list(
                "Mode",
                &mut self.mode_list,
                MultiParameter::FXMode,
                params.get_value(MultiParameter::FXMode),
            ))
            .push(multi_slider_with_labels(
                "Length",
                &mut self.length_slider,
                MultiParameter::FXLength,
                params.get_value(MultiParameter::FXLength),
            ))
            .push(multi_slider_with_labels(
                "Feedback",
                &mut self.feedback_slider,
                MultiParameter::FXFeedback,
                params.get_value(MultiParameter::FXFeedback),
            ))
            .push(multi_slider_with_labels(
                "Mix",
                &mut self.mix_slider,
                MultiParameter::FXMix,
                params.get_value(MultiParameter::FXMix),
            ))
            .push(multi_slider_with_labels(
                "Speed",
                &mut self.speed_slider,
                MultiParameter::FXSpeed,
                params.get_value(MultiParameter::FXSpeed),
            ))
            .push(multi_slider_with_labels(
                "Depth",
                &mut self.depth_slider,
                MultiParameter::FXDepth,
                params.get_value(MultiParameter::FXDepth),
            ));
        Container::new(content).style(style::FXSection).into()
    }
}
