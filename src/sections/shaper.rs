use iced::{pick_list, slider, Column, Container, Element, Text};

use crate::elements::{
    shaper_mode_list::{shaper_mode_list, ShaperMode},
    slider::slider_with_labels,
};
use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::style;

pub struct ShaperSection {
    cutoff_slider: slider::State,
    resonance_slider: slider::State,
    enva_amount_slider: slider::State,
    track_slider: slider::State,
    mode_list: pick_list::State<ShaperMode>,
    lfo2_amount_slider: slider::State,
}

impl ShaperSection {
    pub fn new() -> Self {
        Self {
            cutoff_slider: slider::State::new(),
            resonance_slider: slider::State::new(),
            enva_amount_slider: slider::State::new(),
            track_slider: slider::State::new(),
            mode_list: pick_list::State::<ShaperMode>::default(),
            lfo2_amount_slider: slider::State::new(),
        }
    }

    pub fn view(&mut self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Shaper"))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(slider_with_labels(
                "Cutoff",
                &mut self.cutoff_slider,
                SoundParameter::ShaperCutoff,
                params.get_value(SoundParameter::ShaperCutoff),
            ))
            .push(slider_with_labels(
                "Resonance",
                &mut self.resonance_slider,
                SoundParameter::ShaperResonance,
                params.get_value(SoundParameter::ShaperResonance),
            ))
            .push(slider_with_labels(
                "Env A Amt",
                &mut self.enva_amount_slider,
                SoundParameter::ShaperEnvAAmount,
                params.get_value(SoundParameter::ShaperEnvAAmount),
            ))
            .push(slider_with_labels(
                "Track",
                &mut self.track_slider,
                SoundParameter::ShaperTrack,
                params.get_value(SoundParameter::ShaperTrack),
            ))
            .push(shaper_mode_list(
                "Mode",
                &mut self.mode_list,
                SoundParameter::ShaperMode,
                params.get_value(SoundParameter::ShaperMode),
            ))
            .push(slider_with_labels(
                "LFO 2 Amt",
                &mut self.lfo2_amount_slider,
                SoundParameter::ShaperLFO2Amount,
                params.get_value(SoundParameter::ShaperLFO2Amount),
            ));
        Container::new(content).style(style::ShaperSection).into()
    }
}
