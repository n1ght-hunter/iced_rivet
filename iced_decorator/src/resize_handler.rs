use std::io::{stdout, Write};

use iced::{
    advanced::{
        layout, mouse, overlay, renderer,
        widget::{tree, Operation, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    event, touch, window, Color, Element, Event, Length, Point, Rectangle, Size, Vector,
};

#[derive(Debug, Clone)]
pub enum ResizeEvent {
    ResizeXY(Size),
    ResizeWindow(Rectangle),
}

#[derive(Debug, Clone, PartialEq)]
enum Dragging {
    HorizontalRight,
    HorizontalLeft,
    VerticalTop,
    VerticalBottom,
    Both,
    None,
}

#[derive(Debug, Clone)]
struct ResizeState {
    dragging: Dragging,
    window_size: Size,
    window_position: Point,
    show: bool,
}

impl Default for ResizeState {
    fn default() -> Self {
        Self {
            dragging: Dragging::None,
            window_size: Size::new(0.0, 0.0),
            window_position: Point::new(0.0, 0.0),
            show: true,
        }
    }
}

pub fn resizer<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    event_handler: impl 'a + Fn(ResizeEvent) -> Message,
) -> Resizer<'a, Message, Theme, Renderer> {
    Resizer::new(content, event_handler)
}

pub struct Resizer<'a, Message, Theme, Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
    event_handler: Box<dyn Fn(ResizeEvent) -> Message + 'a>,
}

impl<'a, Message, Theme, Renderer> Resizer<'a, Message, Theme, Renderer> {
    pub fn new<F: 'a + Fn(ResizeEvent) -> Message>(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        event_handler: F,
    ) -> Self {
        let content = content.into();
        Self {
            content,
            event_handler: Box::new(event_handler),
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Resizer<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: iced::advanced::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<ResizeState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(ResizeState::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = limits.max();

        layout::Node::with_children(
            size,
            vec![self
                .content
                .as_widget()
                .layout(&mut tree.children[0], renderer, limits)],
        )
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.content.as_widget().operate(
                &mut tree.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        let mut state = tree.state.downcast_mut::<ResizeState>();
        let mut bounds = Rectangle::new(state.window_position, state.window_size);

        let cursor_position = cursor.position();

        match event {
            Event::Window(_, ref event) => match event {
                window::Event::Resized { width, height } => {
                    state.window_size = Size::new(width.clone() as f32, height.clone() as f32);
                }
                window::Event::Moved { x, y } => {
                    state.window_position = Point::new(x.clone() as f32, y.clone() as f32);
                }
                _ => {}
            },
            Event::Mouse(mouse::Event::CursorMoved {
                position: cursor_position,
            })
            | Event::Touch(touch::Event::FingerMoved {
                position: cursor_position,
                ..
            }) => {
                let mut size = bounds.size();
                let Point { x, y } = cursor_position;

                match state.dragging {
                    Dragging::HorizontalRight => {
                        if x < bounds.width || x > bounds.width {
                            size.width = x;
                            shell.publish((self.event_handler)(ResizeEvent::ResizeXY(size)));
                        }
                    }
                    Dragging::VerticalBottom => {
                        if y < bounds.height || y > bounds.y {
                            size.height = y;
                            shell.publish((self.event_handler)(ResizeEvent::ResizeXY(size)));
                        }
                    }
                    Dragging::HorizontalLeft => {
                        if x < bounds.x || x > bounds.x {
                            bounds.width = bounds.width - x;
                            bounds.x = bounds.x + x;
                            shell.publish((self.event_handler)(ResizeEvent::ResizeWindow(bounds)));
                        }
                    }
                    Dragging::VerticalTop => {
                        if y < bounds.y || y > bounds.y {
                            bounds.height = bounds.height - y;
                            bounds.y = bounds.y + y;
                            shell.publish((self.event_handler)(ResizeEvent::ResizeWindow(bounds)));
                        }
                    }
                    _ => {}
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                if state.dragging != Dragging::None {
                    state.dragging = Dragging::None;
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if state.show & cursor_position.is_some() {
                    let bounds = layout.bounds();

                    let Point { x, y } = cursor_position.unwrap();

                    if x < bounds.x + 5.0 {
                        state.dragging = Dragging::HorizontalLeft;
                        return event::Status::Captured;
                    } else if x > bounds.x + bounds.width - 5.0 {
                        state.dragging = Dragging::HorizontalRight;
                        return event::Status::Captured;
                    } else if y < bounds.y + 5.0 {
                        state.dragging = Dragging::VerticalTop;
                        return event::Status::Captured;
                    } else if y > bounds.y + bounds.height - 5.0 {
                        state.dragging = Dragging::VerticalBottom;
                        return event::Status::Captured;
                    }
                }
            }
            _ => {}
        }

        self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let content_layout = layout.children().next().unwrap();
        renderer.fill_quad(
            renderer::Quad {
                bounds: bounds,
                ..Default::default()
            },
            Color::TRANSPARENT,
        );

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            content_layout,
            cursor,
            &viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<ResizeState>();
        let cursor_position = cursor.position();

        if state.show & cursor_position.is_some() {
            let bounds = layout.bounds();

            let Point { x, y } = cursor_position.unwrap();

            if x < bounds.x + 5.0 {
                return mouse::Interaction::ResizingHorizontally;
            }
            if x > bounds.x + bounds.width - 5.0 {
                return mouse::Interaction::ResizingHorizontally;
            }
            if y < bounds.y + 5.0 {
                return mouse::Interaction::ResizingVertically;
            }
            if y > bounds.y + bounds.height - 5.0 {
                return mouse::Interaction::ResizingVertically;
            }
        }

        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
            translation,
        )
    }
}

impl<'a, Message, Theme, Renderer> From<Resizer<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced::advanced::Renderer + 'a,
    Theme: 'a,
{
    fn from(resizer: Resizer<'a, Message, Theme, Renderer>) -> Self {
        Self::new(resizer)
    }
}
