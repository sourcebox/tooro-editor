//! Panel containing global controls

use iced::{
    Alignment, Element, Length,
    widget::{Button, Column, Container, Row, Text},
};

use super::style;
use crate::messages::Message;
use crate::ui::elements::part_list::part_list;

pub struct ManagerPanel {}

impl ManagerPanel {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, part_id: u8, device_connected: bool) -> Element<'_, Message> {
        let row1 = Row::new()
            .padding(5)
            .spacing(10)
            .push(Column::new().push(part_list(part_id)))
            .push(
                Column::new()
                    .push({
                        let mut button =
                            Button::new(Text::new("Resync").size(style::BUTTON_TEXT_SIZE))
                                .style(|_, status| style::button(status));
                        if device_connected {
                            button = button.on_press(Message::UpdateFromDevice);
                        }
                        button
                    })
                    .width(Length::Shrink),
            );

        let row2 = Row::new()
            .padding(5)
            .spacing(10)
            .push(Column::new().push({
                let mut button = Button::new(Text::new("Load SYX").size(style::BUTTON_TEXT_SIZE))
                    .style(|_, status| style::button(status))
                    .padding(5);
                if device_connected {
                    button = button.on_press(Message::LoadSysexFile);
                }
                button
            }))
            .push(
                Column::new()
                    .push({
                        let mut button =
                            Button::new(Text::new("Save SYX").size(style::BUTTON_TEXT_SIZE))
                                .style(|_, status| style::button(status))
                                .padding(5);
                        if device_connected {
                            button = button.on_press(Message::SavePresetSysexFile);
                        }
                        button
                    })
                    .align_x(Alignment::Start),
            );

        Container::new(Column::new().push(row1).push(row2))
            .padding(5)
            .height(80)
            .into()
    }
}
