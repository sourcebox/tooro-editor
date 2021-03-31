mod elements;
mod messages;
mod midi;
mod params;
mod sections;
mod style;

use std::hash::{Hash, Hasher};
use std::sync::mpsc;

use iced::{
    executor, time, Application, Clipboard, Column, Command, Container, Element, Length, Row,
    Settings, Subscription,
};
use iced_futures::futures;
use iced_native;

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
                    if let Some(cc) = midi::cc::sound_param_to_cc(&param, value) {
                        let (cc_num, cc_value) = cc;
                        self.midi.send(&[0xB0, cc_num, cc_value]);
                    }
                }
            }
            Message::MidiReceived(data) => {
                self.process_incoming_midi(&data);
            }
            Message::Tick => {
                self.midi.scan();
                if self.midi.is_connected() {
                    let message = [0xF0, midi::sysex::SERVICE_PRESET_REQUEST, 0x70, 0xF7];
                    self.midi.send(&message);
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let tick_subscription =
            time::every(std::time::Duration::from_millis(1000)).map(|_| Message::Tick);

        let (sender, receiver) = mpsc::channel();
        self.midi.set_midi_in_sender(&sender);
        let midi_subscription =
            Subscription::from_recipe(MidiReceiveSubscription { receiver: receiver })
                .map(|data| Message::MidiReceived(data));

        let subscriptions = vec![tick_subscription, midi_subscription];

        Subscription::batch(subscriptions.into_iter())
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

impl EditorApp {
    fn process_incoming_midi(&mut self, message: &Vec<u8>) {
        match message[0] {
            0xB0 => {
                // Control change
                let sound_param = midi::cc::cc_to_sound_param(message[1], message[2]);
                if sound_param.is_some() {
                    let (param, value) = sound_param.unwrap();
                    // self.sound_params.insert(param, value);
                }
            }
            0xF0 => {
                // Sysex
                match message[1] {
                    midi::sysex::SERVICE_PRESET_DUMP => {
                        let preset_id = message[2];
                        println!("Preset dump");
                        match preset_id {
                            0..=99 => {
                                println!("Preset {}", preset_id);
                            }
                            0x70..=0x73 => {
                                let part_no = preset_id - 0x70;
                                println!("Part {}", part_no + 1);
                                let param_values =
                                    midi::sysex::unpack_data(&message[3..message.len()]);
                                midi::sysex::update_sound_params(
                                    &mut self.sound_params,
                                    &param_values,
                                );
                            }
                            _ => {
                                println!("Unknown id {}", preset_id);
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

pub struct MidiReceiveSubscription {
    receiver: mpsc::Receiver<Vec<u8>>,
}

impl<H, I> iced_native::subscription::Recipe<H, I> for MidiReceiveSubscription
where
    H: Hasher,
{
    type Output = Vec<u8>;

    fn hash(&self, state: &mut H) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        Box::pin(futures::stream::unfold(
            self.receiver,
            move |state| async move {
                let receiver = &state;
                let result = receiver.recv();
                if result.is_ok() {
                    Some((result.unwrap(), state))
                } else {
                    None
                }
            },
        ))
    }
}
