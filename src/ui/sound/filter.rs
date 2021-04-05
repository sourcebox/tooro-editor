use iced::{slider, Column, Container, Element, Text};

use crate::messages::Message;
use crate::params::{GetValue, SoundParameter, SoundParameterValues};
use crate::ui::elements::slider::slider_with_labels;
use crate::ui::style;

pub struct FilterSection {
    cutoff_slider: slider::State,
    resonance_slider: slider::State,
    envf_amount_slider: slider::State,
    track_slider: slider::State,
    after_slider: slider::State,
    lfo1_amount_slider: slider::State,
}

impl FilterSection {
    pub fn new() -> Self {
        Self {
            cutoff_slider: slider::State::new(),
            resonance_slider: slider::State::new(),
            envf_amount_slider: slider::State::new(),
            track_slider: slider::State::new(),
            after_slider: slider::State::new(),
            lfo1_amount_slider: slider::State::new(),
        }
    }

    pub fn view(&mut self, params: &SoundParameterValues) -> Element<Message> {
        let content = Column::new()
            .push(Text::new("Filter"))
            .padding(style::SECTION_PADDING)
            .spacing(style::SECTION_SPACING)
            .push(slider_with_labels(
                "Cutoff",
                &mut self.cutoff_slider,
                SoundParameter::FilterCutoff,
                params.get_value(SoundParameter::FilterCutoff),
            ))
            .push(slider_with_labels(
                "Resonance",
                &mut self.resonance_slider,
                SoundParameter::FilterResonance,
                params.get_value(SoundParameter::FilterResonance),
            ))
            .push(slider_with_labels(
                "Env F Amt",
                &mut self.envf_amount_slider,
                SoundParameter::FilterEnvFAmount,
                params.get_value(SoundParameter::FilterEnvFAmount),
            ))
            .push(slider_with_labels(
                "Track",
                &mut self.track_slider,
                SoundParameter::FilterTrack,
                params.get_value(SoundParameter::FilterTrack),
            ))
            .push(slider_with_labels(
                "After",
                &mut self.after_slider,
                SoundParameter::FilterAfter,
                params.get_value(SoundParameter::FilterAfter),
            ))
            .push(slider_with_labels(
                "LFO 1 Amt",
                &mut self.lfo1_amount_slider,
                SoundParameter::FilterLFO1Amount,
                params.get_value(SoundParameter::FilterLFO1Amount),
            ));
        Container::new(content).style(style::FilterSection).into()
    }
}
