use iced::{pick_list, slider, Column, Container, Element, Text};

use crate::messages::Message;
use crate::params::{GetValue, MultiParameter, MultiParameterValues};
use crate::ui::elements::slider::multi_slider_with_labels;
use crate::ui::style;

pub struct MixerSection {
    part1_vol_slider: slider::State,
    part1_bal_slider: slider::State,
    part2_vol_slider: slider::State,
    part2_bal_slider: slider::State,
    part3_vol_slider: slider::State,
    part3_bal_slider: slider::State,
    part4_vol_slider: slider::State,
    part4_bal_slider: slider::State,
}

impl MixerSection {
    pub fn new() -> Self {
        Self {
            part1_vol_slider: slider::State::new(),
            part1_bal_slider: slider::State::new(),
            part2_vol_slider: slider::State::new(),
            part2_bal_slider: slider::State::new(),
            part3_vol_slider: slider::State::new(),
            part3_bal_slider: slider::State::new(),
            part4_vol_slider: slider::State::new(),
            part4_bal_slider: slider::State::new(),
        }
    }

    pub fn view(&mut self, params: &MultiParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Mix").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(multi_slider_with_labels(
                "Part 1 Vol",
                &mut self.part1_vol_slider,
                MultiParameter::VolumePart1,
                params.get_value(MultiParameter::VolumePart1),
            ))
            .push(multi_slider_with_labels(
                "Part 1 Bal",
                &mut self.part1_bal_slider,
                MultiParameter::BalancePart1,
                params.get_value(MultiParameter::BalancePart1),
            ))
            .push(multi_slider_with_labels(
                "Part 2 Vol",
                &mut self.part2_vol_slider,
                MultiParameter::VolumePart2,
                params.get_value(MultiParameter::VolumePart2),
            ))
            .push(multi_slider_with_labels(
                "Part 2 Bal",
                &mut self.part2_bal_slider,
                MultiParameter::BalancePart2,
                params.get_value(MultiParameter::BalancePart2),
            ))
            .push(multi_slider_with_labels(
                "Part 3 Vol",
                &mut self.part3_vol_slider,
                MultiParameter::VolumePart3,
                params.get_value(MultiParameter::VolumePart3),
            ))
            .push(multi_slider_with_labels(
                "Part 3 Bal",
                &mut self.part3_bal_slider,
                MultiParameter::BalancePart3,
                params.get_value(MultiParameter::BalancePart3),
            ))
            .push(multi_slider_with_labels(
                "Part 4 Vol",
                &mut self.part4_vol_slider,
                MultiParameter::VolumePart4,
                params.get_value(MultiParameter::VolumePart4),
            ))
            .push(multi_slider_with_labels(
                "Part 4 Bal",
                &mut self.part4_bal_slider,
                MultiParameter::BalancePart4,
                params.get_value(MultiParameter::BalancePart4),
            ));
        Container::new(content).style(style::MixerSection).into()
    }
}
