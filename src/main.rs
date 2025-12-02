mod actor;
mod character;
mod condition;
mod health;
mod initiative_queue;

use std::collections::{BTreeMap, HashMap};

use character::Character;
use character::ID;
use health::Health;
use iced::Element;
use iced::Subscription;
use iced::Task;
use iced::Theme;
use iced::Vector;
use iced::widget::container;
use iced::widget::horizontal_space;
use iced::widget::text_input;
use iced::widget::{column, text};
use iced::widget::{responsive, row};
use initiative_queue::InitiativeQueue;

use iced::widget::pane_grid::{self, PaneGrid};
use iced::widget::{button, center};
use iced::window;
use iced::{Center, Color, Fill, Size};

const PANE_ID_COLOR_UNFOCUSED: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0xC7 as f32 / 255.0,
    0xC7 as f32 / 255.0,
);
const PANE_ID_COLOR_FOCUSED: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0x47 as f32 / 255.0,
    0x47 as f32 / 255.0,
);

#[derive(Debug, Clone)]
enum Message {
    SetHealth(ID, Health),
    OpenWindow,
    WindowOpened(window::Id),
    WindowClosed(window::Id),
    Split(window::Id, pane_grid::Axis, pane_grid::Pane),
    SplitFocused(window::Id, pane_grid::Axis),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    Maximize(pane_grid::Pane),
    Restore,
    Close(pane_grid::Pane),
    CloseFocused,
}

fn main() -> iced::Result {
    iced::daemon(AppState::title, AppState::update, AppState::view)
        .subscription(AppState::subscription)
        .theme(AppState::theme)
        .run_with(|| AppState::new())
}

#[derive(Debug, Clone)]
enum WindowMessage {}

#[derive(Default)]
struct AppState {
    windows: BTreeMap<window::Id, Window>,
    focus: Option<pane_grid::Pane>,
    initiative_queue: Option<InitiativeQueue>,
    actors: Option<HashMap<ID, Character>>,
    party: Option<Vec<ID>>,
}

impl AppState {
    fn new() -> (Self, Task<Message>) {
        let (_id, open) = window::open(window::Settings::default());
        // let (panes, _) = pane_grid::State::new(Pane::new(0));
        let windows: BTreeMap<window::Id, Window> = BTreeMap::new();
        (
            Self {
                windows: BTreeMap::new(),
                focus: None,
                initiative_queue: None,
                actors: None,
                party: None,
            },
            open.map(Message::WindowOpened),
        )
    }

    fn title(&self, window: window::Id) -> String {
        self.windows
            .get(&window)
            .map(|_| "Adventurust".to_string())
            .unwrap_or_default()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match &message {
            Message::SetHealth(character, health) => Task::none(),
            Message::WindowOpened(id) => {
                let window_id = self.windows.len();
                let window = Window::new(window_id);
                self.windows.insert(*id, window);
                let focus_input = text_input::focus(format!("input-{id}"));

                focus_input
            }
            Message::WindowClosed(id) => {
                self.windows.remove(&id);

                if self.windows.is_empty() {
                    iced::exit()
                } else {
                    Task::none()
                }
            }
            Message::OpenWindow => {
                let Some(last_window) = self.windows.keys().last() else {
                    return Task::none();
                };

                window::get_position(*last_window)
                    .then(|last_postition| {
                        let position =
                            last_postition.map_or(window::Position::Default, |last_postition| {
                                window::Position::Specific(last_postition + Vector::new(20.0, 20.0))
                            });
                        let (_id, open) = window::open(window::Settings {
                            position,
                            ..window::Settings::default()
                        });
                        open
                    })
                    .map(Message::WindowOpened)
            }
            Message::Split(window_id, axis, pane) => {
                if let Some(window) = self.windows.get_mut(window_id) {
                    window.update(Message::Split(*window_id, *axis, *pane));
                }
                Task::none()
            }
            Message::SplitFocused(window_id, axis) => {
                if let Some(window) = self.windows.get_mut(window_id) {
                    window.update(Message::SplitFocused(*window_id, *axis));
                }
                Task::none()
            }
            _ => Task::none(),
        }
    }
    fn subscription(&self) -> Subscription<Message> {
        window::close_events().map(Message::WindowClosed)
    }

    fn view(&self, window_id: window::Id) -> Element<'_, Message> {
        if let Some(window) = self.windows.get(&window_id) {
            center(window.view(window_id)).into()
        } else {
            horizontal_space().into()
        }
    }

    fn theme(&self, window: window::Id) -> Theme {
        if let Some(window) = self.windows.get(&window) {
            window.theme.clone()
        } else {
            Theme::default()
        }
    }
}

#[derive(Clone)]
struct Window {
    title: String,
    id: usize,
    panes: pane_grid::State<Pane>,
    panes_created: usize,
    focus: Option<pane_grid::Pane>,
    scale_input: String,
    current_scale: f64,
    theme: Theme,
}

impl Window {
    fn new(id: usize) -> Self {
        let (panes, _) = pane_grid::State::new(Pane::new(id));

        Window {
            title: "Adventurust".to_string(),
            id,
            panes,
            panes_created: 1,
            focus: None,
            scale_input: "1.0".to_string(),
            current_scale: 1.0,
            theme: Theme::Dark,
        }
    }
    fn view(&self, window_id: window::Id) -> Element<Message> {
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
                    view_controls(id, total_panes, pane.is_pinned, is_maximized),
                    button(text("X").size(14))
                        .style(button::danger)
                        .padding(3)
                        .on_press_maybe(if total_panes > 1 && !pane.is_pinned {
                            Some(Message::Close(id))
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
                view_content(window_id, id, total_panes, pane.is_pinned, size)
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
        .on_resize(10, Message::Resized);
        let content = column![new_window_button, pane_grid].spacing(10);

        container(content).into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Split(window_id, axis, pane) => {
                let result = self.panes.split(axis, pane, Pane::new(self.panes_created));

                if let Some((pane, _)) = result {
                    self.focus = Some(pane);
                }

                self.panes_created += 1;
            }
            Message::SplitFocused(windwo_id, axis) => {
                if let Some(pane) = self.focus {
                    let result = self.panes.split(axis, pane, Pane::new(self.panes_created));

                    if let Some((pane, _)) = result {
                        self.focus = Some(pane);
                    }

                    self.panes_created += 1;
                }
            }
            _ => (),
        }
    }
}

#[derive(Clone)]
struct Pane {
    id: usize,
    pub is_pinned: bool,
}

impl Pane {
    fn new(id: usize) -> Self {
        Self {
            id,
            is_pinned: false,
        }
    }
}

fn view_content<'a>(
    window_id: window::Id,
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    size: Size,
) -> Element<'a, Message> {
    let button = |label, message| {
        button(text(label).width(Fill).align_x(Center).size(16))
            .width(Fill)
            .padding(8)
            .on_press(message)
    };

    let controls = column![
        button(
            "Split horizontally",
            Message::Split(window_id, pane_grid::Axis::Horizontal, pane),
        ),
        button(
            "Split vertically",
            Message::Split(window_id, pane_grid::Axis::Vertical, pane),
        )
    ]
    .push_maybe(if total_panes > 1 && !is_pinned {
        Some(button("Close", Message::Close(pane)).style(button::danger))
    } else {
        None
    })
    .spacing(5)
    .max_width(160);

    let content = column![text!("{}x{}", size.width, size.height).size(24), controls,]
        .spacing(10)
        .align_x(Center);

    container(content).center_y(Fill).padding(5).into()
}

fn view_controls<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    is_maximized: bool,
) -> Element<'a, Message> {
    let row = row![].spacing(5).push_maybe(if total_panes > 1 {
        let (content, message) = if is_maximized {
            ("Restore", Message::Restore)
        } else {
            ("Maximize", Message::Maximize(pane))
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
            Some(Message::Close(pane))
        } else {
            None
        });

    row.push(close).into()
}

mod style {
    use iced::widget::container;
    use iced::{Border, Theme};

    pub fn title_bar_active(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            text_color: Some(palette.background.strong.text),
            background: Some(palette.background.strong.color.into()),
            ..Default::default()
        }
    }

    pub fn title_bar_focused(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            text_color: Some(palette.primary.strong.text),
            background: Some(palette.primary.strong.color.into()),
            ..Default::default()
        }
    }

    pub fn pane_active(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            background: Some(palette.background.weak.color.into()),
            border: Border {
                width: 2.0,
                color: palette.background.strong.color,
                ..Border::default()
            },
            ..Default::default()
        }
    }

    pub fn pane_focused(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            background: Some(palette.background.weak.color.into()),
            border: Border {
                width: 2.0,
                color: palette.primary.strong.color,
                ..Border::default()
            },
            ..Default::default()
        }
    }
}
