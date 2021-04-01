pub mod cc;
pub mod sysex;

use std::cell::Cell;
use std::sync::mpsc;

use midir::{MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};

pub struct MidiConnector {
    midi_out: Option<MidiOutputConnection>,
    midi_in: Option<MidiInputConnection<OnReceiveArgs>>,
    midi_in_mpsc_sender: Cell<Option<mpsc::Sender<Vec<u8>>>>,
}

impl MidiConnector {
    pub fn new() -> Self {
        Self {
            midi_out: None,
            midi_in: None,
            midi_in_mpsc_sender: Cell::new(None),
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
                            println!("Device MIDI out connected");
                            self.midi_out = Some(midi_out.connect(p, "tooro output").unwrap());
                        }
                        connected = true;
                        break;
                    }
                }
                if !connected {
                    self.midi_out = None;
                }
            }
            Err(error) => {
                println!("MIDI out error {}", error);
            }
        }

        match MidiInput::new("midi scan input") {
            Ok(midi_in) => {
                let mut connected = false;
                for p in midi_in.ports().iter() {
                    let port_name = midi_in.port_name(p).unwrap();
                    if port_name.starts_with("Tooro") {
                        if self.midi_in.is_none() {
                            println!("Device MIDI in connected");
                            let on_receive_args = OnReceiveArgs {
                                sender: self.midi_in_mpsc_sender.take(),
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
                if !connected {
                    self.midi_in = None;
                }
            }
            Err(error) => {
                println!("MIDI in error {}", error);
            }
        }
    }

    /// Sets the mpsc sender for the received messages
    pub fn set_midi_in_sender(&self, sender: &mpsc::Sender<Vec<u8>>) {
        self.midi_in_mpsc_sender.set(Some(sender.clone()));
    }

    /// Sends a raw message
    pub fn send(&mut self, message: &[u8]) {
        println!("MIDI out {:?}", message);
        match self.midi_out.as_mut() {
            Some(conn) => {
                conn.send(message).ok();
            }
            None => {}
        }
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
    println!("MIDI in {:?}", message);
    if args.sender.is_some() {
        args.sender.as_ref().unwrap().send(message).ok();
    }
}
