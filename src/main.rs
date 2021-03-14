mod elements;
mod messages;
mod midi;
mod params;
mod sections;
mod style;

use iced::{
    executor, time, Application, Clipboard, Column, Command, Container, Element, Length, Row,
    Settings, Subscription,
};

use messages::Message;
use midi::MidiConnector;
use params::{GetValue, SoundParameterValues};
use sections::{
    amp::AmpSection, arp::ArpSection, enva::EnvASection, envf::EnvFSection, extra::ExtraSection,
    filter::FilterSection, lfo1::LFO1Section, lfo2::LFO2Section, misc::MiscSection,
    osc1::Osc1Section, osc2::Osc2Section, shaper::ShaperSection,
};

pub fn main() -> iced::Result {
    let settings = Settings {
        window: iced::window::Settings {
            min_size: Some((800, 700)),
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    };
    EditorApp::run(settings)
}

struct EditorApp {
    // Sections
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

    // Current sound parameter values
    sound_params: SoundParameterValues,

    // MIDI connection handler
    midi: MidiConnector,
}

impl Application for EditorApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
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

                sound_params: SoundParameterValues::with_capacity(128),

                midi: MidiConnector::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Töörö Editor")
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Message> {
        // println!("{:?}", message);
        match message {
            Message::SoundParameterChange(param, value) => {
                let last_value = self.sound_params.get_value(param);
                if value != last_value {
                    self.sound_params.insert(param, value);
                    self.midi.send_sound_param(0, &param, value);
                }
            }
            Message::Tick(_local_time) => {
                self.midi.scan();
            }
            _ => {}
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_millis(1000))
            .map(|_| Message::Tick(chrono::Local::now()))
    }

    fn view(&mut self) -> Element<Self::Message> {
        let row1_col1 = Column::new()
            .padding(5)
            .push(self.osc1_section.view(&self.sound_params))
            .width(Length::FillPortion(4));

        let row1_col2 = Column::new()
            .padding(5)
            .push(self.osc2_section.view(&self.sound_params))
            .width(Length::FillPortion(4));

        let row1_col3 = Column::new()
            .padding(5)
            .spacing(10)
            .push(self.extra_section.view(&self.sound_params))
            .push(self.shaper_section.view(&self.sound_params))
            .width(Length::FillPortion(4));

        let row1_col4 = Column::new()
            .padding(5)
            .spacing(10)
            .push(self.filter_section.view(&self.sound_params))
            .push(self.amp_section.view(&self.sound_params))
            .width(Length::FillPortion(4));

        let row2_col1 = Column::new()
            .padding(5)
            .spacing(10)
            .push(self.lfo1_section.view(&self.sound_params))
            .push(self.lfo2_section.view(&self.sound_params))
            .width(Length::FillPortion(4));

        let row2_col2 = Column::new()
            .padding(5)
            .spacing(10)
            .push(self.envf_section.view(&self.sound_params))
            .width(Length::FillPortion(4));

        let row2_col3 = Column::new()
            .padding(5)
            .spacing(10)
            .push(self.enva_section.view(&self.sound_params))
            .width(Length::FillPortion(4));

        let row2_col4 = Column::new()
            .padding(5)
            .spacing(10)
            .push(self.arp_section.view(&self.sound_params))
            .push(self.misc_section.view(&self.sound_params))
            .width(Length::FillPortion(4));

        let row1 = Row::new()
            .push(row1_col1)
            .push(row1_col2)
            .push(row1_col3)
            .push(row1_col4);

        let row2 = Row::new()
            .push(row2_col1)
            .push(row2_col2)
            .push(row2_col3)
            .push(row2_col4);

        Container::new(Column::new().push(row1).push(row2))
            .padding(10)
            .height(Length::Fill)
            .style(style::MainWindow)
            .into()
    }
}
