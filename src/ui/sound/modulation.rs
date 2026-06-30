//! Section containing the modulation parameters

use iced::{
    Color, Element, Length, Padding,
    widget::{column, container, row},
};

use crate::messages::Message;
use crate::params::{GetValue, Parameter, SoundParameter, SoundParameterValues};
use crate::ui::elements::{mod_target_list::mod_target_list, slider::slider_with_labels};
use crate::ui::style;

pub struct ModSection;

impl ModSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, params: &SoundParameterValues) -> Element<'_, Message> {
        container(row![
            column![
                mod_target_list(
                    "MD Target",
                    Parameter::Sound(SoundParameter::ModModwheelTarget),
                    params.get_value(SoundParameter::ModModwheelTarget),
                ),
                slider_with_labels(
                    "MD Amt",
                    Parameter::Sound(SoundParameter::ModModwheelAmount),
                    params.get_value(SoundParameter::ModModwheelAmount),
                )
            ]
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING)
            .width(Length::FillPortion(4)),
            column![
                mod_target_list(
                    "PI Target",
                    Parameter::Sound(SoundParameter::ModPitchTarget),
                    params.get_value(SoundParameter::ModPitchTarget),
                ),
                slider_with_labels(
                    "PI Amt",
                    Parameter::Sound(SoundParameter::ModPitchAmount),
                    params.get_value(SoundParameter::ModPitchAmount),
                )
            ]
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .width(Length::FillPortion(4)),
            column![
                mod_target_list(
                    "VL Target",
                    Parameter::Sound(SoundParameter::ModVelocityTarget),
                    params.get_value(SoundParameter::ModVelocityTarget),
                ),
                slider_with_labels(
                    "VL Amt",
                    Parameter::Sound(SoundParameter::ModVelocityAmount),
                    params.get_value(SoundParameter::ModVelocityAmount),
                )
            ]
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .width(Length::FillPortion(4)),
            column![
                mod_target_list(
                    "AF Target",
                    Parameter::Sound(SoundParameter::ModAftertouchTarget),
                    params.get_value(SoundParameter::ModAftertouchTarget),
                ),
                slider_with_labels(
                    "AF Amt",
                    Parameter::Sound(SoundParameter::ModAftertouchAmount),
                    params.get_value(SoundParameter::ModAftertouchAmount),
                )
            ]
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .width(Length::FillPortion(4)),
        ])
        .style(|_| style::section(Color::from_rgb8(0xB4, 0xCB, 0xD9)))
        .into()
    }
}
