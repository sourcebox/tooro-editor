use iced::{button, pick_list, Button, Column, Container, Element, Length, Row, Text};

use super::style;
use crate::messages::Message;
use crate::params::MultiParameterValues;
use crate::ui::elements::part_list::{part_list, PartList};

pub struct ManagerPanel {
    part_list: pick_list::State<PartList>,
    refresh_button: button::State,
}

impl ManagerPanel {
    pub fn new() -> Self {
        Self {
            part_list: pick_list::State::<PartList>::default(),
            refresh_button: button::State::new(),
        }
    }

    pub fn view(&mut self, part_id: u8, multi_params: &MultiParameterValues) -> Element<Message> {
        let row = Row::new()
            .padding(5)
            .spacing(10)
            .push(part_list(&mut self.part_list, part_id))
            .push(
                Button::new(
                    &mut self.refresh_button,
                    Text::new("Refresh from device").size(style::BUTTON_TEXT_SIZE),
                )
                .on_press(Message::UpdateFromDevice)
                .style(style::Button::Primary),
            );

        Container::new(Column::new().push(row))
            .padding(5)
            .height(Length::Fill)
            .style(style::MainWindow)
            .into()
    }
}
