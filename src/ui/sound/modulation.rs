use iced::{pick_list, slider, Column, Container, Element, Length, Row};

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::{
    mod_target_list::{mod_target_list, ModTarget},
    slider::slider_with_labels,
};
use crate::ui::style;

pub struct ModSection {
    mw_target_list: pick_list::State<ModTarget>,
    mw_amount_slider: slider::State,
    pitch_target_list: pick_list::State<ModTarget>,
    pitch_amount_slider: slider::State,
    vel_target_list: pick_list::State<ModTarget>,
    vel_amount_slider: slider::State,
    at_target_list: pick_list::State<ModTarget>,
    at_amount_slider: slider::State,
}

impl ModSection {
    pub fn new() -> Self {
        Self {
            mw_target_list: pick_list::State::<ModTarget>::default(),
            mw_amount_slider: slider::State::new(),
            pitch_target_list: pick_list::State::<ModTarget>::default(),
            pitch_amount_slider: slider::State::new(),
            vel_target_list: pick_list::State::<ModTarget>::default(),
            vel_amount_slider: slider::State::new(),
            at_target_list: pick_list::State::<ModTarget>::default(),
            at_amount_slider: slider::State::new(),
        }
    }

    pub fn view(&mut self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new().push(
            Row::new()
                .push(
                    Column::new()
                        .padding(style::SECTION_PADDING)
                        .spacing(style::SECTION_SPACING)
                        .push(mod_target_list(
                            "MW Target",
                            &mut self.mw_target_list,
                            SoundParameter::ModModwheelTarget,
                            params.get_value(SoundParameter::ModModwheelTarget),
                        ))
                        .push(slider_with_labels(
                            "MW Amount",
                            &mut self.mw_amount_slider,
                            SoundParameter::ModModwheelAmount,
                            params.get_value(SoundParameter::ModModwheelAmount),
                        ))
                        .width(Length::FillPortion(4)),
                )
                .push(
                    Column::new()
                        .padding(style::SECTION_PADDING)
                        .spacing(style::SECTION_SPACING)
                        .push(mod_target_list(
                            "Pitch Target",
                            &mut self.pitch_target_list,
                            SoundParameter::ModPitchTarget,
                            params.get_value(SoundParameter::ModPitchTarget),
                        ))
                        .push(slider_with_labels(
                            "Pitch Amt",
                            &mut self.pitch_amount_slider,
                            SoundParameter::ModPitchAmount,
                            params.get_value(SoundParameter::ModPitchAmount),
                        ))
                        .width(Length::FillPortion(4)),
                )
                .push(
                    Column::new()
                        .padding(style::SECTION_PADDING)
                        .spacing(style::SECTION_SPACING)
                        .push(mod_target_list(
                            "Vel Target",
                            &mut self.vel_target_list,
                            SoundParameter::ModVelocityTarget,
                            params.get_value(SoundParameter::ModVelocityTarget),
                        ))
                        .push(slider_with_labels(
                            "Vel Amount",
                            &mut self.vel_amount_slider,
                            SoundParameter::ModVelocityAmount,
                            params.get_value(SoundParameter::ModVelocityAmount),
                        ))
                        .width(Length::FillPortion(4)),
                )
                .push(
                    Column::new()
                        .padding(style::SECTION_PADDING)
                        .spacing(style::SECTION_SPACING)
                        .push(mod_target_list(
                            "AT Target",
                            &mut self.at_target_list,
                            SoundParameter::ModAftertouchTarget,
                            params.get_value(SoundParameter::ModAftertouchTarget),
                        ))
                        .push(slider_with_labels(
                            "AT Amount",
                            &mut self.at_amount_slider,
                            SoundParameter::ModAftertouchAmount,
                            params.get_value(SoundParameter::ModAftertouchAmount),
                        ))
                        .width(Length::FillPortion(4)),
                ),
        );
        Container::new(content).style(style::ModSection).into()
    }
}
