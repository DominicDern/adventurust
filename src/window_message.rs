use iced::widget::pane_grid;
use iced::window;

use crate::widgets::WidgetType;

#[derive(Debug, Clone)]
pub enum WindowMessage {
    OpenWindow,
    WindowOpened(window::Id),
    WindowClosed(window::Id),
    Split(window::Id, pane_grid::Axis, pane_grid::Pane),
    SplitFocused(window::Id, pane_grid::Axis),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(window::Id, pane_grid::ResizeEvent),
    Maximize(window::Id, pane_grid::Pane),
    Restore(window::Id),
    Close(window::Id, pane_grid::Pane),
    CloseFocused,
    WidgetChange(window::Id, pane_grid::Pane, WidgetType),
}
