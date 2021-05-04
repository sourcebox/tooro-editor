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
use tinyfiledialogs::{open_file_dialog, save_file_dialog_with_filter};

use messages::Message;
use midi::MidiConnector;
use params::{GetValue, MultiParameterValues, SoundParameterValues};
use ui::manager::ManagerPanel;
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
    manager_panel: ManagerPanel,

    // Current part id 0-3 for part 1-4
    part_id: u8,

    // Current sound/multi parameter values
    sound_params: SoundParameterValues,
    multi_params: MultiParameterValues,

    // MIDI connection handler
    midi: MidiConnector,

    // Device connection state
    device_connected: bool,

    // Flag set after a dump request was sent
    waiting_for_dump: bool,

    // Flags for parameter update from device
    request_sound_update: bool,
    request_multi_update: bool,

    // File to capture next received preset dump
    preset_capture_file: Option<String>,

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
                manager_panel: ManagerPanel::new(),

                part_id: 0,

                sound_params: SoundParameterValues::with_capacity(128),
                multi_params: MultiParameterValues::with_capacity(32),

                midi: MidiConnector::new(),
                device_connected: false,

                waiting_for_dump: false,
                request_sound_update: false,
                request_multi_update: false,

                preset_capture_file: None,

                should_exit: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("Töörö Editor v{}", env!("CARGO_PKG_VERSION"))
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
                    let message = midi::sysex::multi_dump(0x7F, &self.multi_params);
                    // log::info!("Sending multi dump {:?}", message);
                    self.midi.send(&message);
                }
            }

            Message::PartChange(part_id) => {
                self.part_id = part_id;
                self.request_sound_update = true;
            }

            Message::UpdateFromDevice => {
                self.request_sound_update = true;
                self.request_multi_update = true;
            }

            Message::LoadSysexFile => {
                match open_file_dialog("Open syx file", "", Some((&["*.syx"], "Sysex files"))) {
                    Some(file) => {
                        log::info!("Loading file {}", file);
                        let data = std::fs::read(file);

                        if data.is_ok() {
                            let mut message = data.unwrap();

                            match message[1] {
                                midi::sysex::SERVICE_PRESET_DUMP
                                    if message.len() == midi::sysex::PRESET_DUMP_LENGTH =>
                                {
                                    let preset_id = 0x70 + self.part_id;
                                    log::info!("Sending preset dump with id {:#X}", preset_id);
                                    message[2] = preset_id;
                                    self.midi.send(&message);
                                    self.request_sound_update = true;
                                }

                                midi::sysex::SERVICE_MULTI_DUMP
                                    if message.len() == midi::sysex::MULTI_DUMP_LENGTH =>
                                {
                                    let multi_id = 0x7F;
                                    log::info!("Sending multi dump with id {:#X}", multi_id);
                                    message[2] = multi_id;
                                    self.midi.send(&message);
                                    self.request_multi_update = true;
                                }

                                _ => {
                                    log::info!("Invalid data");
                                }
                            }
                        }
                    }
                    None => {}
                }
            }

            Message::SavePresetSysexFile => {
                match save_file_dialog_with_filter("Save syx file", "", &["*.syx"], "Sysex files") {
                    Some(file) => {
                        let mut file = std::path::PathBuf::from(file);
                        file.set_extension("syx");
                        log::info!("Capturing next preset dump in file {:?}", file);
                        self.preset_capture_file =
                            Some(file.into_os_string().into_string().unwrap());
                        self.request_sound_update = true;
                    }
                    None => {}
                }
            }

            Message::Tick => {
                self.midi.scan();
                let connection_state = self.midi.is_connected();

                if connection_state != self.device_connected {
                    if connection_state {
                        self.on_device_connected();
                    } else {
                        self.on_device_disconnected();
                    }
                    self.device_connected = connection_state;
                }
            }

            Message::FastTick => {
                if let Some(message) = self.midi.receive() {
                    self.process_midi(&message);
                }

                if self.device_connected {
                    if self.request_sound_update && !self.waiting_for_dump {
                        let preset_id = 0x70 + self.part_id;
                        log::info!("Requesting preset with id {:#X}", preset_id);
                        let message = midi::sysex::preset_request(preset_id);
                        self.midi.send(&message);
                        self.waiting_for_dump = true;
                        self.request_sound_update = false;
                    }

                    if self.request_multi_update && !self.waiting_for_dump {
                        let multi_id = 0x7F;
                        log::info!("Requesting multi with id {:#X}", multi_id);
                        let message = midi::sysex::multi_request(multi_id);
                        self.midi.send(&message);
                        self.waiting_for_dump = true;
                        self.request_multi_update = false;
                    }
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
                        .push(self.manager_panel.view(self.part_id))
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
    /// Called when device is connected
    fn on_device_connected(&mut self) {
        log::info!("Device connected");
        self.request_sound_update = true;
        self.request_multi_update = true;
    }

    /// Called when device is disconnected
    fn on_device_disconnected(&mut self) {
        log::info!("Device disconnected");
        self.request_sound_update = false;
        self.request_multi_update = false;
        self.waiting_for_dump = false;
    }

    /// Process an incoming MIDI message from the device
    ///
    /// - `message` MIDI message
    fn process_midi(&mut self, message: &Vec<u8>) {
        match message[0] {
            0xB0..=0xBF => {
                // Control change (all channels)
                self.request_sound_update = true;
                self.request_multi_update = true;
            }

            0xF0 => {
                // Sysex
                match message[1] {
                    midi::sysex::SERVICE_PRESET_DUMP
                        if message.len() == midi::sysex::PRESET_DUMP_LENGTH =>
                    {
                        self.process_preset_dump(&message);
                    }
                    midi::sysex::SERVICE_MULTI_DUMP
                        if message.len() == midi::sysex::MULTI_DUMP_LENGTH =>
                    {
                        self.process_multi_dump(&message);
                    }
                    _ => {}
                }
            }

            _ => {}
        }
    }

    /// Process an incoming preset dump from the device
    ///
    /// - `message` MIDI message containing dump
    fn process_preset_dump(&mut self, message: &Vec<u8>) {
        let preset_id = message[2];

        log::info!("Preset dump received with id {:#X}", preset_id);

        match preset_id {
            0..=99 => {}
            0x70..=0x73 => {
                if self.part_id == preset_id - 0x70 {
                    let param_values = midi::sysex::unpack_data(&message[3..message.len()]);
                    midi::sysex::update_sound_params(&mut self.sound_params, &param_values);
                    if let Some(file) = &self.preset_capture_file {
                        log::info!("Preset dump captured in file {}", file);
                        let mut message = message.clone();
                        message[2] = 0x70;
                        std::fs::write(file, message).ok();
                        self.preset_capture_file = None;
                    }
                }
            }
            _ => {}
        }

        self.waiting_for_dump = false;
    }

    /// Process an incoming multi dump from the device
    ///
    /// - `message` MIDI message containing dump
    fn process_multi_dump(&mut self, message: &Vec<u8>) {
        let multi_id = message[2];

        log::info!("Multi dump received with id {:#X}", multi_id);

        if multi_id == 0x7F {
            let param_values = midi::sysex::unpack_data(&message[3..message.len()]);
            midi::sysex::update_multi_params(&mut self.multi_params, &param_values);
        }

        self.waiting_for_dump = false;
    }
}
