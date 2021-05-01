use iced::{checkbox, container, pick_list, slider, Background, Color};

// Default window size
pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 630;

// Element spacing
pub const SECTION_PADDING: u16 = 7;
pub const SECTION_SPACING: u16 = 1;

// Text sizes
pub const SECTION_LABEL_TEXT_SIZE: u16 = 16;
pub const PARAM_LABEL_TEXT_SIZE: u16 = 14;
pub const PARAM_LABEL_WIDTH: u16 = 65;
pub const PARAM_VALUE_WIDTH: u16 = 25;
pub const LIST_ITEM_TEXT_SIZE: u16 = 13;

pub struct OscSection;

impl container::StyleSheet for OscSection {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0x90, 0xFC, 0xA2))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
            ..Default::default()
        }
    }
}

pub struct ShaperSection;

impl container::StyleSheet for ShaperSection {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0x9B, 0xD1, 0xE8))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
            ..Default::default()
        }
    }
}

pub struct FilterSection;

impl container::StyleSheet for FilterSection {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0xFF, 0x6E, 0x6E))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
            ..Default::default()
        }
    }
}

pub struct AmpSection;

impl container::StyleSheet for AmpSection {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0x80, 0xBE, 0x8B))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
            ..Default::default()
        }
    }
}

pub struct LFOSection;

impl container::StyleSheet for LFOSection {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0xC0, 0xBE, 0x4B))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
            ..Default::default()
        }
    }
}

pub struct EnvSection;

impl container::StyleSheet for EnvSection {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0xC0, 0xBE, 0x8B))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
            ..Default::default()
        }
    }
}

pub struct ArpSection;

impl container::StyleSheet for ArpSection {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0xDE, 0xB2, 0x97))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
            ..Default::default()
        }
    }
}

pub struct MiscSection;

impl container::StyleSheet for MiscSection {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0xD6, 0xDE, 0x97))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
            ..Default::default()
        }
    }
}

pub struct ModSection;

impl container::StyleSheet for ModSection {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0xF2, 0xCB, 0x68))),
            border_width: 0.0,
            border_color: Color::from_rgb(0.7, 0.7, 0.7),
            border_radius: 5.0,
            ..Default::default()
        }
    }
}

pub struct MainWindow;

impl container::StyleSheet for MainWindow {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0x20, 0x20, 0x20))),
            ..Default::default()
        }
    }
}

const ACTIVE: Color = Color::from_rgb(
    0x20 as f32 / 255.0,
    0x20 as f32 / 255.0,
    0x20 as f32 / 255.0,
);

const HOVERED: Color = Color::from_rgb(
    0x67 as f32 / 255.0,
    0x7B as f32 / 255.0,
    0xC4 as f32 / 255.0,
);

const SURFACE: Color = Color::from_rgb(
    0x20 as f32 / 255.0,
    0x20 as f32 / 255.0,
    0x20 as f32 / 255.0,
);

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

pub struct Checkbox;

impl checkbox::StyleSheet for Checkbox {
    fn active(&self, is_checked: bool) -> checkbox::Style {
        checkbox::Style {
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

pub struct PickList;

impl pick_list::StyleSheet for PickList {
    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            text_color: Color::from_rgb8(0xFF, 0xFF, 0xFF),
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
