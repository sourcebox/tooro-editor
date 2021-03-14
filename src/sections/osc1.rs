use iced::{pick_list, slider, Column, Container, Element, Text};

use crate::elements::{
    slider::slider_with_labels,
    wavetable_list::{wavetable_list, Wavetable},
};
use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::style;

pub struct Osc1Section {
    table_list: pick_list::State<Wavetable>,
    wave_slider: slider::State,
    coarse_slider: slider::State,
    fine_slider: slider::State,
    fm_amount_slider: slider::State,
    fm_rate_slider: slider::State,
    sync_slider: slider::State,
    level_slider: slider::State,
}

impl Osc1Section {
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
            .push(Text::new("Osc 1"))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(wavetable_list(
                "Table",
                &mut self.table_list,
                SoundParameter::Osc1Table,
                params.get_value(SoundParameter::Osc1Table),
            ))
            .push(slider_with_labels(
                "Wave",
                &mut self.wave_slider,
                SoundParameter::Osc1Wave,
                params.get_value(SoundParameter::Osc1Wave),
            ))
            .push(slider_with_labels(
                "Coarse",
                &mut self.coarse_slider,
                SoundParameter::Osc1Coarse,
                params.get_value(SoundParameter::Osc1Coarse),
            ))
            .push(slider_with_labels(
                "Fine",
                &mut self.fine_slider,
                SoundParameter::Osc1Fine,
                params.get_value(SoundParameter::Osc1Fine),
            ))
            .push(slider_with_labels(
                "FM Amount",
                &mut self.fm_amount_slider,
                SoundParameter::Osc1FMAmount,
                params.get_value(SoundParameter::Osc1FMAmount),
            ))
            .push(slider_with_labels(
                "FM Rate",
                &mut self.fm_rate_slider,
                SoundParameter::Osc1FMRate,
                params.get_value(SoundParameter::Osc1FMRate),
            ))
            .push(slider_with_labels(
                "Sync",
                &mut self.sync_slider,
                SoundParameter::Osc1Sync,
                params.get_value(SoundParameter::Osc1Sync),
            ))
            .push(slider_with_labels(
                "Level",
                &mut self.level_slider,
                SoundParameter::Osc1Level,
                params.get_value(SoundParameter::Osc1Level),
            ));
        Container::new(content).style(style::OscSection).into()
    }
}
