use iced::{
    advanced::{layout, widget::Tree, Text, Widget},
    alignment, mouse, touch, Element, Event, Length, Point, Rectangle, Size,
};

/// An amount of empty space.
///
/// It can be useful if you want to fill some space with nothing.
#[derive(Debug)]
pub struct DragWindow<'a, Message> {
    width: Length,
    height: Length,
    message: Message,
    title: Option<&'a str>,
}

impl<'a, Message> DragWindow<'a, Message> {
    /// Creates an amount of empty [`Space`] with the given width and height.
    pub fn new(width: impl Into<Length>, height: impl Into<Length>, message: Message) -> Self {
        DragWindow {
            width: width.into(),
            height: height.into(),
            message,
            title: None,
        }
    }

    /// Creates an amount of horizontal [`Space`].
    pub fn with_width(width: impl Into<Length>, message: Message) -> Self {
        DragWindow {
            width: width.into(),
            height: Length::Fill,
            message,
            title: None,
        }
    }

    /// Creates an amount of vertical [`Space`].
    pub fn with_height(height: impl Into<Length>, message: Message) -> Self {
        DragWindow {
            width: Length::Fill,
            height: height.into(),
            message,
            title: None,
        }
    }

    pub fn set_title(mut self, title: Option<&'a str>) -> Self {
        self.title = title;
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for DragWindow<'a, Message>
where
    Message: Clone,
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    fn size(&self) -> iced::Size<Length> {
        iced::Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);

        layout::Node::new(limits.resolve(self.width, self.height, Size::ZERO))
    }

    fn on_event(
        &mut self,
        _state: &mut Tree,
        event: iced::Event,
        layout: layout::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &iced::Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(layout.bounds()) {
                    shell.publish(self.message.clone())
                }
            }
            _ => {}
        }
        iced::advanced::graphics::core::event::Status::Ignored
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: layout::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        if let Some(title) = &self.title {
            let bounds = layout.bounds();

            let horizontal_alignment = alignment::Horizontal::Center;
            let vertical_alignment = alignment::Vertical::Center;

            let x = match horizontal_alignment {
                alignment::Horizontal::Left => bounds.x,
                alignment::Horizontal::Center => bounds.center_x(),
                alignment::Horizontal::Right => bounds.x + bounds.width,
            };

            let y = match vertical_alignment {
                alignment::Vertical::Top => bounds.y,
                alignment::Vertical::Center => bounds.center_y(),
                alignment::Vertical::Bottom => bounds.y + bounds.height,
            };

            renderer.fill_text(
                Text {
                    content: title,
                    bounds: Size::new(bounds.width, bounds.height),
                    size: renderer.default_size(),

                    line_height: Default::default(),
                    shaping: iced::widget::text::Shaping::Basic,
                    font: renderer.default_font(),
                    horizontal_alignment,
                    vertical_alignment,
                },
                Point::new(0.0, 0.0),
                style.text_color,
                bounds,
            );
        }
    }
}

impl<'a, Message, Theme, Renderer> From<DragWindow<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    fn from(space: DragWindow<'a, Message>) -> Element<'a, Message, Theme, Renderer> {
        Element::new(space)
    }
}
