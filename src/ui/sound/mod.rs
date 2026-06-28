//! Panel containing the sound (preset) parameters

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

use iced::widget::{column, container, row};
use iced::{Element, Length};

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

    pub fn view(&self, params: &SoundParameterValues) -> Element<'_, Message> {
        let sound_col1 = column![
            self.osc1_section.view(params),
            self.lfo1_section.view(params),
            self.arp_section.view(params)
        ]
        .spacing(10)
        .width(Length::FillPortion(4));

        let sound_col2 = column![
            self.osc2_section.view(params),
            self.lfo2_section.view(params),
            self.misc_section.view(params)
        ]
        .spacing(10)
        .width(Length::FillPortion(4));

        let sound_col3 = column![
            self.shaper_section.view(params),
            self.extra_section.view(params),
            self.envf_section.view(params)
        ]
        .spacing(10)
        .width(Length::FillPortion(4));

        let sound_col4 = column![
            self.filter_section.view(params),
            self.amp_section.view(params),
            self.enva_section.view(params)
        ]
        .spacing(10)
        .width(Length::FillPortion(4));

        container(
            column![
                row![sound_col1, sound_col2, sound_col3, sound_col4].spacing(10),
                container(self.mod_section.view(params))
            ]
            .spacing(10),
        )
        .height(Length::Fill)
        .into()
    }
}
