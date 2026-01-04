use crate::ui::pane::Pane;
use crate::ui::widgets::WidgetType;
use crate::ui::{Message, SessionState, style};
use crate::ui::{PANE_ID_COLOR_FOCUSED, PANE_ID_COLOR_UNFOCUSED};

use iced::Element;
use iced::widget::{
    PaneGrid, button, column, container, pane_grid, pick_list, responsive, row, text,
};
use iced::{Center, Fill, Size, Theme, Top, window};

#[derive(Clone)]
pub struct Window {
    title: String,
    panes: pane_grid::State<Pane>,
    panes_created: usize,
    focus: Option<pane_grid::Pane>,
    scale_input: String,
    current_scale: f64,
    pub theme: Theme,
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
            theme: Theme::Dark,
        }
    }
    pub fn view(&self, state: SessionState, window_id: window::Id) -> Element<Message> {
        let new_window_button = button(text("New Window")).on_press(Message::OpenWindow);
        let focus = self.focus;
        let total_panes = self.panes.len();

        let pane_grid = PaneGrid::new(&self.panes, |id, pane, is_maximized| {
            let is_focused = focus == Some(id);

            let title = row![
                "Pane",
                text(pane.id.to_string()).color(if is_focused {
                    PANE_ID_COLOR_FOCUSED
                } else {
                    PANE_ID_COLOR_UNFOCUSED
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
                            Some(Message::Close(window_id, id))
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

            let state_clone = state.clone();

            pane_grid::Content::new(responsive(move |size| {
                view_content(
                    state_clone.clone(),
                    window_id,
                    id,
                    total_panes,
                    pane.widget_state,
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
        .on_click(Message::Clicked)
        .on_drag(Message::Dragged)
        .on_resize(10, move |event| Message::Resized(window_id, event));
        let content = column![new_window_button, pane_grid].spacing(10);

        container(content).width(Fill).height(Fill).into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Split(_, axis, pane) => {
                let result = self.panes.split(axis, pane, Pane::new(self.panes_created));

                if let Some((pane, _)) = result {
                    self.focus = Some(pane);
                }

                self.panes_created += 1;
            }
            Message::SplitFocused(_, axis) => {
                if let Some(pane) = self.focus {
                    let result = self.panes.split(axis, pane, Pane::new(self.panes_created));

                    if let Some((pane, _)) = result {
                        self.focus = Some(pane);
                    }

                    self.panes_created += 1;
                }
            }
            Message::Resized(_, pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
            Message::Maximize(_, pane) => {
                self.panes.maximize(pane);
                self.focus = Some(pane);
            }
            Message::Restore(_) => {
                self.panes.restore();
            }
            Message::Close(_, pane) => {
                if let Some(fallback) = self.panes.close(pane) {
                    self.focus = Some(fallback.1);
                } else {
                    // nothing to fall back to, remove focus
                    self.focus = None;
                }
            }
            Message::WidgetChange(_, pane, new_widget_state) => {
                if let Some(pane) = self.panes.get_mut(pane) {
                    pane.widget_state = new_widget_state;
                }
            }
            _ => todo!(),
        }
    }
}
fn view_controls<'a>(
    window_id: window::Id,
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    is_maximized: bool,
) -> Element<'a, Message> {
    let row = row![].spacing(5).push_maybe(if total_panes > 1 {
        let (content, message) = if is_maximized {
            ("Restore", Message::Restore(window_id))
        } else {
            ("Maximize", Message::Maximize(window_id, pane))
        };

        Some(
            button(text(content).size(14))
                .style(button::secondary)
                .padding(3)
                .on_press(message),
        )
    } else {
        None
    });

    let close = button(text("Close").size(14))
        .style(button::danger)
        .padding(3)
        .on_press_maybe(if total_panes > 1 && !is_pinned {
            Some(Message::Close(window_id, pane))
        } else {
            None
        });

    row.push(close).into()
}
fn view_content<'a>(
    state: SessionState,
    window_id: window::Id,
    pane: pane_grid::Pane,
    total_panes: usize,
    widget_type: WidgetType,
    is_pinned: bool,
    _size: Size,
) -> Element<'a, Message> {
    let button = |label, message| {
        button(text(label).width(Fill).align_x(Center).size(16))
            .width(Fill)
            .padding(8)
            .on_press(message)
    };

    let content: Element<Message> = match widget_type {
        WidgetType::Blank => row![text("New widget")].into(),

        WidgetType::InitiativeQueue => {
            // Build a column with character names and initiative rolls
            let mut column_content = column![];

            if let (Some(current_queue), Some(actors)) = (&state.current_queue, &state.actors) {
                if let Some(queue) = state.queues.get(current_queue) {
                    for (i, (id, roll)) in queue.get_queue().unwrap_or_default().iter().enumerate()
                    {
                        if let Some(actor) = actors.get(id) {
                            let mut label = text(format!("{}: {}", actor.name, roll)).size(16);

                            // Highlight the current turn
                            if i == queue.position {
                                label = label.color(iced::Color::from_rgb(1.0, 0.8, 0.0)); // gold
                            }

                            column_content = column_content.push(label);
                        }
                    }
                }
            } else {
                column_content = column_content.push(text("No initiative data"));
            }

            column_content.spacing(5).into()
        }

        WidgetType::ActiveSheet => row![text("ActiveSheet")].into(),

        WidgetType::AddCharacter {
            add_to_current_initiative: add_to_current_iniative,
        } => {
            // TODO add a way to add a default character and input a custom character.
            let add_character_button =
                button("Add default character", Message::DefaultCharacterAdded);
            row![column![text("Add Character"), add_character_button]].into()
        }
    };

    let controls = column![
        row![
            button(
                "Split horizontally",
                Message::Split(window_id, pane_grid::Axis::Horizontal, pane),
            ),
            button(
                "Split vertically",
                Message::Split(window_id, pane_grid::Axis::Vertical, pane)
            ),
            pick_list(WidgetType::ALL, Some(widget_type), move |selected| {
                Message::WidgetChange(window_id, pane, selected)
            })
        ]
        .align_y(Center),
        content
    ]
    .push_maybe(if total_panes > 1 && !is_pinned {
        Some(button("Close", Message::Close(window_id, pane)).style(button::danger))
    } else {
        None
    })
    .spacing(5);

    container(controls)
        .center_y(Fill)
        .padding(5)
        .align_y(Top)
        .into()
}
