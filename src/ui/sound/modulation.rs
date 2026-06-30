//! Section containing the modulation parameters

use iced::{
    Color, Element, Length, Padding,
    widget::{column, container, row},
};

use crate::messages::Message;
use crate::params::{Parameter, ParameterValues, SoundParameter};
use crate::ui::elements::{mod_target_list::mod_target_list, slider::slider_with_labels};
use crate::ui::style;

pub struct ModSection;

impl ModSection {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view<'a>(&'a self, values: &'a ParameterValues) -> Element<'a, Message> {
        container(row![
            column![
                mod_target_list(
                    "MD Target",
                    Parameter::Sound(SoundParameter::ModModwheelTarget),
                    values
                ),
                slider_with_labels(
                    "MD Amt",
                    Parameter::Sound(SoundParameter::ModModwheelAmount),
                    values
                )
            ]
            .padding(Padding::from(style::SECTION_PADDING))
            .spacing(style::SECTION_SPACING)
            .width(Length::FillPortion(4)),
            column![
                mod_target_list(
                    "PI Target",
                    Parameter::Sound(SoundParameter::ModPitchTarget),
                    values
                ),
                slider_with_labels(
                    "PI Amt",
                    Parameter::Sound(SoundParameter::ModPitchAmount),
                    values
                )
            ]
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .width(Length::FillPortion(4)),
            column![
                mod_target_list(
                    "VL Target",
                    Parameter::Sound(SoundParameter::ModVelocityTarget),
                    values
                ),
                slider_with_labels(
                    "VL Amt",
                    Parameter::Sound(SoundParameter::ModVelocityAmount),
                    values
                )
            ]
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .width(Length::FillPortion(4)),
            column![
                mod_target_list(
                    "AF Target",
                    Parameter::Sound(SoundParameter::ModAftertouchTarget),
                    values
                ),
                slider_with_labels(
                    "AF Amt",
                    Parameter::Sound(SoundParameter::ModAftertouchAmount),
                    values
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
