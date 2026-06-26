//! Style definitions for the different elements

use iced::widget::{button, checkbox, container, pick_list, slider};
use iced::{Background, Border, Color, Theme, Vector};

/// Default window width
pub const WINDOW_WIDTH: u32 = 1024;

/// Default window height
pub const WINDOW_HEIGHT: u32 = 655;

/// Common element padding
pub const SECTION_PADDING: u16 = 7;

/// Common element spacing
pub const SECTION_SPACING: u32 = 1;

/// Text size for section labels
pub const SECTION_LABEL_TEXT_SIZE: u32 = 16;

/// Text size for parameter labels
pub const PARAM_LABEL_TEXT_SIZE: u32 = 14;

/// Width of parameter labels
pub const PARAM_LABEL_WIDTH: u32 = 65;

/// Width of parameter values
pub const PARAM_VALUE_WIDTH: u16 = 25;

/// Text size of dropdown menu items
pub const LIST_ITEM_TEXT_SIZE: u32 = 13;

/// Button text size
pub const BUTTON_TEXT_SIZE: u32 = 14;

/// Text size of status bar items
pub const STATUS_TEXT_SIZE: u32 = 14;

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

/// Returns a style for a section with a specific background color.
pub fn section(background: Color) -> container::Style {
    container::Style::default()
        .color(SECTION_TEXT_COLOR)
        .background(background)
        .border(Border::default().rounded(5.0))
}

/// Styles for all sliders
pub struct Slider;

// impl slider::StyleSheet for Slider {
//     type Style = Theme;

//     fn active(&self, _style: &Self::Style) -> slider::Appearance {
//         slider::Appearance {
//             rail: slider::Rail {
//                 colors: (ACTIVE, Color { a: 0.1, ..ACTIVE }),
//                 width: 0.0,
//             },
//             handle: slider::Handle {
//                 shape: slider::HandleShape::Circle { radius: 6.0 },
//                 color: ACTIVE,
//                 border_width: 0.0,
//                 border_color: Color::TRANSPARENT,
//             },
//         }
//     }

//     fn hovered(&self, _style: &Self::Style) -> slider::Appearance {
//         slider::Appearance {
//             rail: slider::Rail {
//                 colors: (ACTIVE, Color { a: 0.1, ..ACTIVE }),
//                 width: 0.0,
//             },
//             handle: slider::Handle {
//                 shape: slider::HandleShape::Circle { radius: 6.0 },
//                 color: HOVERED,
//                 border_width: 0.0,
//                 border_color: Color::TRANSPARENT,
//             },
//         }
//     }

//     fn dragging(&self, _style: &Self::Style) -> slider::Appearance {
//         slider::Appearance {
//             rail: slider::Rail {
//                 colors: (ACTIVE, Color { a: 0.1, ..ACTIVE }),
//                 width: 0.0,
//             },
//             handle: slider::Handle {
//                 shape: slider::HandleShape::Circle { radius: 6.0 },
//                 color: Color::from_rgb8(0x50, 0x50, 0x50),
//                 border_width: 0.0,
//                 border_color: Color::TRANSPARENT,
//             },
//         }
//     }
// }

// impl From<Slider> for iced::theme::Slider {
//     fn from(val: Slider) -> Self {
//         Self::Custom(Box::new(val))
//     }
// }

/// Styles for all checkboxes
pub struct Checkbox;

// impl checkbox::StyleSheet for Checkbox {
//     type Style = Theme;

//     fn active(&self, _style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
//         checkbox::Appearance {
//             text_color: Some(SECTION_TEXT_COLOR),
//             background: if is_checked { ACTIVE } else { SURFACE }.into(),
//             icon_color: Color::WHITE,
//             border_radius: 2.0,
//             border_width: 1.0,
//             border_color: ACTIVE,
//         }
//     }

//     fn hovered(&self, _style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
//         checkbox::Appearance {
//             text_color: Some(SECTION_TEXT_COLOR),
//             background: Color {
//                 a: 0.8,
//                 ..if is_checked { ACTIVE } else { HOVERED }
//             }
//             .into(),
//             icon_color: Color::WHITE,
//             border_radius: 2.0,
//             border_width: 1.0,
//             border_color: ACTIVE,
//         }
//     }
// }

// impl From<Checkbox> for iced::theme::Checkbox {
//     fn from(val: Checkbox) -> Self {
//         Self::Custom(Box::new(val))
//     }
// }

/// Styles for all dropdown menus
pub struct PickList;

// impl pick_list::StyleSheet for PickList {
//     type Style = Theme;

//     fn active(&self, _style: &Self::Style) -> pick_list::Appearance {
//         pick_list::Appearance {
//             text_color: Color::from_rgb8(0xFF, 0xFF, 0xFF),
//             placeholder_color: Color::from_rgb8(0xFF, 0xFF, 0xFF),
//             background: Background::Color(Color::from_rgb8(0x20, 0x20, 0x20)),
//             border_radius: 5.0,
//             border_width: 1.0,
//             border_color: Color::from_rgb8(0x80, 0x80, 0x80),
//             handle_color: Color::from_rgb8(0x80, 0x80, 0x80),
//         }
//     }

//     fn hovered(&self, _style: &Self::Style) -> pick_list::Appearance {
//         pick_list::Appearance {
//             text_color: Color::from_rgb8(0xFF, 0xFF, 0xFF),
//             placeholder_color: Color::from_rgb8(0xFF, 0xFF, 0xFF),
//             background: Background::Color(HOVERED),
//             border_radius: 5.0,
//             border_width: 1.0,
//             border_color: Color::from_rgb8(0x80, 0x80, 0x80),
//             handle_color: Color::from_rgb8(0x80, 0x80, 0x80),
//         }
//     }
// }

pub struct Menu;

// impl menu::StyleSheet for Menu {
//     type Style = Theme;

//     fn appearance(&self, _style: &Self::Style) -> menu::Appearance {
//         menu::Appearance {
//             text_color: Color::from_rgb8(0xFF, 0xFF, 0xFF),
//             background: Background::Color(Color::from_rgb8(0x20, 0x20, 0x20)),
//             border_width: 1.0,
//             border_radius: 5.0,
//             border_color: Color::from_rgb8(0x80, 0x80, 0x80),
//             selected_text_color: Color::from_rgb8(0xFF, 0xFF, 0xFF),
//             selected_background: Background::Color(Color::from_rgb8(0x80, 0x80, 0x80)),
//         }
//     }
// }

// impl From<PickList> for iced::theme::PickList {
//     fn from(val: PickList) -> Self {
//         Self::Custom(Rc::new(val), Rc::new(Menu))
//     }
// }

/// Different button variations
pub struct Button;

// impl button::StyleSheet for Button {
//     type Style = Theme;

//     fn active(&self, _style: &Self::Style) -> button::Appearance {
//         button::Appearance {
//             background: Some(Background::Color(Color::from_rgb(0.11, 0.42, 0.87))),
//             border_radius: 5.0,
//             shadow_offset: Vector::new(1.0, 1.0),
//             text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
//             ..button::Appearance::default()
//         }
//     }

//     fn hovered(&self, _style: &Self::Style) -> button::Appearance {
//         button::Appearance {
//             background: Some(Background::Color(Color::from_rgb(0.11, 0.42, 0.87))),
//             border_radius: 5.0,
//             shadow_offset: Vector::new(1.0, 2.0),
//             text_color: Color::WHITE,
//             ..button::Appearance::default()
//         }
//     }
// }
//
// impl From<Button> for iced::theme::Button {
//     fn from(val: Button) -> Self {
//         Self::Custom(Box::new(val))
//     }
// }
