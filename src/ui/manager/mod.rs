//! User interface panel containing global controls

use iced::{button, pick_list, Button, Column, Container, Element, Length, Row, Text};

use super::style;
use crate::messages::Message;
use crate::ui::elements::part_list::{part_list, PartList};

pub struct ManagerPanel {
    part_list: pick_list::State<PartList>,
    update_button: button::State,
    load_syx_button: button::State,
    save_preset_syx_button: button::State,
}

impl ManagerPanel {
    pub fn new() -> Self {
        Self {
            /// Drop down list for selecting the current part
            part_list: pick_list::State::<PartList>::default(),

            /// Button to request an update from the device
            update_button: button::State::new(),

            /// Button to open the load file dialog
            load_syx_button: button::State::new(),

            /// Button to open the save file dialog
            save_preset_syx_button: button::State::new(),
        }
    }

    pub fn view(&mut self, part_id: u8) -> Element<Message> {
        let row1 = Row::new()
            .padding(5)
            .spacing(10)
            .push(part_list(&mut self.part_list, part_id))
            .push(
                Button::new(
                    &mut self.update_button,
                    Text::new("Update from device").size(style::BUTTON_TEXT_SIZE),
                )
                .on_press(Message::UpdateFromDevice)
                .style(style::Button::Primary),
            );

        let row2 = Row::new()
            .padding(5)
            .spacing(10)
            .push(
                Button::new(
                    &mut self.load_syx_button,
                    Text::new("Load syx file...").size(style::BUTTON_TEXT_SIZE),
                )
                .on_press(Message::LoadSysexFile)
                .style(style::Button::Primary),
            )
            .push(
                Button::new(
                    &mut self.save_preset_syx_button,
                    Text::new("Save syx file...").size(style::BUTTON_TEXT_SIZE),
                )
                .on_press(Message::SavePresetSysexFile)
                .style(style::Button::Primary),
            );

        Container::new(Column::new().push(row1).push(row2))
            .padding(5)
            .height(Length::Units(80))
            .style(style::MainWindow)
            .into()
    }
}
