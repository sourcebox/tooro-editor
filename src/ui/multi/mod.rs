mod fx;

use iced::{Column, Container, Element, Length, Row};

use super::style;
use crate::messages::Message;
use crate::params::MultiParameterValues;
use fx::FXSection;

pub struct MultiPanel {
    fx_section: FXSection,
}

impl MultiPanel {
    pub fn new() -> Self {
        Self {
            fx_section: FXSection::new(),
        }
    }

    pub fn view(&mut self, params: &MultiParameterValues) -> Element<Message> {
        let col = Column::new()
            .padding(5)
            .spacing(10)
            .push(self.fx_section.view(params));

        Container::new(Column::new().push(Row::new().push(col)))
            .padding(5)
            .height(Length::Fill)
            .style(style::MainWindow)
            .into()
    }
}
