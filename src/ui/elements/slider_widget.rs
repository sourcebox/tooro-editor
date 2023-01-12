//! Display an interactive selector of a single value from a range of values.
//!
//! This is a modified version of the original slider widget from `iced_native`
//! with the following changes:
//!     - Use of pointer shape mouse cursor when hovering the slider.
//!     - Mouse wheel support.
//!     - Control-click/right-click resets slider to a default value.
//!     - Shift-drag enables fine control.
//!     - Clippy related fixes.
//!
//! A [`Slider`] has some local [`State`].

use iced::mouse::ScrollDelta;
use iced_native::event::{self, Event};
use iced_native::keyboard;
use iced_native::layout;
use iced_native::mouse;
use iced_native::renderer;
use iced_native::touch;
use iced_native::widget::tree::{self, Tree};
use iced_native::{
    Background, Clipboard, Color, Element, Layout, Length, Point, Rectangle, Shell, Size, Widget,
};
pub use iced_style::slider::{Handle, HandleShape, StyleSheet};

use std::ops::{Add, RangeInclusive};

/// An horizontal bar and a handle that selects a single value from a range of
/// values.
///
/// A [`Slider`] will try to fill the horizontal space of its container.
///
/// The [`Slider`] range of numeric values is generic and its step size defaults
/// to 1 unit.
///
/// # Example
/// ```
/// # use iced_native::widget::slider;
/// # use iced_native::renderer::Null;
/// #
/// # type Slider<'a, T, Message> = slider::Slider<'a, T, Message, Null>;
/// #
/// #[derive(Clone)]
/// pub enum Message {
///     SliderChanged(f32),
/// }
///
/// let value = 50.0;
///
/// Slider::new(0.0..=100.0, value, Message::SliderChanged);
/// ```
///
/// ![Slider drawn by Coffee's renderer](https://github.com/hecrj/coffee/blob/bda9818f823dfcb8a7ad0ff4940b4d4b387b5208/images/ui/slider.png?raw=true)
#[allow(missing_debug_implementations)]
pub struct Slider<'a, T, Message, Renderer>
where
    Renderer: iced_native::Renderer,
    Renderer::Theme: StyleSheet,
{
    range: RangeInclusive<T>,
    step: T,
    value: T,
    default: T,
    on_change: Box<dyn Fn(T) -> Message + 'a>,
    on_release: Option<Message>,
    width: Length,
    height: u16,
    style: <Renderer::Theme as StyleSheet>::Style,
}

impl<'a, T, Message, Renderer> Slider<'a, T, Message, Renderer>
where
    T: Copy + From<u8> + std::cmp::PartialOrd,
    Message: Clone,
    Renderer: iced_native::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// The default height of a [`Slider`].
    pub const DEFAULT_HEIGHT: u16 = 22;

    /// Creates a new [`Slider`].
    ///
    /// It expects:
    ///   * an inclusive range of possible values
    ///   * the current value of the [`Slider`]
    ///   * a function that will be called when the [`Slider`] is dragged.
    ///   It receives the new value of the [`Slider`] and must produce a
    ///   `Message`.
    pub fn new<F>(range: RangeInclusive<T>, value: T, default: T, on_change: F) -> Self
    where
        F: 'static + Fn(T) -> Message,
    {
        let value = if value >= *range.start() {
            value
        } else {
            *range.start()
        };

        let value = if value <= *range.end() {
            value
        } else {
            *range.end()
        };

        Slider {
            value,
            range,
            step: T::from(1),
            default,
            on_change: Box::new(on_change),
            on_release: None,
            width: Length::Fill,
            height: Self::DEFAULT_HEIGHT,
            style: Default::default(),
        }
    }

    /// Sets the release message of the [`Slider`].
    /// This is called when the mouse is released from the slider.
    ///
    /// Typically, the user's interaction with the slider is finished when this message is produced.
    /// This is useful if you need to spawn a long-running task from the slider's result, where
    /// the default on_change message could create too many events.
    pub fn on_release(mut self, on_release: Message) -> Self {
        self.on_release = Some(on_release);
        self
    }

    /// Sets the width of the [`Slider`].
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Slider`].
    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    /// Sets the style of the [`Slider`].
    pub fn style(mut self, style: impl Into<<Renderer::Theme as StyleSheet>::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Sets the step size of the [`Slider`].
    pub fn step(mut self, step: T) -> Self {
        self.step = step;
        self
    }
}

/// Processes an [`Event`] and updates the [`State`] of a [`Slider`]
/// accordingly.
#[allow(clippy::too_many_arguments)]
pub fn update<Message, T>(
    event: Event,
    layout: Layout<'_>,
    cursor_position: Point,
    shell: &mut Shell<'_, Message>,
    state: &mut State,
    value: &mut T,
    range: &RangeInclusive<T>,
    step: T,
    default: T,
    on_change: &dyn Fn(T) -> Message,
    on_release: &Option<Message>,
) -> event::Status
where
    T: Default + Copy + Into<f64> + Add<Output = T> + Ord + num_traits::FromPrimitive,
    Message: Clone,
{
    let is_dragging = state.is_dragging;

    match event {
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
        | Event::Touch(touch::Event::FingerPressed { .. }) => {
            let bounds = layout.bounds();
            if bounds.contains(cursor_position) {
                if state.control_pressed {
                    let new_value = default;
                    shell.publish((on_change)(new_value));
                    *value = new_value;
                } else {
                    let step = step.into();
                    let start = (*range.start()).into();
                    let end = (*range.end()).into();
                    let percent = f64::from(cursor_position.x - bounds.x) / f64::from(bounds.width);
                    let steps = (percent * (end - start) / step).round();
                    let v = steps * step + start;
                    let new_value = T::from_f64(v)
                        .unwrap_or_default()
                        .clamp(*range.start(), *range.end());
                    shell.publish((on_change)(new_value));
                    *value = new_value;
                    state.is_dragging = true;
                    state.click_pos_x = cursor_position.x;
                    state.click_value = v;
                }

                return event::Status::Captured;
            }
        }
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
        | Event::Touch(touch::Event::FingerLifted { .. })
        | Event::Touch(touch::Event::FingerLost { .. }) => {
            if is_dragging {
                if let Some(on_release) = on_release.clone() {
                    shell.publish(on_release);
                }
                state.is_dragging = false;

                return event::Status::Captured;
            }
        }
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) => {
            if layout.bounds().contains(cursor_position) {
                let new_value = default;
                shell.publish((on_change)(new_value));
                *value = new_value;
            }
        }
        Event::Mouse(mouse::Event::CursorMoved { .. })
        | Event::Touch(touch::Event::FingerMoved { .. }) => {
            if is_dragging {
                let bounds = layout.bounds();
                let step = step.into();
                let start = (*range.start()).into();
                let end = (*range.end()).into();
                let mut percent =
                    f64::from(cursor_position.x - state.click_pos_x) / f64::from(bounds.width);
                if state.shift_pressed {
                    percent /= 4.0;
                }
                let steps = (percent * (end - start) / step).round();
                let v = state.click_value + steps;
                let new_value = T::from_f64(v)
                    .unwrap_or_default()
                    .clamp(*range.start(), *range.end());
                shell.publish((on_change)(new_value));
                *value = new_value;

                return event::Status::Captured;
            }
        }
        Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
            if layout.bounds().contains(cursor_position) {
                let delta = match delta {
                    ScrollDelta::Lines { x: _, y } => {
                        if y.is_sign_positive() {
                            y.ceil()
                        } else {
                            y.floor()
                        }
                    }
                    _ => 0.0,
                };

                let new_value = { *value + T::from_f32(delta).unwrap_or_default() }
                    .clamp(*range.start(), *range.end());
                shell.publish((on_change)(new_value));

                *value = new_value;

                return event::Status::Captured;
            }
        }
        Event::Keyboard(keyboard::Event::KeyPressed {
            key_code: keyboard::KeyCode::LShift | keyboard::KeyCode::RShift,
            ..
        }) => {
            state.shift_pressed = true;
            if state.is_dragging {
                state.click_pos_x = cursor_position.x;
                state.click_value = (*value).into();
            }

            return event::Status::Captured;
        }
        Event::Keyboard(keyboard::Event::KeyReleased {
            key_code: keyboard::KeyCode::LShift | keyboard::KeyCode::RShift,
            ..
        }) => {
            state.shift_pressed = false;
            if state.is_dragging {
                let bounds = layout.bounds();
                let step = step.into();
                let start = (*range.start()).into();
                let end = (*range.end()).into();
                let percent = f64::from(cursor_position.x - bounds.x) / f64::from(bounds.width);
                let steps = (percent * (end - start) / step).round();
                let v = steps * step + start;
                let new_value = T::from_f64(v)
                    .unwrap_or_default()
                    .clamp(*range.start(), *range.end());
                shell.publish((on_change)(new_value));
                *value = new_value;
                state.click_pos_x = cursor_position.x;
                state.click_value = (*value).into();
            }

            return event::Status::Captured;
        }
        Event::Keyboard(keyboard::Event::KeyPressed {
            key_code: keyboard::KeyCode::LControl | keyboard::KeyCode::RControl,
            ..
        }) => {
            state.control_pressed = true;

            return event::Status::Captured;
        }
        Event::Keyboard(keyboard::Event::KeyReleased {
            key_code: keyboard::KeyCode::LControl | keyboard::KeyCode::RControl,
            ..
        }) => {
            state.control_pressed = false;

            return event::Status::Captured;
        }
        _ => {}
    }

    event::Status::Ignored
}

/// Draws a [`Slider`].
pub fn draw<T, R>(
    renderer: &mut R,
    layout: Layout<'_>,
    cursor_position: Point,
    state: &State,
    value: T,
    range: &RangeInclusive<T>,
    style_sheet: &dyn StyleSheet<Style = <R::Theme as StyleSheet>::Style>,
    style: &<R::Theme as StyleSheet>::Style,
) where
    T: Into<f64> + Copy,
    R: iced_native::Renderer,
    R::Theme: StyleSheet,
{
    let bounds = layout.bounds();
    let is_mouse_over = bounds.contains(cursor_position);

    let style = if state.is_dragging {
        style_sheet.dragging(style)
    } else if is_mouse_over {
        style_sheet.hovered(style)
    } else {
        style_sheet.active(style)
    };

    let rail_y = bounds.y + (bounds.height / 2.0).round();

    renderer.fill_quad(
        renderer::Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: rail_y - 1.0,
                width: bounds.width,
                height: 2.0,
            },
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
        style.rail_colors.0,
    );

    renderer.fill_quad(
        renderer::Quad {
            bounds: Rectangle {
                x: bounds.x,
                y: rail_y + 1.0,
                width: bounds.width,
                height: 2.0,
            },
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
        Background::Color(style.rail_colors.1),
    );

    let (handle_width, handle_height, handle_border_radius) = match style.handle.shape {
        HandleShape::Circle { radius } => (radius * 2.0, radius * 2.0, radius),
        HandleShape::Rectangle {
            width,
            border_radius,
        } => (f32::from(width), bounds.height, border_radius),
    };

    let value = value.into() as f32;
    let (range_start, range_end) = {
        let (start, end) = range.clone().into_inner();

        (start.into() as f32, end.into() as f32)
    };

    let handle_offset = if range_start >= range_end {
        0.0
    } else {
        bounds.width * (value - range_start) / (range_end - range_start) - handle_width / 2.0
    };

    renderer.fill_quad(
        renderer::Quad {
            bounds: Rectangle {
                x: bounds.x + handle_offset.round(),
                y: rail_y - handle_height / 2.0,
                width: handle_width,
                height: handle_height,
            },
            border_radius: handle_border_radius.into(),
            border_width: style.handle.border_width,
            border_color: style.handle.border_color,
        },
        style.handle.color,
    );
}

/// Computes the current [`mouse::Interaction`] of a [`Slider`].
pub fn mouse_interaction(
    layout: Layout<'_>,
    cursor_position: Point,
    state: &State,
) -> mouse::Interaction {
    let bounds = layout.bounds();
    let is_mouse_over = bounds.contains(cursor_position);

    if state.is_dragging {
        mouse::Interaction::Grabbing
    } else if is_mouse_over {
        mouse::Interaction::Pointer
    } else {
        mouse::Interaction::default()
    }
}

/// The local state of a [`Slider`].
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct State {
    is_dragging: bool,
    shift_pressed: bool,
    control_pressed: bool,
    click_pos_x: f32,
    click_value: f64,
}

impl State {
    /// Creates a new [`State`].
    pub fn new() -> State {
        State::default()
    }
}

impl<'a, T, Message, Renderer> Widget<Message, Renderer> for Slider<'a, T, Message, Renderer>
where
    T: Default + Copy + Into<f64> + Add<Output = T> + Ord + num_traits::FromPrimitive,
    Message: Clone,
    Renderer: iced_native::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(&self, _renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let limits = limits.width(self.width).height(Length::Units(self.height));

        let size = limits.resolve(Size::ZERO);

        layout::Node::new(size)
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        update(
            event,
            layout,
            cursor_position,
            shell,
            tree.state.downcast_mut::<State>(),
            &mut self.value,
            &self.range,
            self.step,
            self.default,
            self.on_change.as_ref(),
            &self.on_release,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        draw(
            renderer,
            layout,
            cursor_position,
            tree.state.downcast_ref::<State>(),
            self.value,
            &self.range,
            theme,
            &self.style,
        )
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        mouse_interaction(layout, cursor_position, tree.state.downcast_ref::<State>())
    }
}

impl<'a, T, Message, Renderer> From<Slider<'a, T, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    T: 'a + Default + Copy + Into<f64> + Add<Output = T> + Ord + num_traits::FromPrimitive,
    Message: 'a + Clone,
    Renderer: 'a + iced_native::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn from(slider: Slider<'a, T, Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(slider)
    }
}
