use crate::ui::WidgetType;

#[derive(Clone)]
pub struct Pane {
    pub id: usize,
    pub widget_state: WidgetType,
    pub is_pinned: bool,
}

impl Pane {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            widget_state: WidgetType::Blank,
            is_pinned: false,
        }
    }
}
