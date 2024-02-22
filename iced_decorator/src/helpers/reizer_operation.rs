
use iced::{advanced::widget::Operation, Point, Rectangle, Size};
use lazy_static::lazy_static;

use crate::resizer::ResizeState;

lazy_static! {
    pub static ref RESIZER_ID: iced::advanced::widget::Id = iced::advanced::widget::Id::new("rsizer");
}
pub struct SetState {
    id: iced::advanced::widget::Id,
    position: Option<Point>,
    size: Option<Size>,
}

impl SetState {
    pub fn with_position(position: Point) -> Self {
        SetState {
            id: RESIZER_ID.clone(),
            position: Some(position),
            size: None,
        }
    }

    pub fn with_size(size: Size) -> Self {
        SetState {
            id: RESIZER_ID.clone(),
            position: None,
            size: Some(size),
        }
    }
}

impl<T> Operation<T> for SetState {
    fn container(
        &mut self,
        _id: Option<&iced::advanced::widget::Id>,
        _bounds: Rectangle,
        operate_on_children: &mut dyn FnMut(&mut dyn Operation<T>),
    ) {
        operate_on_children(self)
    }

    fn custom(&mut self, state: &mut dyn std::any::Any, id: Option<&iced::advanced::widget::Id>) {
        if Some(&self.id) == id {
            if let Some(state) = state.downcast_mut::<ResizeState>() {
                if let Some(position) = self.position {
                    state.window_position = position;
                }
                if let Some(size) = self.size {
                    state.window_size = size;
                }
            }
        }
    }
}
