use std::collections::{BTreeMap, HashMap};

use iced::widget::center;
use iced::widget::{button, horizontal_space, pane_grid, row, text, text_input};
use iced::window;
use iced::{Element, Subscription, Task, Theme, Vector};

use crate::actor::Actor;
use crate::character::{Character, ID};
use crate::character_message::CharacterMessage;
use crate::health::Health;
use crate::initiative_queue::{InitiativeMessage, InitiativeQueue};
use crate::messages::Message;
use crate::widgets::WidgetType;
use crate::widgets::{AddCharacterField, AddCharacterState};
use crate::window::{Window, WindowMessage};

#[derive(Clone)]
pub struct AppState {
    windows: BTreeMap<window::Id, Window>,
    focus: Option<pane_grid::Pane>,
    pub theme: Theme,
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
                theme: Theme::Dark,
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
                WindowMessage::Dragged(window_id, event) => {
                    if let Some(window) = self.windows.get_mut(window_id) {
                        window.update(Message::WindowMessage(WindowMessage::Dragged(
                            *window_id, *event,
                        )));
                    }
                    Task::none()
                }
                WindowMessage::AddCharacterChanged(field, new_input) => {
                    let new_input = new_input.clone();
                    match field {
                        AddCharacterField::Name => {
                            self.add_character.name = new_input;
                        }
                        AddCharacterField::AC => match new_input.parse::<u16>() {
                            Ok(_) => {
                                self.add_character.ac = new_input;
                            }
                            Err(_) => {
                                if new_input.is_empty() {
                                    self.add_character.ac = "".to_string();
                                }
                            }
                        },
                        AddCharacterField::InParty => {
                            self.add_character.in_party = !self.add_character.in_party;
                        }
                        AddCharacterField::InBattle => {
                            self.add_character.in_battle = !self.add_character.in_battle;
                        }
                        AddCharacterField::Initiative => match new_input.parse::<u16>() {
                            Ok(_) => {
                                self.add_character.initiative = new_input;
                            }
                            Err(_) => {
                                if new_input.is_empty() {
                                    self.add_character.initiative = "".to_string();
                                }
                            }
                        },
                        AddCharacterField::HP => match new_input.parse::<u16>() {
                            Ok(_) => {
                                self.add_character.max_hp = new_input;
                            }
                            Err(_) => {
                                if new_input.is_empty() {
                                    self.add_character.max_hp = "".to_string();
                                }
                            }
                        },
                        AddCharacterField::Speed => match new_input.parse::<u16>() {
                            Ok(_) => {
                                self.add_character.speed = new_input;
                            }
                            Err(_) => {
                                if new_input.is_empty() {
                                    self.add_character.speed = "".to_string();
                                }
                            }
                        },
                        AddCharacterField::Str => match new_input.parse::<u16>() {
                            Ok(_) => {
                                self.add_character.str = new_input;
                            }
                            Err(_) => {
                                if new_input.is_empty() {
                                    self.add_character.str = "".to_string();
                                }
                            }
                        },
                        AddCharacterField::Dex => match new_input.parse::<u16>() {
                            Ok(_) => {
                                self.add_character.dex = new_input;
                            }
                            Err(_) => {
                                if new_input.is_empty() {
                                    self.add_character.dex = "".to_string();
                                }
                            }
                        },
                        AddCharacterField::Con => match new_input.parse::<u16>() {
                            Ok(_) => {
                                self.add_character.con = new_input;
                            }
                            Err(_) => {
                                if new_input.is_empty() {
                                    self.add_character.con = "".to_string();
                                }
                            }
                        },
                        AddCharacterField::Int => match new_input.parse::<u16>() {
                            Ok(_) => {
                                self.add_character.int = new_input;
                            }
                            Err(_) => {
                                if new_input.is_empty() {
                                    self.add_character.int = "".to_string();
                                }
                            }
                        },
                        AddCharacterField::Wis => match new_input.parse::<u16>() {
                            Ok(_) => {
                                self.add_character.wis = new_input;
                            }
                            Err(_) => {
                                if new_input.is_empty() {
                                    self.add_character.wis = "".to_string();
                                }
                            }
                        },
                        AddCharacterField::Cha => match new_input.parse::<u16>() {
                            Ok(_) => {
                                self.add_character.cha = new_input;
                            }
                            Err(_) => {
                                if new_input.is_empty() {
                                    self.add_character.cha = "".to_string();
                                }
                            }
                        },
                    }
                    Task::none()
                }
                WindowMessage::AddCharacter => {
                    let name = self.add_character.name.clone();
                    let primary_max_hp = match self.add_character.max_hp.parse::<u16>() {
                        Ok(hp) => hp,
                        Err(error) => {
                            println!("Failed to parse hp into u16, defaulting to 0\n{error}");
                            0
                        }
                    };
                    let health = Health {
                        primary_hp: primary_max_hp,
                        primary_max_hp,
                        secondary_hp: 0,
                        secondary_max_hp: 0,
                        temp_hp: 0,
                        temp_max_hp: 0,
                        wild_shape: false,
                    };
                    let speed = match self.add_character.speed.parse::<u16>() {
                        Ok(speed) => speed,
                        Err(error) => {
                            println!("Failed to parse speed into u16, defaulting to 0\n{error}");
                            0
                        }
                    };
                    let ac = match self.add_character.ac.parse::<u16>() {
                        Ok(ac) => ac,
                        Err(error) => {
                            println!("Failed to parse ac into u16, defaulting to 0\n{error}");
                            0
                        }
                    };
                    let character = Character::new(name, health, speed, ac, Vec::new(), Vec::new());

                    self.actors
                        .insert(character.clone().get_id(), character.clone());
                    // Assumed if empty -> not adding to initiative, will later add a button that
                    // turns the input on and off via a tick box.
                    if self.add_character.in_battle {
                        let initiative = match self.add_character.initiative.parse::<u16>() {
                            Ok(initiative) => initiative,
                            Err(error) => {
                                println!("Failed to parse roll into u16, defaulting to 0\n{error}");
                                0
                            }
                        };
                        self.initiative_queue
                            .add(character.clone().get_id(), initiative, true); // TODO currently
                        // assuming that added character is going into battle, need a checkbox for
                        // this
                    }
                    if self.add_character.in_party {
                        self.party.push(character.get_id());
                    }

                    Task::none()
                }
                WindowMessage::NextTurn => {
                    self.initiative_queue.next_turn();
                    Task::none()
                }
                WindowMessage::RemoveFromInit(index) => {
                    self.initiative_queue.remove(*index);
                    Task::none()
                }
                WindowMessage::ThemeChanged(theme) => {
                    self.theme = theme.clone();
                    Task::none()
                }
                WindowMessage::Clicked(window_id, pane) => {
                    if let Some(window) = self.windows.get_mut(window_id) {
                        window.update(Message::WindowMessage(WindowMessage::Clicked(
                            *window_id, *pane,
                        )));
                    }
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

    pub fn theme(&self, _window: window::Id) -> Theme {
        self.theme.clone()
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
                .style(button::primary)
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
    use iced::border::Radius;
    use iced::widget::container;
    use iced::{Border, Theme};

    const R: f32 = 8.0;

    fn top_only() -> Radius {
        Radius {
            top_left: R,
            top_right: R,
            bottom_right: 0.0,
            bottom_left: 0.0,
        }
    }

    pub fn title_bar_active(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            text_color: Some(palette.background.strong.text),
            background: Some(palette.background.strong.color.into()),
            border: Border {
                radius: top_only(),
                ..Border::default()
            },
            ..Default::default()
        }
    }

    pub fn title_bar_focused(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            text_color: Some(palette.primary.strong.text),
            background: Some(palette.primary.strong.color.into()),
            border: Border {
                radius: top_only(),
                ..Border::default()
            },
            ..Default::default()
        }
    }

    pub fn pane_active(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            background: Some(palette.secondary.strong.color.into()),
            border: Border {
                width: 0.0,
                color: palette.background.strong.color,
                radius: R.into(),
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
                radius: R.into(),
            },
            ..Default::default()
        }
    }

    /// theme for an entry in the initiaive queue widget
    pub fn init_entry(is_party: bool, is_current: bool) -> impl Fn(&Theme) -> container::Style {
        move |theme| {
            let palette = theme.extended_palette();
            let pair = if is_current {
                palette.success.strong
            } else if is_party {
                palette.primary.weak
            } else {
                palette.secondary.weak
            };

            let mut style = container::bordered_box(theme)
                .background(pair.color)
                .color(pair.text);
            if is_current {
                style.border = Border {
                    width: 3.0,
                    color: palette.background.base.text,
                    radius: R.into(),
                };
            }
            style
        }
    }

    /// theme for an entry in the initiaive queue widget
    pub fn primary_box() -> impl Fn(&Theme) -> container::Style {
        move |theme| {
            let palette = theme.extended_palette();
            let pair = palette.background.base;

            container::rounded_box(theme)
                .background(pair.color)
                .color(pair.text)
        }
    }
}
