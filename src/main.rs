mod messages;
mod midi;
mod params;
mod ui;

use iced::{
    executor, time, Application, Clipboard, Column, Command, Container, Element, Length, Row,
    Settings, Subscription,
};
use iced_native;
use log;
use simple_logger::SimpleLogger;

use messages::Message;
use midi::MidiConnector;
use params::{GetValue, MultiParameterValues, SoundParameterValues};
use ui::sound::{
    amp::AmpSection, arp::ArpSection, enva::EnvASection, envf::EnvFSection, extra::ExtraSection,
    filter::FilterSection, lfo1::LFO1Section, lfo2::LFO2Section, misc::MiscSection,
    modulation::ModSection, osc1::Osc1Section, osc2::Osc2Section, shaper::ShaperSection,
};
use ui::style;

fn main() -> iced::Result {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();
    let settings = Settings {
        window: iced::window::Settings {
            size: (style::WINDOW_WIDTH, style::WINDOW_HEIGHT),
            min_size: Some((style::WINDOW_WIDTH, style::WINDOW_HEIGHT)),
            resizable: true,
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
    mod_section: ModSection,

    // Current sound/multi parameter values
    sound_params: SoundParameterValues,
    multi_params: MultiParameterValues,

    // MIDI connection handler
    midi: MidiConnector,

    // Flag for preset update on next tick
    request_update: bool,

    // Exit flag
    should_exit: bool,
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
                mod_section: ModSection::new(),

                sound_params: SoundParameterValues::with_capacity(128),
                multi_params: MultiParameterValues::with_capacity(32),

                midi: MidiConnector::new(),

                request_update: true,
                should_exit: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Töörö Editor")
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::EventOccurred(event) => {
                if let iced_native::Event::Window(iced_native::window::Event::CloseRequested) =
                    event
                {
                    self.should_exit = true;
                }
            }
            Message::SoundParameterChange(param, value) => {
                let last_value = self.sound_params.get_value(param);
                if value != last_value {
                    self.sound_params.insert(param, value);
                    let message = midi::sysex::preset_param_dump(0x70, &param, value);
                    // log::info!("Sending preset parameter dump {:?}", message);
                    self.midi.send(&message);
                }
            }
            Message::Tick => {
                self.midi.scan();
            }
            Message::FastTick => {
                if let Some(message) = self.midi.receive() {
                    self.process_incoming_midi(&message);
                }
                if self.midi.is_connected() && self.request_update {
                    // let multi_id = 0x7F;
                    // log::info!("Requesting multi with id {:#X}", multi_id);
                    // let message = midi::sysex::multi_request(multi_id);
                    // self.midi.send(&message);
                    let preset_id = 0x70;
                    log::info!("Requesting preset with id {:#X}", preset_id);
                    let message = midi::sysex::preset_request(preset_id);
                    self.midi.send(&message);
                    self.request_update = false;
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let event_subscription = iced_native::subscription::events().map(Message::EventOccurred);

        let tick_subscription =
            time::every(std::time::Duration::from_millis(1000)).map(|_| Message::Tick);
        let fast_tick_subscription =
            time::every(std::time::Duration::from_millis(100)).map(|_| Message::FastTick);

        let subscriptions = vec![
            tick_subscription,
            fast_tick_subscription,
            event_subscription,
        ];

        Subscription::batch(subscriptions.into_iter())
    }

    fn should_exit(&self) -> bool {
        self.should_exit
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
            .push(self.arp_section.view(&self.sound_params))
            .width(Length::FillPortion(4));

        let row2_col2 = Column::new()
            .padding(5)
            .spacing(10)
            .push(self.lfo2_section.view(&self.sound_params))
            .push(self.misc_section.view(&self.sound_params))
            .width(Length::FillPortion(4));

        let row2_col3 = Column::new()
            .padding(5)
            .spacing(10)
            .push(self.envf_section.view(&self.sound_params))
            .width(Length::FillPortion(4));

        let row2_col4 = Column::new()
            .padding(5)
            .spacing(10)
            .push(self.enva_section.view(&self.sound_params))
            .width(Length::FillPortion(4));

        let row3_col1 = Column::new()
            .padding(5)
            .spacing(10)
            .push(self.mod_section.view(&self.sound_params))
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

        let row3 = Row::new().push(row3_col1);

        Container::new(Column::new().push(row1).push(row2).push(row3))
            .padding(10)
            .height(Length::Fill)
            .style(style::MainWindow)
            .into()
    }
}

impl EditorApp {
    fn process_incoming_midi(&mut self, message: &Vec<u8>) {
        match message[0] {
            0xB0 => {
                // Control change
                self.request_update = true;
            }
            0xF0 => {
                // Sysex
                match message[1] {
                    midi::sysex::SERVICE_PRESET_DUMP
                        if message.len() == midi::sysex::PRESET_DUMP_LENGTH =>
                    {
                        let preset_id = message[2];
                        log::info!("Preset dump received with id {:#X}", preset_id);
                        match preset_id {
                            0..=99 => {}
                            0x70..=0x73 => {
                                let part_no = preset_id - 0x70;
                                let param_values =
                                    midi::sysex::unpack_data(&message[3..message.len()]);
                                midi::sysex::update_sound_params(
                                    &mut self.sound_params,
                                    &param_values,
                                );
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
