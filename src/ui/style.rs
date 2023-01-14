//! Style definitions for the different elements

use std::rc::Rc;

use iced::overlay::menu;
use iced::widget::{button, checkbox, container, pick_list, slider};
use iced::{Background, Color, Theme, Vector};

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
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xab, 0xa3, 0x39))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

impl From<OscSection> for iced::theme::Container {
    fn from(val: OscSection) -> Self {
        Self::Custom(Box::new(val))
    }
}

/// Styles for the extra section
pub struct ExtraSection;

impl container::StyleSheet for ExtraSection {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xf9, 0xb0, 0x8b))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

impl From<ExtraSection> for iced::theme::Container {
    fn from(val: ExtraSection) -> Self {
        Self::Custom(Box::new(val))
    }
}

/// Styles for the shaper section
pub struct ShaperSection;

impl container::StyleSheet for ShaperSection {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xd8, 0x00, 0x00))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

impl From<ShaperSection> for iced::theme::Container {
    fn from(val: ShaperSection) -> Self {
        Self::Custom(Box::new(val))
    }
}

/// Styles for the filter section
pub struct FilterSection;

impl container::StyleSheet for FilterSection {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xd8, 0x00, 0x00))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

impl From<FilterSection> for iced::theme::Container {
    fn from(val: FilterSection) -> Self {
        Self::Custom(Box::new(val))
    }
}

/// Styles for the amplifier section
pub struct AmpSection;

impl container::StyleSheet for AmpSection {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0x65, 0xa4, 0x7e))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

impl From<AmpSection> for iced::theme::Container {
    fn from(val: AmpSection) -> Self {
        Self::Custom(Box::new(val))
    }
}

/// Styles for the LFO sections
pub struct LFOSection;

impl container::StyleSheet for LFOSection {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xd2, 0x6a, 0x25))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

impl From<LFOSection> for iced::theme::Container {
    fn from(val: LFOSection) -> Self {
        Self::Custom(Box::new(val))
    }
}

/// Styles for the envelope sections
pub struct EnvSection;

impl container::StyleSheet for EnvSection {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xff, 0xbd, 0x00))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

impl From<EnvSection> for iced::theme::Container {
    fn from(val: EnvSection) -> Self {
        Self::Custom(Box::new(val))
    }
}

/// Styles for the arpeggiator section
pub struct ArpSection;

impl container::StyleSheet for ArpSection {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xf9, 0xb0, 0x8b))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

impl From<ArpSection> for iced::theme::Container {
    fn from(val: ArpSection) -> Self {
        Self::Custom(Box::new(val))
    }
}

/// Styles for the misc section
pub struct MiscSection;

impl container::StyleSheet for MiscSection {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xC0, 0xC0, 0xC0))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

impl From<MiscSection> for iced::theme::Container {
    fn from(val: MiscSection) -> Self {
        Self::Custom(Box::new(val))
    }
}

/// Styles for the modulation section
pub struct ModSection;

impl container::StyleSheet for ModSection {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xb4, 0xcb, 0xd9))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

impl From<ModSection> for iced::theme::Container {
    fn from(val: ModSection) -> Self {
        Self::Custom(Box::new(val))
    }
}

/// Styles for the FX section
pub struct FXSection;

impl container::StyleSheet for FXSection {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0x65, 0xa4, 0x7e))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

impl From<FXSection> for iced::theme::Container {
    fn from(val: FXSection) -> Self {
        Self::Custom(Box::new(val))
    }
}

/// Styles for the mixer section
pub struct MixerSection;

impl container::StyleSheet for MixerSection {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xC0, 0xC0, 0xC0))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

impl From<MixerSection> for iced::theme::Container {
    fn from(val: MixerSection) -> Self {
        Self::Custom(Box::new(val))
    }
}

/// Styles for the MIDI section
pub struct MidiSection;

impl container::StyleSheet for MidiSection {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Some(Background::Color(Color::from_rgb8(0xC0, 0xC0, 0xC0))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
        }
    }
}

impl From<MidiSection> for iced::theme::Container {
    fn from(val: MidiSection) -> Self {
        Self::Custom(Box::new(val))
    }
}

/// Styles for the main window
pub struct MainWindow;

impl container::StyleSheet for MainWindow {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(Color::from_rgb8(0xFF, 0xFF, 0xFF)),
            background: Some(Background::Color(Color::from_rgb8(0x20, 0x20, 0x20))),
            ..Default::default()
        }
    }
}

impl From<MainWindow> for iced::theme::Container {
    fn from(val: MainWindow) -> Self {
        Self::Custom(Box::new(val))
    }
}

/// Styles for all sliders
pub struct Slider;

impl slider::StyleSheet for Slider {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail_colors: (ACTIVE, Color { a: 0.1, ..ACTIVE }),
            handle: slider::Handle {
                shape: slider::HandleShape::Circle { radius: 6.0 },
                color: ACTIVE,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn hovered(&self, _style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail_colors: (ACTIVE, Color { a: 0.1, ..ACTIVE }),
            handle: slider::Handle {
                shape: slider::HandleShape::Circle { radius: 6.0 },
                color: HOVERED,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }

    fn dragging(&self, _style: &Self::Style) -> slider::Appearance {
        slider::Appearance {
            rail_colors: (ACTIVE, Color { a: 0.1, ..ACTIVE }),
            handle: slider::Handle {
                shape: slider::HandleShape::Circle { radius: 6.0 },
                color: Color::from_rgb8(0x50, 0x50, 0x50),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        }
    }
}

impl From<Slider> for iced::theme::Slider {
    fn from(val: Slider) -> Self {
        Self::Custom(Box::new(val))
    }
}

/// Styles for all checkboxes
pub struct Checkbox;

impl checkbox::StyleSheet for Checkbox {
    type Style = Theme;

    fn active(&self, _style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            text_color: Some(SECTION_TEXT_COLOR),
            background: if is_checked { ACTIVE } else { SURFACE }.into(),
            checkmark_color: Color::WHITE,
            border_radius: 2.0,
            border_width: 1.0,
            border_color: ACTIVE,
        }
    }

    fn hovered(&self, _style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        checkbox::Appearance {
            text_color: Some(SECTION_TEXT_COLOR),
            background: Color {
                a: 0.8,
                ..if is_checked { ACTIVE } else { HOVERED }
            }
            .into(),
            checkmark_color: Color::WHITE,
            border_radius: 2.0,
            border_width: 1.0,
            border_color: ACTIVE,
        }
    }
}

impl From<Checkbox> for iced::theme::Checkbox {
    fn from(val: Checkbox) -> Self {
        Self::Custom(Box::new(val))
    }
}

/// Styles for all dropdown menus
pub struct PickList;

impl pick_list::StyleSheet for PickList {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: Color::from_rgb8(0xFF, 0xFF, 0xFF),
            placeholder_color: Color::from_rgb8(0xFF, 0xFF, 0xFF),
            background: Background::Color(Color::from_rgb8(0x20, 0x20, 0x20)),
            border_radius: 5.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(0x80, 0x80, 0x80),
            handle_color: Color::from_rgb8(0x80, 0x80, 0x80),
        }
    }

    fn hovered(&self, _style: &Self::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: Color::from_rgb8(0xFF, 0xFF, 0xFF),
            placeholder_color: Color::from_rgb8(0xFF, 0xFF, 0xFF),
            background: Background::Color(HOVERED),
            border_radius: 5.0,
            border_width: 1.0,
            border_color: Color::from_rgb8(0x80, 0x80, 0x80),
            handle_color: Color::from_rgb8(0x80, 0x80, 0x80),
        }
    }
}

pub struct Menu;

impl menu::StyleSheet for Menu {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> menu::Appearance {
        menu::Appearance {
            text_color: Color::from_rgb8(0xFF, 0xFF, 0xFF),
            background: Background::Color(Color::from_rgb8(0x20, 0x20, 0x20)),
            border_width: 1.0,
            border_radius: 5.0,
            border_color: Color::from_rgb8(0x80, 0x80, 0x80),
            selected_text_color: Color::from_rgb8(0xFF, 0xFF, 0xFF),
            selected_background: Background::Color(Color::from_rgb8(0x80, 0x80, 0x80)),
        }
    }
}

impl From<PickList> for iced::theme::PickList {
    fn from(val: PickList) -> Self {
        Self::Custom(Rc::new(val), Rc::new(Menu))
    }
}

/// Different button variations
pub struct Button;

impl button::StyleSheet for Button {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.11, 0.42, 0.87))),
            border_radius: 5.0,
            shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.11, 0.42, 0.87))),
            border_radius: 5.0,
            shadow_offset: Vector::new(1.0, 2.0),
            text_color: Color::WHITE,
            ..button::Appearance::default()
        }
    }
}

impl From<Button> for iced::theme::Button {
    fn from(val: Button) -> Self {
        Self::Custom(Box::new(val))
    }
}
