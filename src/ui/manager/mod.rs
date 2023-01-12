//! Panel containing global controls

use iced::widget::{Button, Column, Container, Row, Text};
use iced::{theme, Alignment, Element, Length};

use super::style;
use crate::messages::Message;
use crate::ui::elements::part_list::part_list;

pub struct ManagerPanel {}

impl ManagerPanel {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, part_id: u8, device_connected: bool) -> Element<Message> {
        let row1 = Row::new()
            .padding(5)
            .spacing(10)
            .push(
                Column::new()
                    .push(part_list(part_id))
                    .width(Length::FillPortion(2)),
            )
            .push(
                Column::new()
                    .push({
                        let mut button = Button::new(
                            Text::new("Update from device").size(style::BUTTON_TEXT_SIZE),
                        )
                        .style(theme::Button::Primary);
                        if device_connected {
                            button = button.on_press(Message::UpdateFromDevice);
                        }
                        button
                    })
                    .width(Length::FillPortion(4))
                    .align_items(Alignment::End),
            );

        let row2 = Row::new()
            .padding(5)
            .spacing(10)
            .push(
                Column::new()
                    .push({
                        let mut button = Button::new(
                            Text::new("Load syx file...").size(style::BUTTON_TEXT_SIZE),
                        )
                        .style(theme::Button::Primary);
                        if device_connected {
                            button = button.on_press(Message::LoadSysexFile);
                        }
                        button
                    })
                    .width(Length::FillPortion(2)),
            )
            .push(
                Column::new()
                    .push({
                        let mut button = Button::new(
                            Text::new("Save syx file...").size(style::BUTTON_TEXT_SIZE),
                        )
                        .style(theme::Button::Primary);
                        if device_connected {
                            button = button.on_press(Message::SavePresetSysexFile);
                        }
                        button
                    })
                    .width(Length::FillPortion(2))
                    .align_items(Alignment::End),
            );

        Container::new(Column::new().push(row1).push(row2))
            .padding(5)
            .height(Length::Units(80))
            .style(style::MainWindow)
            .into()
    }
}
