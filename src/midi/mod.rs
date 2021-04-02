pub mod cc;
pub mod sysex;

use std::sync::mpsc;

use log;
use midir::{MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};

pub struct MidiConnector {
    midi_out: Option<MidiOutputConnection>,
    midi_in: Option<MidiInputConnection<OnReceiveArgs>>,
    midi_in_mpsc_channel: Option<(mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>)>,
}

impl MidiConnector {
    pub fn new() -> Self {
        Self {
            midi_out: None,
            midi_in: None,
            midi_in_mpsc_channel: None,
        }
    }

    /// Scans the ports and establishes a connection to the device if found
    pub fn scan(&mut self) {
        match MidiOutput::new("midi scan output") {
            Ok(midi_out) => {
                let mut connected = false;
                for p in midi_out.ports().iter() {
                    let port_name = midi_out.port_name(p).unwrap();
                    if port_name.starts_with("Tooro") {
                        if self.midi_out.is_none() {
                            log::info!("MIDI out connected to port {}", port_name);
                            self.midi_out = Some(midi_out.connect(p, "tooro output").unwrap());
                        }
                        connected = true;
                        break;
                    }
                }
                if !connected && self.midi_out.is_some() {
                    log::info!("MIDI out disconnected");
                    self.midi_out = None;
                }
            }
            Err(error) => {
                log::error!("MIDI out error: {}", error);
            }
        }

        match MidiInput::new("midi scan input") {
            Ok(midi_in) => {
                let mut connected = false;
                for p in midi_in.ports().iter() {
                    let port_name = midi_in.port_name(p).unwrap();
                    if port_name.starts_with("Tooro") {
                        if self.midi_in.is_none() {
                            log::info!("MIDI in connected to port {}", port_name);
                            self.midi_in_mpsc_channel = Some(mpsc::channel());
                            let on_receive_args = OnReceiveArgs {
                                sender: Some(self.midi_in_mpsc_channel.as_ref().unwrap().0.clone()),
                            };
                            self.midi_in = Some(
                                midi_in
                                    .connect(p, "tooro input", on_receive, on_receive_args)
                                    .unwrap(),
                            );
                        }
                        connected = true;
                        break;
                    }
                }
                if !connected && self.midi_in.is_some() {
                    log::info!("MIDI in disconnected");
                    self.midi_in = None;
                    self.midi_in_mpsc_channel = None;
                }
            }
            Err(error) => {
                log::error!("MIDI in error: {}", error);
            }
        }
    }

    /// Sends a message
    pub fn send(&mut self, message: &[u8]) {
        match self.midi_out.as_mut() {
            Some(conn) => {
                conn.send(message).ok();
            }
            None => {}
        }
    }

    /// Receive a message
    pub fn receive(&mut self) -> Option<Vec<u8>> {
        if self.midi_in_mpsc_channel.is_none() {
            return None;
        }

        let receiver = &self.midi_in_mpsc_channel.as_ref().unwrap().1;
        let result = receiver.try_recv();

        if result.is_err() {
            return None;
        }

        Some(result.unwrap())
    }

    /// Returns if device is connected
    pub fn is_connected(&self) -> bool {
        self.midi_in.is_some() && self.midi_out.is_some()
    }

    /// Send a dump request for a preset
    ///
    /// - `preset_id`   Preset id, either 0..99 or 0x70..0x73
    pub fn request_preset_dump(&mut self, preset_id: u8) {
        let message = [0xF0, sysex::SERVICE_PRESET_REQUEST, preset_id, 0xF7];
        self.send(&message);
    }
}

/// Arguments for on_receive() callback function
struct OnReceiveArgs {
    sender: Option<mpsc::Sender<Vec<u8>>>,
}

/// Callback for received MIDI messages
fn on_receive(_timestamp: u64, message: &[u8], args: &mut OnReceiveArgs) {
    let message = Vec::<u8>::from(message);
    if args.sender.is_some() {
        args.sender.as_ref().unwrap().send(message).ok();
    }
}
