use std::collections::{BTreeMap, HashMap};

use iced::widget::{center, horizontal_space, pane_grid, text_input};
use iced::window;
use iced::{Element, Task, Theme, Vector};

use crate::character::{Character, ID};
use crate::health::Health;
use crate::ui::Subscription;
use crate::ui::WidgetType;
use crate::ui::app_window::Window;
use crate::ui::session::SessionState;

#[derive(Debug, Clone)]
pub enum Message {
    SetHealth(ID, Health),
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
    DefaultCharacterAdded,
    CharacterAddedToCurrentQueue(ID, u16, bool),
}

#[derive(Default)]
pub struct AppState {
    windows: BTreeMap<window::Id, Window>,
    focus: Option<pane_grid::Pane>,
    session: SessionState,
}

impl AppState {
    pub fn new() -> (Self, Task<Message>) {
        let (_id, open) = window::open(window::Settings::default());
        let session = SessionState::new(None);
        // let (panes, _) = pane_grid::State::new(Pane::new(0));
        (
            Self {
                windows: BTreeMap::new(),
                focus: None,
                session,
            },
            open.map(Message::WindowOpened),
        )
    }

    pub fn title(&self, window: window::Id) -> String {
        self.windows
            .get(&window)
            .map(|_| "Adventurust".to_string())
            .unwrap_or_default()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match &message {
            Message::SetHealth(character, health) => Task::none(),
            Message::WindowOpened(id) => {
                let window_id = self.windows.len();
                let window = Window::new(window_id);
                self.windows.insert(*id, window);
                text_input::focus(format!("input-{id}"))
            }
            Message::WindowClosed(id) => {
                self.windows.remove(id);

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
            Message::Resized(window_id, resize_event) => {
                if let Some(window) = self.windows.get_mut(window_id) {
                    window.update(Message::Resized(*window_id, *resize_event));
                }
                Task::none()
            }
            Message::Maximize(window_id, pane) => {
                if let Some(window) = self.windows.get_mut(window_id) {
                    window.update(Message::Maximize(*window_id, *pane));
                }
                Task::none()
            }
            Message::Restore(window_id) => {
                if let Some(window) = self.windows.get_mut(window_id) {
                    window.update(Message::Restore(*window_id));
                }
                Task::none()
            }
            Message::Close(window_id, pane) => {
                if let Some(window) = self.windows.get_mut(window_id) {
                    window.update(Message::Close(*window_id, *pane));
                }
                Task::none()
            }
            Message::WidgetChange(window_id, pane, widget_type) => {
                if let Some(window) = self.windows.get_mut(window_id) {
                    window.update(Message::WidgetChange(*window_id, *pane, *widget_type));
                }
                Task::none()
            }
            Message::DefaultCharacterAdded => {
                match &mut self.session.actors {
                    Some(actors) => {
                        let default_character = Character::default();
                        actors.insert(default_character.id, default_character);
                    }
                    None => {
                        let mut actors = HashMap::new();
                        let default_character = Character::default();
                        actors.insert(default_character.id, default_character);
                    }
                }

                Task::none()
            }
            Message::CharacterAddedToCurrentQueue(character, roll, in_battle) => {
                self.session
                    .add_character_to_current_queue(character, roll, in_battle);
                Task::none()
            }
            _ => Task::none(),
        }
    }
    pub fn subscription(&self) -> Subscription<Message> {
        window::close_events().map(Message::WindowClosed)
    }

    pub fn view(&self, window_id: window::Id) -> Element<'_, Message> {
        if let Some(window) = self.windows.get(&window_id) {
            center(window.view(self.session.clone(), window_id)).into()
        } else {
            horizontal_space().into()
        }
    }

    pub fn theme(&self, window: window::Id) -> Theme {
        if let Some(window) = self.windows.get(&window) {
            window.theme.clone()
        } else {
            Theme::default()
        }
    }
}
