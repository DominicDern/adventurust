use std::fmt;

use iced::Element;
use iced::widget::{button, column, container, pane_grid, row, text, text_input};
use iced::window;
use iced::{Center, Fill, Size};

use crate::AppState;
use crate::actor::Actor;
use crate::character::{Character, ID};
use crate::condition::Condition;
use crate::health::Health;
use crate::messages::Message;
use crate::widgets;
use crate::window::WindowMessage;

// Where to define new widget types
#[derive(Debug, Clone, Copy, Default)]
pub enum WidgetType {
    #[default]
    Blank,
    InitQueue,
    AddCharacter,
}

impl WidgetType {
    pub const ALL: &'static [WidgetType] = &[WidgetType::Blank, WidgetType::InitQueue];
}

impl fmt::Display for WidgetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WidgetType::Blank => write!(f, "blank"),
            WidgetType::InitQueue => write!(f, "Initiative Queue"),
            WidgetType::AddCharacter => write!(f, "Add Character"),
        }
    }
}

// Where to add the views of the widgets.
pub fn view_content<'a>(
    window_id: window::Id,
    state: &'a AppState,
    pane: pane_grid::Pane,
    total_panes: usize,
    widget_type: widgets::WidgetType,
    is_pinned: bool,
    _size: Size,
) -> Element<'a, Message> {
    let button = |label, message| {
        button(text(label).width(Fill).align_x(Center).size(16))
            .width(Fill)
            .padding(8)
            .on_press(message)
    };

    let content = match widget_type {
        WidgetType::Blank => {
            row![
                text!("Blank widget"),
                button(
                    "Change to init queue",
                    Message::WindowMessage(WindowMessage::WidgetChange(
                        window_id,
                        pane,
                        WidgetType::InitQueue
                    ))
                )
            ]
        }
        WidgetType::InitQueue => {
            let mut queue_display: iced::widget::Column<'a, Message> = column![];

            for (_, character) in &state.actors {
                queue_display = queue_display.push(text(character.get_name()));
            }

            row![
                text!("Init queue"),
                button(
                    "Change to AddCharacter",
                    Message::WindowMessage(WindowMessage::WidgetChange(
                        window_id,
                        pane,
                        WidgetType::AddCharacter
                    )),
                ),
                queue_display
            ]
        }
        WidgetType::AddCharacter => {
            let aaron = Character::from_file("src/example_characters/aaron_nightbringer.toml");
            let lyra = Character::from_file("src/example_characters/lyra_shadebottom.toml");

            let name: String;
            let id: ID;
            let health: Health;
            let ac: u16;
            let resistances: Option<Vec<String>>;
            let conditions: Option<Vec<Condition>>;

            row![
                column![
                    text_input("name", &state.add_character.name).on_input(|string| {
                        Message::WindowMessage(WindowMessage::AddCharacterChanged(
                            AddCharacterField::Name,
                            string,
                        ))
                    }),
                    text_input("init roll", &state.add_character.roll).on_input(|string| {
                        Message::WindowMessage(WindowMessage::AddCharacterChanged(
                            AddCharacterField::Roll,
                            string,
                        ))
                    }),
                    button(
                        "Add character",
                        Message::WindowMessage(WindowMessage::AddCharacter)
                    )
                ],
                text!("AddCharacter"),
                button(
                    "Change to blank widget",
                    Message::WindowMessage(WindowMessage::WidgetChange(
                        window_id,
                        pane,
                        WidgetType::Blank
                    ))
                )
            ]
        }
    }
    .width(Fill)
    .height(Fill);

    let controls = column![
        row![
            button(
                "Split horizontally",
                Message::WindowMessage(WindowMessage::Split(
                    window_id,
                    pane_grid::Axis::Horizontal,
                    pane
                )),
            ),
            button(
                "Split vertically",
                Message::WindowMessage(WindowMessage::Split(
                    window_id,
                    pane_grid::Axis::Vertical,
                    pane
                ))
            ),
        ],
        content.width(Fill)
    ]
    .push_maybe(if total_panes > 1 && !is_pinned {
        Some(
            button(
                "Close",
                Message::WindowMessage(WindowMessage::Close(window_id, pane)),
            )
            .style(button::danger),
        )
    } else {
        None
    })
    .spacing(5);

    container(controls).center_y(Fill).padding(5).into()
}

#[derive(Default, Clone)]
pub struct AddCharacterState {
    pub name: String,
    pub roll: String,
}

#[derive(Debug, Clone)]
pub enum AddCharacterField {
    Name,
    Roll,
}
