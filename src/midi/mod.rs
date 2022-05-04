//! Module containing all MIDI-related code

pub mod sysex;

use std::sync::mpsc;

use midir::{MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};

type MpscChannel = (mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>);

/// Container for connections and state
pub struct MidiConnector {
    /// Output connection to the device
    device_output: Option<MidiOutputConnection>,

    /// Input connection from the device
    device_input: Option<MidiInputConnection<OnReceiveArgs>>,

    /// MPSC channel to transfer incoming messages from callback to main thread
    device_input_mpsc_channel: MpscChannel,

    /// Objects used for port scanning
    scan_input: Option<MidiInput>,
    scan_output: Option<MidiOutput>,

    /// Vector of port names that are usable as merge inputs
    merge_inputs_list: Vec<String>,

    /// Merge input connection
    merge_input: Option<MidiInputConnection<OnReceiveArgs>>,
}

impl MidiConnector {
    /// Constructs a new instance
    pub fn new() -> Self {
        Self {
            device_output: None,
            device_input: None,
            device_input_mpsc_channel: mpsc::channel(),
            scan_input: None,
            scan_output: None,
            merge_inputs_list: Vec::new(),
            merge_input: None,
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
            let mut merge_inputs = Vec::new();
            let input = self.scan_input.as_ref().unwrap();

            for port in input.ports().iter() {
                let port_name = cleanup_port_name(input.port_name(port).unwrap());
                if !port_name.contains("Tooro") && !port_name.contains("tooro") {
                    merge_inputs.push(port_name);
                }
            }

            self.merge_inputs_list = merge_inputs;

            let mut connected = false;
            let input = self.scan_input.as_ref().unwrap();

            for port in input.ports().iter() {
                let port_name = cleanup_port_name(input.port_name(port).unwrap());
                if port_name.contains("Tooro") {
                    if self.device_input.is_none() {
                        log::info!("MIDI input connected to port {}", port_name);
                        let on_receive_args = OnReceiveArgs {
                            sender: Some(self.device_input_mpsc_channel.0.clone()),
                        };
                        self.device_input = Some(
                            self.scan_input
                                .take()
                                .unwrap()
                                .connect(port, "tooro input", on_receive, on_receive_args)
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
                let port_name = cleanup_port_name(output.port_name(port).unwrap());
                if port_name.contains("Tooro") {
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
        if let Some(conn) = self.device_output.as_mut() {
            conn.send(message).ok();
        }
    }

    /// Receives a message
    pub fn receive(&mut self) -> Option<Vec<u8>> {
        let receiver = &self.device_input_mpsc_channel.1;
        let result = receiver.try_recv();

        if result.is_err() {
            return None;
        }

        Some(result.unwrap())
    }

    /// Returns the device connection state
    pub fn is_connected(&self) -> bool {
        self.device_input.is_some() && self.device_output.is_some()
    }

    /// Return a vector of inputs that are suitable for merging
    pub fn get_merge_inputs(&self) -> &Vec<String> {
        &self.merge_inputs_list
    }

    /// Select the input for merging messages
    pub fn select_merge_input(&mut self, input_name: String, sender: mpsc::Sender<Vec<u8>>) {
        if self.merge_input.is_some() {
            self.merge_input = None;
        }

        if self.scan_input.is_none() {
            match MidiInput::new(&(env!("CARGO_PKG_NAME").to_owned() + " scan input")) {
                Ok(input) => {
                    self.scan_input = Some(input);
                }
                Err(error) => {
                    log::error!("MIDI scan input error: {}", error);
                    return;
                }
            }
        }

        let input = self.scan_input.as_ref().unwrap();

        for port in input.ports().iter() {
            let port_name = cleanup_port_name(input.port_name(port).unwrap());
            if port_name == input_name {
                log::info!("Merge MIDI input connected to port {}", port_name);
                let on_receive_args = OnReceiveArgs {
                    sender: Some(sender),
                };
                self.merge_input = Some(
                    self.scan_input
                        .take()
                        .unwrap()
                        .connect(port, "tooro merge input", on_receive, on_receive_args)
                        .unwrap(),
                );
                break;
            }
        }
    }
}

/// Arguments for on_receive() callback function
struct OnReceiveArgs {
    sender: Option<mpsc::Sender<Vec<u8>>>,
}

/// Callback for received messages from device or merge input
fn on_receive(_timestamp: u64, message: &[u8], args: &mut OnReceiveArgs) {
    if args.sender.is_some() {
        let message = Vec::<u8>::from(message);
        args.sender.as_ref().unwrap().send(message).ok();
    }
}

/// Remove client name part from port name on Linux
fn cleanup_port_name(port_name: String) -> String {
    #[cfg(target_os = "linux")]
    {
        if let Some((client_name, remainder)) = port_name.split_once(':') {
            if remainder.starts_with(client_name) {
                return remainder.to_owned();
            }
        }
        port_name
    }

    #[cfg(not(target_os = "linux"))]
    port_name
}
