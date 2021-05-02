mod amp;
mod arp;
mod enva;
mod envf;
mod extra;
mod filter;
mod lfo1;
mod lfo2;
mod misc;
mod modulation;
mod osc1;
mod osc2;
mod shaper;

use iced::{Column, Container, Element, Length, Row};

use super::style;
use crate::messages::Message;
use crate::params::SoundParameterValues;
use {
    amp::AmpSection, arp::ArpSection, enva::EnvASection, envf::EnvFSection, extra::ExtraSection,
    filter::FilterSection, lfo1::LFO1Section, lfo2::LFO2Section, misc::MiscSection,
    modulation::ModSection, osc1::Osc1Section, osc2::Osc2Section, shaper::ShaperSection,
};

pub struct SoundPanel {
    osc1_section: Osc1Section,
    osc2_section: Osc2Section,
    extra_section: ExtraSection,
    shaper_section: ShaperSection,
    filter_section: FilterSection,
    amp_section: AmpSection,
    lfo1_section: LFO1Section,
    lfo2_section: LFO2Section,
    envf_section: EnvFSection,
    enva_section: EnvASection,
    arp_section: ArpSection,
    misc_section: MiscSection,
    mod_section: ModSection,
}

impl SoundPanel {
    pub fn new() -> Self {
        Self {
            osc1_section: Osc1Section::new(),
            osc2_section: Osc2Section::new(),
            extra_section: ExtraSection::new(),
            shaper_section: ShaperSection::new(),
            filter_section: FilterSection::new(),
            amp_section: AmpSection::new(),
            lfo1_section: LFO1Section::new(),
            lfo2_section: LFO2Section::new(),
            envf_section: EnvFSection::new(),
            enva_section: EnvASection::new(),
            arp_section: ArpSection::new(),
            misc_section: MiscSection::new(),
            mod_section: ModSection::new(),
        }
    }

    pub fn view(&mut self, params: &SoundParameterValues) -> Element<Message> {
        let sound_col1 = Column::new()
            .padding(5)
            .spacing(10)
            .push(self.osc1_section.view(params))
            .push(self.lfo1_section.view(params))
            .push(self.arp_section.view(params))
            .width(Length::FillPortion(4));

        let sound_col2 = Column::new()
            .padding(5)
            .spacing(10)
            .push(self.osc2_section.view(params))
            .push(self.lfo2_section.view(params))
            .push(self.misc_section.view(params))
            .width(Length::FillPortion(4));

        let sound_col3 = Column::new()
            .padding(5)
            .spacing(10)
            .push(self.extra_section.view(params))
            .push(self.shaper_section.view(params))
            .push(self.envf_section.view(params))
            .width(Length::FillPortion(4));

        let sound_col4 = Column::new()
            .padding(5)
            .spacing(10)
            .push(self.filter_section.view(params))
            .push(self.amp_section.view(params))
            .push(self.enva_section.view(params))
            .width(Length::FillPortion(4));

        Container::new(
            Column::new()
                .push(
                    Row::new()
                        .push(sound_col1)
                        .push(sound_col2)
                        .push(sound_col3)
                        .push(sound_col4),
                )
                .push(Row::new().padding(5).push(self.mod_section.view(params))),
        )
        .padding(5)
        .height(Length::Fill)
        .style(style::MainWindow)
        .into()
    }
}
