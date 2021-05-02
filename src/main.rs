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
use ui::multi::MultiPanel;
use ui::sound::SoundPanel;
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
    // Panels
    sound_panel: SoundPanel,
    multi_panel: MultiPanel,

    // Current part id 0-3 for part 1-4
    part_id: u8,

    // Current sound/multi parameter values
    sound_params: SoundParameterValues,
    multi_params: MultiParameterValues,

    // MIDI connection handler
    midi: MidiConnector,

    // Flag for parameter update from device on next tick
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
                sound_panel: SoundPanel::new(),
                multi_panel: MultiPanel::new(),

                part_id: 0,

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
                    let message =
                        midi::sysex::preset_param_dump(0x70 + self.part_id, &param, value);
                    // log::info!("Sending preset parameter dump {:?}", message);
                    self.midi.send(&message);
                }
            }
            Message::MultiParameterChange(param, value) => {
                let last_value = self.multi_params.get_value(param);
                if value != last_value {
                    self.multi_params.insert(param, value);
                    // let message =
                    //    midi::sysex::preset_param_dump(0x70 + self.part_id, &param, value);
                    // log::info!("Sending preset parameter dump {:?}", message);
                    // self.midi.send(&message);
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
                    let preset_id = 0x70 + self.part_id;
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
        Container::new(
            Row::new()
                .push(
                    Column::new()
                        .push(self.sound_panel.view(&self.sound_params))
                        .width(Length::FillPortion(4)),
                )
                .push(
                    Column::new()
                        .push(self.multi_panel.view(&self.multi_params))
                        .width(Length::FillPortion(1)),
                ),
        )
        .padding(5)
        .height(Length::Fill)
        .style(style::MainWindow)
        .into()
    }
}

impl EditorApp {
    fn process_incoming_midi(&mut self, message: &Vec<u8>) {
        match message[0] {
            0xB0..=0xBF => {
                // Control change (all channels)
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
                                if self.part_id == preset_id - 0x70 {
                                    let param_values =
                                        midi::sysex::unpack_data(&message[3..message.len()]);
                                    midi::sysex::update_sound_params(
                                        &mut self.sound_params,
                                        &param_values,
                                    );
                                }
                            }
                            _ => {}
                        }
                    }
                    midi::sysex::SERVICE_MULTI_DUMP
                        if message.len() == midi::sysex::MULTI_DUMP_LENGTH =>
                    {
                        let multi_id = message[2];
                        log::info!("Multi dump received with id {:#X}", multi_id);
                        if multi_id == 0x7F {
                            let param_values = midi::sysex::unpack_data(&message[3..message.len()]);
                            midi::sysex::update_multi_params(&mut self.multi_params, &param_values);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
