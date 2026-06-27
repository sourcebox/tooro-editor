//! Style definitions for the different elements.

use iced::{color, theme::Palette, widget, Background, Border, Color, Shadow, Theme};

/// Default window width.
pub const WINDOW_WIDTH: u32 = 1024;

/// Default window height.
pub const WINDOW_HEIGHT: u32 = 655;

/// Common element padding.
pub const SECTION_PADDING: u16 = 7;

/// Common element spacing.
pub const SECTION_SPACING: u32 = 1;

/// Text size for section labels.
pub const SECTION_LABEL_TEXT_SIZE: u32 = 14;

/// Text size for parameter labels.
pub const PARAM_LABEL_TEXT_SIZE: u32 = 12;

/// Width of parameter labels.
pub const PARAM_LABEL_WIDTH: u32 = 65;

/// Width of parameter values.
pub const PARAM_VALUE_WIDTH: u32 = 25;

/// Text size of dropdown menu items.
pub const LIST_ITEM_TEXT_SIZE: u32 = 11;

/// Button text size.
pub const BUTTON_TEXT_SIZE: u32 = 11;

/// Text size of status bar items.
pub const STATUS_TEXT_SIZE: u32 = 11;

/// Text color for all section elements.
const SECTION_TEXT_COLOR: Color = Color::BLACK;

/// Color for active elements.
const ACTIVE: Color = color!(0x202020);

/// Color for hovered elements.
const HOVERED: Color = color!(0x677BC4);

/// Color for dragged elements.
const DRAGGED: Color = color!(0x505050);

/// Color for disabled elements.
const DISABLED: Color = color!(0x202020);

/// Color for background.
const BACKGROUND: Color = color!(0x101010);

/// Returns a custom theme.
pub fn theme() -> Theme {
    Theme::custom(
        "Custom",
        Palette {
            background: BACKGROUND,
            text: Color::WHITE,
            primary: color!(0x5865F2),
            success: color!(0x12664f),
            warning: color!(0xffc14e),
            danger: color!(0xc3423f),
        },
    )
}

/// Returns a style for a section with a specific background color.
pub fn section(background: Color) -> widget::container::Style {
    widget::container::Style::default()
        .color(SECTION_TEXT_COLOR)
        .background(background)
        .border(Border::default().rounded(5.0))
}

/// Returns a style for the button.
pub fn button(status: widget::button::Status) -> widget::button::Style {
    use widget::button::*;

    let text_color = match status {
        Status::Active => color!(0xEEEEEE),
        Status::Hovered | Status::Pressed => Color::WHITE,
        Status::Disabled => DISABLED,
    };

    Style {
        background: Some(Background::Color(Color::from_rgb(0.11, 0.42, 0.87))),
        text_color,
        border: Border {
            color: Color::from_rgb(0.11, 0.42, 0.87),
            width: 1.0,
            radius: 5.0.into(),
        },
        shadow: Shadow::default(),
        snap: false,
    }
}

/// Returns a style for the slider.
pub fn slider(status: widget::slider::Status) -> widget::slider::Style {
    use widget::slider::*;

    let handle_color = match status {
        Status::Active => ACTIVE,
        Status::Hovered => HOVERED,
        Status::Dragged => DRAGGED,
    };

    Style {
        rail: Rail {
            backgrounds: (
                Background::Color(ACTIVE),
                Background::Color(Color { a: 0.1, ..ACTIVE }),
            ),
            width: 2.0,
            border: Border::default(),
        },
        handle: Handle {
            shape: HandleShape::Circle { radius: 6.0 },
            background: Background::Color(handle_color),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
    }
}

/// Returns a style for the pick list.
pub fn pick_list(_status: widget::pick_list::Status) -> widget::pick_list::Style {
    use widget::pick_list::*;

    Style {
        text_color: Color::WHITE,
        placeholder_color: Color::WHITE,
        handle_color: Color::WHITE,
        background: Background::Color(BACKGROUND),
        border: Border {
            color: color!(0x808080),
            width: 1.0,
            radius: 5.0.into(),
        },
    }
}

/// Returns a style for the checkbox.
pub fn checkbox(status: widget::checkbox::Status) -> widget::checkbox::Style {
    use widget::checkbox::*;

    let background_color = match status {
        Status::Active { is_checked } => {
            if is_checked {
                ACTIVE
            } else {
                DISABLED
            }
        }
        Status::Hovered { is_checked } => {
            if is_checked {
                ACTIVE
            } else {
                HOVERED
            }
        }
        Status::Disabled { is_checked: _ } => DISABLED,
    };

    Style {
        background: Background::Color(background_color),
        icon_color: Color::WHITE,
        border: Border {
            color: ACTIVE,
            width: 1.0,
            radius: 2.0.into(),
        },
        text_color: Some(SECTION_TEXT_COLOR),
    }
}

/// Returns a style for the rule.
pub fn rule() -> widget::rule::Style {
    use widget::rule::*;

    Style {
        color: Color::from_rgb(0.3, 0.3, 0.3),
        radius: 2.0.into(),
        fill_mode: FillMode::Full,
        snap: false,
    }
}
