//! Module containing all MIDI-related code

pub mod sysex;

use std::sync::mpsc;

use log;
use midir::{MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};

/// Container for connections and state
pub struct MidiConnector {
    /// Output connection to the device
    device_output: Option<MidiOutputConnection>,

    /// Input connection from the device
    device_input: Option<MidiInputConnection<OnReceiveArgs>>,

    /// MPSC channel to transfer incoming messages from callback to main thread
    device_input_mpsc_channel: Option<(mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>)>,

    /// Objects used for port scanning
    scan_input: Option<MidiInput>,
    scan_output: Option<MidiOutput>,
}

impl MidiConnector {
    /// Constructs a new instance
    pub fn new() -> Self {
        Self {
            device_output: None,
            device_input: None,
            device_input_mpsc_channel: None,
            scan_input: None,
            scan_output: None,
        }
    }

    /// Scans the ports and establishes a connection to the device if found
    pub fn scan_ports(&mut self) {
        if self.scan_input.is_none() {
            match MidiInput::new(&(env!("CARGO_PKG_NAME").to_owned() + " scan input")) {
                Ok(input) => {
                    self.scan_input = Some(input);
                }
                Err(error) => {
                    log::error!("MIDI scan input error: {}", error);
                }
            }
        }

        if self.scan_input.is_some() {
            let mut connected = false;
            let input = self.scan_input.as_ref().unwrap();

            for port in input.ports().iter() {
                let port_name = input.port_name(port).unwrap();
                if port_name.starts_with("Tooro") {
                    if self.device_input.is_none() {
                        log::info!("MIDI input connected to port {}", port_name);
                        self.device_input_mpsc_channel = Some(mpsc::channel());
                        let on_receive_args = OnReceiveArgs {
                            sender: Some(
                                self.device_input_mpsc_channel.as_ref().unwrap().0.clone(),
                            ),
                        };
                        self.device_input = Some(
                            self.scan_input
                                .take()
                                .unwrap()
                                .connect(port, "tooro input", on_device_receive, on_receive_args)
                                .unwrap(),
                        );
                    }
                    connected = true;
                    break;
                }
            }

            if !connected && self.device_input.is_some() {
                log::info!("MIDI input disconnected");
                self.device_input = None;
            }
        }

        if self.scan_output.is_none() {
            match MidiOutput::new(&(env!("CARGO_PKG_NAME").to_owned() + " scan output")) {
                Ok(output) => {
                    self.scan_output = Some(output);
                }
                Err(error) => {
                    log::error!("MIDI scan output error: {}", error);
                }
            }
        }

        if self.scan_output.is_some() {
            let mut connected = false;
            let output = self.scan_output.as_ref().unwrap();

            for port in output.ports().iter() {
                let port_name = output.port_name(port).unwrap();
                if port_name.starts_with("Tooro") {
                    if self.device_output.is_none() {
                        log::info!("MIDI output connected to port {}", port_name);
                        self.device_output = Some(
                            self.scan_output
                                .take()
                                .unwrap()
                                .connect(port, "tooro output")
                                .unwrap(),
                        );
                    }
                    connected = true;
                    break;
                }
            }

            if !connected && self.device_output.is_some() {
                log::info!("MIDI output disconnected");
                self.device_output = None;
            }
        }
    }

    /// Sends a message
    pub fn send(&mut self, message: &[u8]) {
        match self.device_output.as_mut() {
            Some(conn) => {
                conn.send(message).ok();
            }
            None => {}
        }
    }

    /// Receives a message
    pub fn receive(&mut self) -> Option<Vec<u8>> {
        if self.device_input_mpsc_channel.is_none() {
            return None;
        }

        let receiver = &self.device_input_mpsc_channel.as_ref().unwrap().1;
        let result = receiver.try_recv();

        if result.is_err() {
            return None;
        }

        Some(result.unwrap())
    }

    /// Returns if device is connected
    pub fn is_connected(&self) -> bool {
        self.device_input.is_some() && self.device_output.is_some()
    }
}

/// Arguments for on_receive() callback function
struct OnReceiveArgs {
    sender: Option<mpsc::Sender<Vec<u8>>>,
}

/// Callback for received messages from device
fn on_device_receive(_timestamp: u64, message: &[u8], args: &mut OnReceiveArgs) {
    if args.sender.is_some() {
        let message = Vec::<u8>::from(message);
        args.sender.as_ref().unwrap().send(message).ok();
    }
}
