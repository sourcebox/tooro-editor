use std::cell::Cell;
use std::sync::mpsc;

use midir::{MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};

use crate::params::SoundParameter;

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

    /// Sends a sound parameter via CC
    pub fn send_sound_param(&mut self, channel: u8, param: &SoundParameter, value: i32) {
        let cc_num = match param {
            // Osc 1
            SoundParameter::Osc1Wave => Some(70),
            SoundParameter::Osc1Coarse => Some(21),
            SoundParameter::Osc1FMAmount => Some(22),
            SoundParameter::Osc1Level => Some(23),
            SoundParameter::Osc1Table => Some(24),
            SoundParameter::Osc1Fine => Some(25),
            SoundParameter::Osc1FMRate => Some(26),
            SoundParameter::Osc1Sync => Some(27),

            // Osc 2
            SoundParameter::Osc2Wave => Some(77),
            SoundParameter::Osc2Coarse => Some(31),
            SoundParameter::Osc2FMAmount => Some(32),
            SoundParameter::Osc2Level => Some(33),
            SoundParameter::Osc2Table => Some(34),
            SoundParameter::Osc2Fine => Some(35),
            SoundParameter::Osc2FMRate => Some(36),
            SoundParameter::Osc2Sync => Some(37),

            // Extra
            SoundParameter::ExtraNoise => Some(78),
            SoundParameter::ExtraRingMod => Some(79),

            // Filter
            SoundParameter::FilterCutoff => Some(74),
            SoundParameter::FilterResonance => Some(71),
            SoundParameter::FilterEnvFAmount => Some(54),
            SoundParameter::FilterTrack => Some(55),
            SoundParameter::FilterAfter => Some(56),
            SoundParameter::FilterLFO1Amount => Some(57),

            // Shaper
            SoundParameter::ShaperCutoff => Some(75),
            SoundParameter::ShaperResonance => Some(76),
            SoundParameter::ShaperEnvAAmount => Some(58),
            SoundParameter::ShaperTrack => Some(59),
            SoundParameter::ShaperMode => Some(60),
            SoundParameter::ShaperLFO2Amount => Some(61),

            // Env F
            SoundParameter::EnvFAttack => Some(46),
            SoundParameter::EnvFDecay => Some(47),
            SoundParameter::EnvFSustain => Some(48),
            SoundParameter::EnvFRelease => Some(49),
            SoundParameter::EnvFVelo => Some(51),
            SoundParameter::EnvFHold => Some(50),
            SoundParameter::EnvFAfter => Some(52),
            SoundParameter::EnvFTrigger => Some(53),

            // Env A
            SoundParameter::EnvAAttack => Some(73),
            SoundParameter::EnvADecay => Some(40),
            SoundParameter::EnvASustain => Some(41),
            SoundParameter::EnvARelease => Some(72),
            SoundParameter::EnvAVelo => Some(43),
            SoundParameter::EnvAHold => Some(42),
            SoundParameter::EnvAAfter => Some(44),
            SoundParameter::EnvATrigger => Some(45),

            // LFO 1
            SoundParameter::LFO1Shape => Some(102),
            SoundParameter::LFO1Speed => Some(103),
            SoundParameter::LFO1Rise => Some(104),
            SoundParameter::LFO1Phase => Some(105),

            // LFO 2
            SoundParameter::LFO2Shape => Some(106),
            SoundParameter::LFO2Speed => Some(107),
            SoundParameter::LFO2Rise => Some(108),
            SoundParameter::LFO2Phase => Some(109),

            // Arpeggiator
            SoundParameter::ArpMode => Some(110),
            SoundParameter::ArpGrid => Some(111),
            SoundParameter::ArpTempo => Some(112),
            SoundParameter::ArpHold => Some(113),

            // Amplifier
            SoundParameter::AmpLevel => Some(80),
            SoundParameter::AmpPan => Some(81),

            // Modulations
            SoundParameter::ModEnvF => Some(17),
            SoundParameter::ModEnvA => Some(18),
            SoundParameter::ModLFO1 => Some(19),
            SoundParameter::ModLFO2 => Some(20),
            SoundParameter::ModModwheel => Some(87),
            SoundParameter::ModPitchbend => Some(88),
            SoundParameter::ModVelocity => Some(89),
            SoundParameter::ModAftertouch => Some(90),

            // Misc
            SoundParameter::BendRange => Some(84),
            SoundParameter::Tune => Some(85),
        };

        if cc_num.is_none() {
            return;
        }

        let cc_value = (match param {
            // Default
            _ => {
                let range = param.get_range();
                rescale(value, *range.start(), *range.end(), 0, 127)
            }
        } as u8)
            .clamp(0, 127);

        self.send(&[0xB0 | channel, cc_num.unwrap(), cc_value]);
    }

    pub fn cc_to_sound_param(&self, cc_num: u8, cc_value: u8) -> Option<(SoundParameter, i32)> {
        let param = match cc_num {
            // Osc 1
            70 => Some(SoundParameter::Osc1Wave),
            21 => Some(SoundParameter::Osc1Coarse),
            22 => Some(SoundParameter::Osc1FMAmount),
            23 => Some(SoundParameter::Osc1Level),
            24 => Some(SoundParameter::Osc1Table),
            25 => Some(SoundParameter::Osc1Fine),
            26 => Some(SoundParameter::Osc1FMRate),
            27 => Some(SoundParameter::Osc1Sync),

            // Osc 2
            77 => Some(SoundParameter::Osc2Wave),
            31 => Some(SoundParameter::Osc2Coarse),
            32 => Some(SoundParameter::Osc2FMAmount),
            33 => Some(SoundParameter::Osc2Level),
            34 => Some(SoundParameter::Osc2Table),
            35 => Some(SoundParameter::Osc2Fine),
            36 => Some(SoundParameter::Osc2FMRate),
            37 => Some(SoundParameter::Osc2Sync),

            // Extra
            78 => Some(SoundParameter::ExtraNoise),
            79 => Some(SoundParameter::ExtraRingMod),

            // Filter
            74 => Some(SoundParameter::FilterCutoff),
            71 => Some(SoundParameter::FilterResonance),
            54 => Some(SoundParameter::FilterEnvFAmount),
            55 => Some(SoundParameter::FilterTrack),
            56 => Some(SoundParameter::FilterAfter),
            57 => Some(SoundParameter::FilterLFO1Amount),

            // Shaper
            75 => Some(SoundParameter::ShaperCutoff),
            76 => Some(SoundParameter::ShaperResonance),
            58 => Some(SoundParameter::ShaperEnvAAmount),
            59 => Some(SoundParameter::ShaperTrack),
            60 => Some(SoundParameter::ShaperMode),
            61 => Some(SoundParameter::ShaperLFO2Amount),

            // Env F
            46 => Some(SoundParameter::EnvFAttack),
            47 => Some(SoundParameter::EnvFDecay),
            48 => Some(SoundParameter::EnvFSustain),
            49 => Some(SoundParameter::EnvFRelease),
            50 => Some(SoundParameter::EnvFVelo),
            51 => Some(SoundParameter::EnvFHold),
            52 => Some(SoundParameter::EnvFAfter),
            53 => Some(SoundParameter::EnvFTrigger),

            // Env A
            73 => Some(SoundParameter::EnvAAttack),
            40 => Some(SoundParameter::EnvADecay),
            41 => Some(SoundParameter::EnvASustain),
            72 => Some(SoundParameter::EnvARelease),
            43 => Some(SoundParameter::EnvAVelo),
            42 => Some(SoundParameter::EnvAHold),
            44 => Some(SoundParameter::EnvAAfter),
            45 => Some(SoundParameter::EnvATrigger),

            // LFO 1
            102 => Some(SoundParameter::LFO1Shape),
            103 => Some(SoundParameter::LFO1Speed),
            104 => Some(SoundParameter::LFO1Rise),
            105 => Some(SoundParameter::LFO1Phase),

            // LFO 2
            106 => Some(SoundParameter::LFO2Shape),
            107 => Some(SoundParameter::LFO2Speed),
            108 => Some(SoundParameter::LFO2Rise),
            109 => Some(SoundParameter::LFO2Phase),

            // Arpeggiator
            110 => Some(SoundParameter::ArpMode),
            111 => Some(SoundParameter::ArpGrid),
            112 => Some(SoundParameter::ArpTempo),
            113 => Some(SoundParameter::ArpHold),

            // Amplifier
            80 => Some(SoundParameter::AmpLevel),
            81 => Some(SoundParameter::AmpPan),

            // Modulations
            17 => Some(SoundParameter::ModEnvF),
            18 => Some(SoundParameter::ModEnvA),
            19 => Some(SoundParameter::ModLFO1),
            20 => Some(SoundParameter::ModLFO2),
            87 => Some(SoundParameter::ModModwheel),
            88 => Some(SoundParameter::ModPitchbend),
            89 => Some(SoundParameter::ModVelocity),
            90 => Some(SoundParameter::ModAftertouch),

            // Misc
            84 => Some(SoundParameter::BendRange),
            85 => Some(SoundParameter::Tune),

            // Unknown
            _ => None,
        };

        if param.is_none() {
            return None;
        }

        let param = param.unwrap();

        let value = match param {
            // Default
            _ => {
                let range = param.get_range();
                rescale(cc_value as i32, 0, 127, *range.start(), *range.end())
            }
        };

        Some((param, value))
    }
}

fn rescale(value: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
    (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
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
