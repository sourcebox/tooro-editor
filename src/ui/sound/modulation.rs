//! Section containing the modulation parameters

use iced::widget::{Column, Container, Row};
use iced::{Element, Length};

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::{mod_target_list::mod_target_list, slider::slider_with_labels};
use crate::ui::style;

pub struct ModSection {}

impl ModSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new().push(
            Row::new()
                .push(
                    Column::new()
                        .padding(style::SECTION_PADDING)
                        .spacing(style::SECTION_SPACING)
                        .push(mod_target_list(
                            "MD Target",
                            SoundParameter::ModModwheelTarget,
                            params.get_value(SoundParameter::ModModwheelTarget),
                        ))
                        .push(slider_with_labels(
                            "MD Amt",
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
                            "PI Target",
                            SoundParameter::ModPitchTarget,
                            params.get_value(SoundParameter::ModPitchTarget),
                        ))
                        .push(slider_with_labels(
                            "PI Amt",
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
                            "VL Target",
                            SoundParameter::ModVelocityTarget,
                            params.get_value(SoundParameter::ModVelocityTarget),
                        ))
                        .push(slider_with_labels(
                            "VL Amt",
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
                            "AF Target",
                            SoundParameter::ModAftertouchTarget,
                            params.get_value(SoundParameter::ModAftertouchTarget),
                        ))
                        .push(slider_with_labels(
                            "AF Amt",
                            SoundParameter::ModAftertouchAmount,
                            params.get_value(SoundParameter::ModAftertouchAmount),
                        ))
                        .width(Length::FillPortion(4)),
                ),
        );
        Container::new(content).style(style::ModSection).into()
    }
}
