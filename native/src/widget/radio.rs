//! Create choices using radio buttons.
use crate::{
    input::{mouse, ButtonState},
    layout, row, text, Align, Clipboard, Element, Event, Font, Hasher,
    HorizontalAlignment, Layout, Length, Point, Rectangle, Row, Text,
    VerticalAlignment, Widget,
};

use std::hash::Hash;

/// A circular button representing a choice.
///
/// # Example
/// ```
/// # type Radio<Message> =
/// #     iced_native::Radio<Message, iced_native::renderer::Null>;
/// #
/// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// pub enum Choice {
///     A,
///     B,
/// }
///
/// #[derive(Debug, Clone, Copy)]
/// pub enum Message {
///     RadioSelected(Choice),
/// }
///
/// let selected_choice = Some(Choice::A);
///
/// Radio::new(Choice::A, "This is A", selected_choice, Message::RadioSelected);
///
/// Radio::new(Choice::B, "This is B", selected_choice, Message::RadioSelected);
/// ```
///
/// ![Radio buttons drawn by `iced_wgpu`](https://github.com/hecrj/iced/blob/7760618fb112074bc40b148944521f312152012a/docs/images/radio.png?raw=true)
#[allow(missing_debug_implementations)]
pub struct Radio<Message, Renderer: self::Renderer> {
    is_selected: bool,
    on_click: Box<dyn Fn() -> Message>,
    label: String,
    style: Renderer::Style,
}

impl<Message, Renderer: self::Renderer> Radio<Message, Renderer> {
    /// Creates a new [`Radio`] button.
    ///
    /// It expects:
    ///   * the value related to the [`Radio`] button
    ///   * the label of the [`Radio`] button
    ///   * the current selected value
    ///   * a function that will be called when the [`Radio`] is selected. It
    ///   receives the value of the radio and must produce a `Message`.
    ///
    /// [`Radio`]: struct.Radio.html
    pub fn new<F, V>(value: V, label: &str, selected: Option<V>, f: F) -> Self
    where
        V: 'static + Eq + Copy,
        F: 'static + Fn(V) -> Message,
    {
        Radio {
            is_selected: Some(value) == selected,
            on_click: Box::new(move || f(value)),
            label: String::from(label),
            style: Renderer::Style::default(),
        }
    }

    /// Sets the style of the [`Radio`] button.
    ///
    /// [`Radio`]: struct.Radio.html
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for Radio<Message, Renderer>
where
    Renderer: self::Renderer + text::Renderer + row::Renderer,
{
    fn width(&self) -> Length {
        Length::Fill
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = self::Renderer::default_size(renderer);

        Row::<(), Renderer>::new()
            .width(Length::Fill)
            .spacing(15)
            .align_items(Align::Center)
            .push(
                Row::new()
                    .width(Length::Units(size as u16))
                    .height(Length::Units(size as u16)),
            )
            .push(Text::new(&self.label))
            .layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) {
        match event {
            Event::Mouse(mouse::Event::Input {
                button: mouse::Button::Left,
                state: ButtonState::Pressed,
            }) => {
                if layout.bounds().contains(cursor_position) {
                    messages.push((self.on_click)());
                }
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Renderer::Output {
        let bounds = layout.bounds();
        let mut children = layout.children();

        let radio_layout = children.next().unwrap();
        let label_layout = children.next().unwrap();
        let radio_bounds = radio_layout.bounds();

        let label = text::Renderer::draw(
            renderer,
            defaults,
            label_layout.bounds(),
            &self.label,
            text::Renderer::default_size(renderer),
            Font::Default,
            None,
            HorizontalAlignment::Left,
            VerticalAlignment::Center,
        );

        let is_mouse_over = bounds.contains(cursor_position);

        self::Renderer::draw(
            renderer,
            radio_bounds,
            self.is_selected,
            is_mouse_over,
            label,
            &self.style,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        self.label.hash(state);
    }
}

/// The renderer of a [`Radio`] button.
///
/// Your [renderer] will need to implement this trait before being
/// able to use a [`Radio`] button in your user interface.
///
/// [`Radio`]: struct.Radio.html
/// [renderer]: ../../renderer/index.html
pub trait Renderer: crate::Renderer {
    /// The style supported by this renderer.
    type Style: Default;

    /// Returns the default size of a [`Radio`] button.
    ///
    /// [`Radio`]: struct.Radio.html
    fn default_size(&self) -> u32;

    /// Draws a [`Radio`] button.
    ///
    /// It receives:
    ///   * the bounds of the [`Radio`]
    ///   * whether the [`Radio`] is selected or not
    ///   * whether the mouse is over the [`Radio`] or not
    ///   * the drawn label of the [`Radio`]
    ///
    /// [`Radio`]: struct.Radio.html
    fn draw(
        &mut self,
        bounds: Rectangle,
        is_selected: bool,
        is_mouse_over: bool,
        label: Self::Output,
        style: &Self::Style,
    ) -> Self::Output;
}

impl<'a, Message, Renderer> From<Radio<Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'static + self::Renderer + row::Renderer + text::Renderer,
    Message: 'static,
{
    fn from(radio: Radio<Message, Renderer>) -> Element<'a, Message, Renderer> {
        Element::new(radio)
    }
}
