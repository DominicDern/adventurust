use crate::actor::Actor;
use crate::character::{Character, ID};
use crate::character_message::CharacterMessage;
use crate::health::Health;
use crate::initiative_queue::{InitiativeMessage, InitiativeQueue};
use crate::messages::Message;
use crate::widgets::WidgetType;
use crate::widgets::{AddCharacterField, AddCharacterState};
use crate::window::{Window, WindowMessage};

use core::clone::Clone;
use std::collections::{BTreeMap, HashMap};

use iced::Element;
use iced::Subscription;
use iced::Task;
use iced::Theme;
use iced::Vector;
use iced::widget::horizontal_space;
use iced::widget::row;
use iced::widget::text;
use iced::widget::text_input;

use iced::Color;
use iced::widget::pane_grid;
use iced::widget::{button, center};
use iced::window;

pub const PANE_ID_COLOR_UNFOCUSED: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0xC7 as f32 / 255.0,
    0xC7 as f32 / 255.0,
);
pub const PANE_ID_COLOR_FOCUSED: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0x47 as f32 / 255.0,
    0x47 as f32 / 255.0,
);

#[derive(Default, Clone)]
pub struct AppState {
    windows: BTreeMap<window::Id, Window>,
    focus: Option<pane_grid::Pane>,
    pub initiative_queue: InitiativeQueue,
    pub actors: HashMap<ID, Character>,
    pub party: Vec<ID>,
    pub add_character: AddCharacterState,
}

impl AppState {
    pub fn new() -> (Self, Task<Message>) {
        let (_id, open) = window::open(window::Settings::default());
        // let (panes, _) = pane_grid::State::new(Pane::new(0));
        (
            Self {
                windows: BTreeMap::new(),
                focus: None,
                initiative_queue: InitiativeQueue::default(),
                actors: HashMap::new(),
                party: Vec::new(),
                add_character: AddCharacterState::default(),
            },
            open.map(|id| Message::WindowMessage(WindowMessage::WindowOpened(id))),
        )
    }

    pub fn add_character(&mut self, character: Character, in_party: bool) {
        self.actors
            .insert(character.clone().get_id(), character.clone());

        if in_party {
            self.party.push(character.clone().get_id());
        }
    }

    pub fn add_to_initiative(&mut self, id: ID, initiative: u16, in_battle: bool) {
        self.initiative_queue.add(id, initiative, in_battle);
    }

    pub fn title(&self, window: window::Id) -> String {
        self.windows
            .get(&window)
            .map(|_| "Adventurust".to_string())
            .unwrap_or_default()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match &message {
            Message::InitiativeMessage(message) => match message {
                InitiativeMessage::AddCharacter(character) => {
                    self.add_character(character.clone(), true);
                    Task::none()
                }
            },
            Message::CharacterMessage(message) => match message {
                CharacterMessage::SetHealth(character, health) => {
                    if let Some(actor) = self.actors.get_mut(character) {
                        actor.set_health(*health)
                    }
                    Task::none()
                }
            },
            Message::WindowMessage(message) => match message {
                WindowMessage::WindowOpened(id) => {
                    let window_id = self.windows.len();
                    let window = Window::new(window_id);
                    self.windows.insert(*id, window);
                    text_input::focus(format!("input-{id}"))
                }
                WindowMessage::WindowClosed(id) => {
                    self.windows.remove(id);

                    if self.windows.is_empty() {
                        iced::exit()
                    } else {
                        Task::none()
                    }
                }
                WindowMessage::OpenWindow => {
                    let Some(last_window) = self.windows.keys().last() else {
                        return Task::none();
                    };

                    window::get_position(*last_window)
                        .then(|last_postition| {
                            let position = last_postition.map_or(
                                window::Position::Default,
                                |last_postition| {
                                    window::Position::Specific(
                                        last_postition + Vector::new(20.0, 20.0),
                                    )
                                },
                            );
                            let (_id, open) = window::open(window::Settings {
                                position,
                                ..window::Settings::default()
                            });
                            open
                        })
                        .map(|id| Message::WindowMessage(WindowMessage::WindowOpened(id)))
                }
                WindowMessage::Split(window_id, axis, pane) => {
                    if let Some(window) = self.windows.get_mut(window_id) {
                        window.update(Message::WindowMessage(WindowMessage::Split(
                            *window_id, *axis, *pane,
                        )));
                    }
                    Task::none()
                }
                WindowMessage::SplitFocused(window_id, axis) => {
                    if let Some(window) = self.windows.get_mut(window_id) {
                        window.update(Message::WindowMessage(WindowMessage::SplitFocused(
                            *window_id, *axis,
                        )));
                    }
                    Task::none()
                }
                WindowMessage::Resized(window_id, resize_event) => {
                    if let Some(window) = self.windows.get_mut(window_id) {
                        window.update(Message::WindowMessage(WindowMessage::Resized(
                            *window_id,
                            *resize_event,
                        )));
                    }
                    Task::none()
                }
                WindowMessage::Maximize(window_id, pane) => {
                    if let Some(window) = self.windows.get_mut(window_id) {
                        window.update(Message::WindowMessage(WindowMessage::Maximize(
                            *window_id, *pane,
                        )));
                    }
                    Task::none()
                }
                WindowMessage::Restore(window_id) => {
                    if let Some(window) = self.windows.get_mut(window_id) {
                        window.update(Message::WindowMessage(WindowMessage::Restore(*window_id)));
                    }
                    Task::none()
                }
                WindowMessage::Close(window_id, pane) => {
                    if let Some(window) = self.windows.get_mut(window_id) {
                        window.update(Message::WindowMessage(WindowMessage::Close(
                            *window_id, *pane,
                        )));
                    }
                    Task::none()
                }
                WindowMessage::WidgetChange(window_id, pane, widget_type) => {
                    if let Some(window) = self.windows.get_mut(window_id) {
                        window.update(Message::WindowMessage(WindowMessage::WidgetChange(
                            *window_id,
                            *pane,
                            *widget_type,
                        )));
                    }
                    Task::none()
                }
                WindowMessage::AddCharacterChanged(field, new_input) => match field {
                    AddCharacterField::Name => {
                        self.add_character.name = new_input.clone();
                        Task::none()
                    }
                    AddCharacterField::Roll => {
                        self.add_character.roll = new_input.clone();
                        Task::none()
                    }
                },
                WindowMessage::AddCharacter => {
                    let name = self.add_character.name.clone();
                    let temp_health = Health {
                        primary_hp: 0,
                        primary_max_hp: 0,
                        secondary_hp: 0,
                        secondary_max_hp: 0,
                        temp_hp: 0,
                        temp_max_hp: 0,
                        wild_shape: false,
                    };
                    let character = Character::new(name, temp_health, 0, Vec::new(), Vec::new());

                    self.actors
                        .insert(character.clone().get_id(), character.clone());
                    // Assumed if empty -> not adding to initiative, will later add a button that
                    // turns the input on and off via a tick box

                    Task::none()
                }
                _ => Task::none(),
            },
        }
    }
    pub fn subscription(&self) -> Subscription<Message> {
        window::close_events().map(|id| Message::WindowMessage(WindowMessage::WindowClosed(id)))
    }

    pub fn view(&self, window_id: window::Id) -> Element<'_, Message> {
        if let Some(window) = self.windows.get(&window_id) {
            center(window.view(self, window_id)).into()
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

#[derive(Clone)]
pub struct Pane {
    pub id: usize,
    pub widget_type: WidgetType,
    pub is_pinned: bool,
}

impl Pane {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            widget_type: WidgetType::Blank,
            is_pinned: false,
        }
    }
}

pub fn view_controls<'a>(
    window_id: window::Id,
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    is_maximized: bool,
) -> Element<'a, Message> {
    let row = row![].spacing(5).push_maybe(if total_panes > 1 {
        let (content, message) = if is_maximized {
            (
                "Restore",
                Message::WindowMessage(WindowMessage::Restore(window_id)),
            )
        } else {
            (
                "Maximize",
                Message::WindowMessage(WindowMessage::Maximize(window_id, pane)),
            )
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
            Some(Message::WindowMessage(WindowMessage::Close(
                window_id, pane,
            )))
        } else {
            None
        });

    row.push(close).into()
}

pub mod style {
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
