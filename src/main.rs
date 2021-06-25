//! Cross-platform sound editor for the [Fred's Lab Töörö](https://fredslab.net/en/tooro-module.php) hardware synthesizer

mod messages;
mod midi;
mod params;
mod ui;

use std::time::{Duration, Instant};

use iced::{
    executor, pick_list, time, Align, Application, Clipboard, Column, Command, Container, Element,
    Length, PickList, Row, Settings, Subscription, Text,
};
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

/// The main entry point
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

/// Holds the application data and state
struct EditorApp {
    /// UI section containing the sound (preset) parameters
    sound_panel: SoundPanel,

    /// UI section containing the multi parameters
    multi_panel: MultiPanel,

    /// UI section containing global controls
    manager_panel: ManagerPanel,

    /// Status bar info if connection is enabled or disabled
    status_connection: String,

    /// Status bar info on communication
    status_communication: String,

    /// Drop down list for the MIDI merge input
    merge_input_list: pick_list::State<String>,

    /// Current part id 0-3 for part 1-4
    part_id: u8,

    /// Current sound (preset) parameter values
    sound_params: SoundParameterValues,

    /// Current multi parameter values
    multi_params: MultiParameterValues,

    /// MIDI connection handler for all ports
    midi: MidiConnector,

    /// Device connection state
    device_connected: bool,

    /// Flag for requested sound (preset) parameter update from device
    request_sound_update: bool,

    /// Flag for requested multi parameter update from device
    request_multi_update: bool,

    /// Time of last dump request
    request_time: Option<Instant>,

    /// File to capture next received preset dump
    preset_capture_file: Option<String>,

    /// Flag for app initialisation complete
    init_complete: bool,

    /// Application exit flag
    should_exit: bool,
}

impl Application for EditorApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    /// Constructs a new application
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                sound_panel: SoundPanel::new(),
                multi_panel: MultiPanel::new(),
                manager_panel: ManagerPanel::new(),

                status_connection: String::from("Device disconnected"),
                status_communication: String::from("Initializing..."),

                merge_input_list: pick_list::State::<String>::default(),

                part_id: 0,

                sound_params: SoundParameterValues::with_capacity(128),
                multi_params: MultiParameterValues::with_capacity(32),

                midi: MidiConnector::new(),
                device_connected: false,

                request_sound_update: false,
                request_multi_update: false,
                request_time: None,

                preset_capture_file: None,

                init_complete: false,
                should_exit: false,
            },
            Command::none(),
        )
    }

    /// Returns the name of the application shown in the title bar
    fn title(&self) -> String {
        format!("Töörö Editor")
    }

    /// Process a message and update the state accordingly
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
                    if self.device_connected {
                        let message =
                            midi::sysex::preset_param_dump(0x70 + self.part_id, &param, value);
                        // log::info!("Sending preset parameter dump {:?}", message);
                        self.midi.send(&message);
                    }
                }
            }

            Message::MultiParameterChange(param, value) => {
                let last_value = self.multi_params.get_value(param);

                if value != last_value {
                    self.multi_params.insert(param, value);
                    if self.device_connected {
                        let message = midi::sysex::multi_param_dump(&param, value);
                        // log::info!("Sending multi parameter dump {:?}", message);
                        self.midi.send(&message);
                    }
                }
            }

            Message::PartChange(part_id) => {
                self.part_id = part_id;
                self.request_sound_update = true;
            }

            Message::MergeInputChange(input_name) => {
                log::info!("Merge input changed to {:?}", input_name);
                self.midi.select_merge_input(input_name);
            }

            Message::UpdateFromDevice if self.device_connected => {
                self.request_sound_update = true;
                self.request_multi_update = true;
            }

            Message::LoadSysexFile if self.device_connected => {
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

            Message::SavePresetSysexFile if self.device_connected => {
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
                self.midi.scan_ports();
                let connection_state = self.midi.is_connected();

                if connection_state != self.device_connected {
                    if connection_state {
                        self.on_device_connected();
                    } else {
                        self.on_device_disconnected();
                    }
                    self.device_connected = connection_state;
                }

                if !self.init_complete {
                    log::info!("Init complete");
                    self.status_communication = if self.device_connected {
                        String::new()
                    } else {
                        String::from("MIDI communication disabled")
                    };
                    self.init_complete = true;
                }
            }

            Message::FastTick => {
                self.midi.update();

                if let Some(message) = self.midi.receive() {
                    self.process_midi(&message);
                }

                if self.device_connected {
                    if self.request_sound_update && self.request_time.is_none() {
                        let preset_id = 0x70 + self.part_id;
                        log::info!("Requesting preset with id {:#X}", preset_id);
                        self.status_communication = String::from("Requesting preset dump...");
                        let message = midi::sysex::preset_request(preset_id);
                        self.midi.send(&message);
                        self.request_time = Some(Instant::now());
                        self.request_sound_update = false;
                    }

                    if self.request_multi_update && self.request_time.is_none() {
                        let multi_id = 0x7F;
                        log::info!("Requesting multi with id {:#X}", multi_id);
                        self.status_communication = String::from("Requesting multi dump...");
                        let message = midi::sysex::multi_request(multi_id);
                        self.midi.send(&message);
                        self.request_time = Some(Instant::now());
                        self.request_multi_update = false;
                    }

                    if let Some(request_time) = self.request_time {
                        if request_time.elapsed() >= Duration::new(1, 0) {
                            log::error!("Response timeout");
                            self.status_communication = String::from("Error: response timeout");
                            self.request_time = None;
                        }
                    }
                }
            }

            _ => {}
        }

        Command::none()
    }

    /// Return a subscripton event
    fn subscription(&self) -> Subscription<Self::Message> {
        let event_subscription = iced_native::subscription::events().map(Message::EventOccurred);

        let tick_subscription = time::every(Duration::from_millis(1000)).map(|_| Message::Tick);
        let fast_tick_subscription =
            time::every(Duration::from_millis(10)).map(|_| Message::FastTick);

        let subscriptions = vec![
            tick_subscription,
            fast_tick_subscription,
            event_subscription,
        ];

        Subscription::batch(subscriptions.into_iter())
    }

    /// Return whether the application should exit
    fn should_exit(&self) -> bool {
        self.should_exit
    }

    /// Returns the widgets to display
    fn view(&mut self) -> Element<Self::Message> {
        Container::new(
            Column::new()
                .push(
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
                        )
                        .height(Length::Units(625)),
                )
                .push(
                    Row::new()
                        .push(Column::new().width(Length::Units(10)))
                        .push(
                            Column::new()
                                .push(
                                    Text::new(&self.status_connection)
                                        .size(style::STATUS_TEXT_SIZE),
                                )
                                .width(Length::FillPortion(1)),
                        )
                        .push(
                            Column::new()
                                .push(
                                    Row::new()
                                        .push(
                                            Text::new("Merge Input:").size(style::STATUS_TEXT_SIZE),
                                        )
                                        .push(
                                            PickList::new(
                                                &mut self.merge_input_list,
                                                {
                                                    let mut inputs =
                                                        self.midi.get_merge_inputs().clone();
                                                    inputs.insert(0, String::from(""));
                                                    inputs
                                                },
                                                Some(self.midi.get_merge_input_name()),
                                                move |v| Message::MergeInputChange(v),
                                            )
                                            .style(style::PickList)
                                            .text_size(style::LIST_ITEM_TEXT_SIZE),
                                        )
                                        .spacing(10),
                                )
                                .width(Length::FillPortion(1)),
                        )
                        .push(
                            Column::new()
                                .push(
                                    Text::new(&self.status_communication)
                                        .size(style::STATUS_TEXT_SIZE),
                                )
                                .width(Length::FillPortion(2))
                                .align_items(Align::Center),
                        )
                        .push(
                            Column::new()
                                .push(
                                    #[cfg(debug_assertions)]
                                    Text::new(format!(
                                        "v{} (debug build)",
                                        env!("CARGO_PKG_VERSION")
                                    ))
                                    .size(style::STATUS_TEXT_SIZE),
                                    #[cfg(not(debug_assertions))]
                                    Text::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                                        .size(style::STATUS_TEXT_SIZE),
                                )
                                .width(Length::FillPortion(1))
                                .align_items(Align::End),
                        )
                        .push(Column::new().width(Length::Units(10))),
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
        self.status_connection = String::from("Device connected");
        self.request_sound_update = true;
        self.request_multi_update = true;
    }

    /// Called when device is disconnected
    fn on_device_disconnected(&mut self) {
        log::info!("Device disconnected");
        self.status_connection = String::from("Device disconnected");
        self.request_sound_update = false;
        self.request_multi_update = false;
        self.request_time = None;
    }

    /// Process an incoming MIDI message from the device
    fn process_midi(&mut self, message: &Vec<u8>) {
        match message[0] {
            0xB0..=0xBF | 0xC0..=0xCF => {
                // Whenever the device sends a CC or program change message,
                // a full parameter update will be requested to keep editor in sync
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
    fn process_preset_dump(&mut self, message: &Vec<u8>) {
        let preset_id = message[2];

        log::info!("Preset dump received with id {:#X}", preset_id);
        self.status_communication = String::from("");

        // Wait a little bit because the dump is possibly echoed by the DAW
        std::thread::sleep(Duration::from_millis(100));

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

        self.request_time = None;
    }

    /// Process an incoming multi dump from the device
    fn process_multi_dump(&mut self, message: &Vec<u8>) {
        let multi_id = message[2];

        log::info!("Multi dump received with id {:#X}", multi_id);
        self.status_communication = String::from("");

        // Wait a little bit because the dump is possibly echoed by the DAW
        std::thread::sleep(Duration::from_millis(100));

        if multi_id == 0x7F {
            let param_values = midi::sysex::unpack_data(&message[3..message.len()]);
            midi::sysex::update_multi_params(&mut self.multi_params, &param_values);
        }

        self.request_time = None;
    }
}
