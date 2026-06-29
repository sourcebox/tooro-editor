//! Panel containing global controls

use iced::{
    Alignment, Element, Length,
    widget::{Button, column, container, row, text},
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
        let row1 = row![
            part_list(part_id).width(Length::Fill),
            {
                let mut button = Button::new(
                    text("Resync")
                        .size(style::BUTTON_TEXT_SIZE)
                        .align_x(Alignment::Center),
                )
                .style(|_, status| style::button(status))
                .width(Length::Fill);
                if device_connected {
                    button = button.on_press(Message::UpdateFromDevice);
                }
                button
            }
            .width(Length::Fill),
        ]
        .spacing(10);

        let row2 = row![
            {
                let mut button = Button::new(
                    text("Load SYX")
                        .size(style::BUTTON_TEXT_SIZE)
                        .align_x(Alignment::Center),
                )
                .style(|_, status| style::button(status))
                .width(Length::Fill);
                if device_connected {
                    button = button.on_press(Message::LoadSysexFile);
                }
                button
            },
            {
                let mut button = Button::new(
                    text("Save SYX")
                        .size(style::BUTTON_TEXT_SIZE)
                        .align_x(Alignment::Center),
                )
                .style(|_, status| style::button(status))
                .width(Length::Fill);
                if device_connected {
                    button = button.on_press(Message::SavePresetSysexFile);
                }
                button
            },
        ]
        .spacing(10);

        container(column![row1, row2].spacing(10)).into()
    }
}
