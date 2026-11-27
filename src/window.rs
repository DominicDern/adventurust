use iced::Element;
use iced::Fill;
use iced::Theme;
use iced::widget::button;
use iced::widget::container;
use iced::widget::pane_grid;
use iced::widget::pane_grid::PaneGrid;
use iced::widget::pick_list;
use iced::widget::{column, text};
use iced::widget::{responsive, row};
use iced::window;

use crate::AppState;
use crate::messages::Message;
use crate::multiwindow::Pane;
use crate::multiwindow::style;
use crate::multiwindow::view_controls;
use crate::widgets::{AddCharacterField, WidgetType, view_content};

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
    AddCharacterChanged(AddCharacterField, String),
    AddCharacter,
    ThemeChanged(Theme),
}

#[derive(Clone)]
pub struct Window {
    title: String,
    panes: pane_grid::State<Pane>,
    panes_created: usize,
    focus: Option<pane_grid::Pane>,
    scale_input: String,
    current_scale: f64,
}

impl Window {
    pub fn new(id: usize) -> Self {
        let (panes, _) = pane_grid::State::new(Pane::new(id));

        Window {
            title: "Adventurust".to_string(),
            panes,
            panes_created: 1,
            focus: None,
            scale_input: "1.0".to_string(),
            current_scale: 1.0,
        }
    }
    pub fn view<'a>(&'a self, state: &'a AppState, window_id: window::Id) -> Element<'a, Message> {
        let palette = state.theme.extended_palette();

        let new_window_button =
            button(text("New Window")).on_press(Message::WindowMessage(WindowMessage::OpenWindow));
        let focus = self.focus;
        let total_panes = self.panes.len();

        let pane_grid = PaneGrid::new(&self.panes, |id, pane, is_maximized| {
            let is_focused = focus == Some(id);

            let title = row![
                "Pane",
                text(pane.id.to_string()).color(if is_focused {
                    palette.background.base.text
                } else {
                    palette.background.weak.text
                }),
            ]
            .spacing(5);

            let title_bar = pane_grid::TitleBar::new(title)
                .controls(pane_grid::Controls::dynamic(
                    view_controls(window_id, id, total_panes, pane.is_pinned, is_maximized),
                    button(text("X").size(14))
                        .style(button::danger)
                        .padding(3)
                        .on_press_maybe(if total_panes > 1 && !pane.is_pinned {
                            Some(Message::WindowMessage(WindowMessage::Close(window_id, id)))
                        } else {
                            None
                        }),
                ))
                .padding(10)
                .style(if is_focused {
                    style::title_bar_focused
                } else {
                    style::title_bar_active
                });

            pane_grid::Content::new(responsive(move |size| {
                view_content(
                    window_id,
                    state,
                    id,
                    total_panes,
                    pane.widget_type,
                    pane.is_pinned,
                    size,
                )
            }))
            .title_bar(title_bar)
            .style(if is_focused {
                style::pane_focused
            } else {
                style::pane_active
            })
        })
        .spacing(10)
        .on_click(|pane| Message::WindowMessage(WindowMessage::Clicked(pane)))
        .on_drag(|pane| Message::WindowMessage(WindowMessage::Dragged(pane)))
        .on_resize(10, move |event| {
            Message::WindowMessage(WindowMessage::Resized(window_id, event))
        });
        let theme_picker = pick_list(Theme::ALL, Some(&state.theme), |theme| {
            Message::WindowMessage(WindowMessage::ThemeChanged(theme))
        });

        let content =
            column![row![new_window_button, theme_picker].spacing(10), pane_grid].spacing(10);

        container(content)
            .width(Fill)
            .height(Fill)
            .padding(10)
            .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::WindowMessage(message) => {
                match message {
                    WindowMessage::Split(_, axis, pane) => {
                        let result = self.panes.split(axis, pane, Pane::new(self.panes_created));

                        if let Some((pane, _)) = result {
                            self.focus = Some(pane);
                        }

                        self.panes_created += 1;
                    }
                    WindowMessage::SplitFocused(_, axis) => {
                        if let Some(pane) = self.focus {
                            let result =
                                self.panes.split(axis, pane, Pane::new(self.panes_created));

                            if let Some((pane, _)) = result {
                                self.focus = Some(pane);
                            }

                            self.panes_created += 1;
                        }
                    }
                    WindowMessage::Resized(_, pane_grid::ResizeEvent { split, ratio }) => {
                        self.panes.resize(split, ratio);
                    }
                    WindowMessage::Maximize(_, pane) => {
                        self.panes.maximize(pane);
                        self.focus = Some(pane);
                    }
                    WindowMessage::Restore(_) => {
                        self.panes.restore();
                    }
                    WindowMessage::Close(_, pane) => {
                        if let Some(fallback) = self.panes.close(pane) {
                            self.focus = Some(fallback.1);
                        } else {
                            // nothing to fall back to, remove focus
                            self.focus = None;
                        }
                    }
                    WindowMessage::WidgetChange(_, pane, new_widget_type) => {
                        if let Some(pane) = self.panes.get_mut(pane) {
                            pane.widget_type = new_widget_type;
                        }
                    }
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
    }
}
