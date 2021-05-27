//! Section containing oscillator 2 parameters

use iced::{pick_list, slider, Column, Container, Element, Text};

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::{
    slider::slider_with_labels,
    wavetable_list::{wavetable_list, Wavetable},
};
use crate::ui::style;

pub struct Osc2Section {
    table_list: pick_list::State<Wavetable>,
    wave_slider: slider::State,
    coarse_slider: slider::State,
    fine_slider: slider::State,
    fm_amount_slider: slider::State,
    fm_rate_slider: slider::State,
    sync_slider: slider::State,
    level_slider: slider::State,
}

impl Osc2Section {
    pub fn new() -> Self {
        Self {
            table_list: pick_list::State::<Wavetable>::default(),
            wave_slider: slider::State::new(),
            coarse_slider: slider::State::new(),
            fine_slider: slider::State::new(),
            fm_amount_slider: slider::State::new(),
            fm_rate_slider: slider::State::new(),
            sync_slider: slider::State::new(),
            level_slider: slider::State::new(),
        }
    }

    pub fn view(&mut self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Osc 2").size(style::SECTION_LABEL_TEXT_SIZE))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(wavetable_list(
                "Table",
                &mut self.table_list,
                SoundParameter::Osc2Table,
                params.get_value(SoundParameter::Osc2Table),
            ))
            .push(slider_with_labels(
                "Wave",
                &mut self.wave_slider,
                SoundParameter::Osc2Wave,
                params.get_value(SoundParameter::Osc2Wave),
            ))
            .push(slider_with_labels(
                "Coarse",
                &mut self.coarse_slider,
                SoundParameter::Osc2Coarse,
                params.get_value(SoundParameter::Osc2Coarse),
            ))
            .push(slider_with_labels(
                "Fine",
                &mut self.fine_slider,
                SoundParameter::Osc2Fine,
                params.get_value(SoundParameter::Osc2Fine),
            ))
            .push(slider_with_labels(
                "FM Amt",
                &mut self.fm_amount_slider,
                SoundParameter::Osc2FMAmount,
                params.get_value(SoundParameter::Osc2FMAmount),
            ))
            .push(slider_with_labels(
                "FM Rate",
                &mut self.fm_rate_slider,
                SoundParameter::Osc2FMRate,
                params.get_value(SoundParameter::Osc2FMRate),
            ))
            .push(slider_with_labels(
                "Sync",
                &mut self.sync_slider,
                SoundParameter::Osc2Sync,
                params.get_value(SoundParameter::Osc2Sync),
            ))
            .push(slider_with_labels(
                "Level",
                &mut self.level_slider,
                SoundParameter::Osc2Level,
                params.get_value(SoundParameter::Osc2Level),
            ));
        Container::new(content).style(style::OscSection).into()
    }
}
