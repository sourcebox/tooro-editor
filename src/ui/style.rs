//! Style definitions for the different elements

use iced::{button, checkbox, container, pick_list, slider, Background, Color, Vector};

/// Default window width
pub const WINDOW_WIDTH: u32 = 1024;

/// Default window height
pub const WINDOW_HEIGHT: u32 = 655;

/// Common element padding
pub const SECTION_PADDING: u16 = 7;

/// Common element spacing
pub const SECTION_SPACING: u16 = 1;

/// Text size for section labels
pub const SECTION_LABEL_TEXT_SIZE: u16 = 16;

/// Text size for parameter labels
pub const PARAM_LABEL_TEXT_SIZE: u16 = 14;

/// Width of parameter labels
pub const PARAM_LABEL_WIDTH: u16 = 65;

/// Width of parameter values
pub const PARAM_VALUE_WIDTH: u16 = 25;

/// Text size of dropdown menu items
pub const LIST_ITEM_TEXT_SIZE: u16 = 13;

/// Button text size
pub const BUTTON_TEXT_SIZE: u16 = 14;

/// Text size of status bar items
pub const STATUS_TEXT_SIZE: u16 = 14;

/// Text color for all section elements
const SECTION_TEXT_COLOR: Color = Color::from_rgb(0_f32, 0_f32, 0_f32);

/// Color for active elements
const ACTIVE: Color = Color::from_rgb(
    0x20 as f32 / 255.0,
    0x20 as f32 / 255.0,
    0x20 as f32 / 255.0,
);

/// Color for hovered elements
const HOVERED: Color = Color::from_rgb(
    0x67 as f32 / 255.0,
    0x7B as f32 / 255.0,
    0xC4 as f32 / 255.0,
);

/// Surface color for checkboxes
const SURFACE: Color = Color::from_rgb(
    0x20 as f32 / 255.0,
    0x20 as f32 / 255.0,
    0x20 as f32 / 255.0,
);

/// Styles for the oscillator sections
pub struct OscSection;

impl container::StyleSheet for OscSection {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xab, 0xa3, 0x39))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

/// Styles for the extra section
pub struct ExtraSection;

impl container::StyleSheet for ExtraSection {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xf9, 0xb0, 0x8b))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

/// Styles for the shaper section
pub struct ShaperSection;

impl container::StyleSheet for ShaperSection {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xd8, 0x00, 0x00))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

/// Styles for the filter section
pub struct FilterSection;

impl container::StyleSheet for FilterSection {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xd8, 0x00, 0x00))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

/// Styles for the amplifier section
pub struct AmpSection;

impl container::StyleSheet for AmpSection {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0x65, 0xa4, 0x7e))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

/// Styles for the LFO sections
pub struct LFOSection;

impl container::StyleSheet for LFOSection {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xd2, 0x6a, 0x25))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

/// Styles for the envelope sections
pub struct EnvSection;

impl container::StyleSheet for EnvSection {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xff, 0xbd, 0x00))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

/// Styles for the arpeggiator section
pub struct ArpSection;

impl container::StyleSheet for ArpSection {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xf9, 0xb0, 0x8b))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

/// Styles for the misc section
pub struct MiscSection;

impl container::StyleSheet for MiscSection {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xC0, 0xC0, 0xC0))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

/// Styles for the modulation section
pub struct ModSection;

impl container::StyleSheet for ModSection {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xb4, 0xcb, 0xd9))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

/// Styles for the FX section
pub struct FXSection;

impl container::StyleSheet for FXSection {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0x65, 0xa4, 0x7e))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

/// Styles for the mixer section
pub struct MixerSection;

impl container::StyleSheet for MixerSection {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xC0, 0xC0, 0xC0))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

/// Styles for the MIDI section
pub struct MidiSection;

impl container::StyleSheet for MidiSection {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xC0, 0xC0, 0xC0))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

/// Styles for the main window
pub struct MainWindow;

impl container::StyleSheet for MainWindow {
    fn style(&self) -> container::Style {
        container::Style {
            text_color: Some(Color::from_rgb8(0xFF, 0xFF, 0xFF)),
            background: Some(Background::Color(Color::from_rgb8(0x20, 0x20, 0x20))),
            ..Default::default()
        }
    }
}

/// Styles for all sliders
pub struct Slider;

impl slider::StyleSheet for Slider {
    fn active(&self) -> slider::Style {
        slider::Style {
            rail_colors: (ACTIVE, Color { a: 0.1, ..ACTIVE }),
            handle: slider::Handle {
                shape: slider::HandleShape::Circle { radius: 6.0 },
                color: ACTIVE,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self) -> slider::Style {
        let active = self.active();

        slider::Style {
            handle: slider::Handle {
                color: HOVERED,
                ..active.handle
            },
            ..active
        }
    }

    fn dragging(&self) -> slider::Style {
        let active = self.active();

        slider::Style {
            handle: slider::Handle {
                color: Color::from_rgb8(0x50, 0x50, 0x50),
                ..active.handle
            },
            ..active
        }
    }
}

/// Styles for all checkboxes
pub struct Checkbox;

impl checkbox::StyleSheet for Checkbox {
    fn active(&self, is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            text_color: Some(Color::from_rgb8(0xFF, 0xFF, 0xFF)),
            background: if is_checked { ACTIVE } else { SURFACE }.into(),
            checkmark_color: Color::WHITE,
            border_radius: 2.0,
            border_width: 1.0,
            border_color: ACTIVE,
        }
    }

    fn hovered(&self, is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: Color {
                a: 0.8,
                ..if is_checked { ACTIVE } else { HOVERED }
            }
            .into(),
            ..self.active(is_checked)
        }
    }
}

/// Styles for all dropdown menus
pub struct PickList;

impl pick_list::StyleSheet for PickList {
    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            text_color: Color::from_rgb8(0xFF, 0xFF, 0xFF),
            placeholder_color: Color::from_rgb8(0xFF, 0xFF, 0xFF),
            background: Background::Color(Color::from_rgb8(0x20, 0x20, 0x20)),
            border_radius: 5.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(0x80, 0x80, 0x80),
            icon_size: 0.5,
        }
    }

    fn menu(&self) -> pick_list::Menu {
        pick_list::Menu {
            text_color: Color::from_rgb8(0xFF, 0xFF, 0xFF),
            background: Background::Color(Color::from_rgb8(0x20, 0x20, 0x20)),
            border_width: 1.0,
            border_color: Color::from_rgb8(0x80, 0x80, 0x80),
            selected_text_color: Color::from_rgb8(0xFF, 0xFF, 0xFF),
            selected_background: Background::Color(Color::from_rgb8(0x80, 0x80, 0x80)),
        }
    }

    fn hovered(&self) -> pick_list::Style {
        pick_list::Style {
            background: Background::Color(HOVERED),
            ..self.active()
        }
    }
}

/// Different button variations
#[allow(dead_code)]
pub enum Button {
    Primary,
    Secondary,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(match self {
                Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
            })),
            border_radius: 5.0,
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: Color::WHITE,
            shadow_offset: Vector::new(1.0, 2.0),
            ..self.active()
        }
    }
}
