//! Module containing all MIDI-related code

pub mod sysex;

use std::sync::mpsc;

use log;
use midir::{MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};

/// Container for connections and state
pub struct MidiConnector {
    /// MIDI output connection to the device
    midi_out: Option<MidiOutputConnection>,

    /// MIDI input connection from the device
    midi_in: Option<MidiInputConnection<OnReceiveArgs>>,

    /// MPSC channel to transfer incoming messages from callback to main thread
    midi_in_mpsc_channel: Option<(mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>)>,
}

impl MidiConnector {
    /// Constructs a new instance
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
}

/// Arguments for on_receive() callback function
struct OnReceiveArgs {
    sender: Option<mpsc::Sender<Vec<u8>>>,
}

/// Callback for received MIDI messages
fn on_receive(_timestamp: u64, message: &[u8], args: &mut OnReceiveArgs) {
    if args.sender.is_some() {
        let message = Vec::<u8>::from(message);
        args.sender.as_ref().unwrap().send(message).ok();
    }
}
