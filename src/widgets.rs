use std::fmt;

use iced::Element;
use iced::widget::{button, checkbox, column, container, pane_grid, row, text, text_input};
use iced::window;
use iced::{Center, Fill, Size, Theme};

use crate::AppState;
use crate::actor::Actor;
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
    pub const ALL: &'static [WidgetType] = &[
        WidgetType::Blank,
        WidgetType::InitQueue,
        WidgetType::AddCharacter,
    ];
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
            let mut init_display: iced::widget::Column<'a, Message> = column![];

            for (id, roll) in &state.initiative_queue.queue {
                let character = state.actors.get(id).unwrap();
                let char_string = character.get_name();
                let mut init_string = roll.to_string();
                init_string.push_str(" : ");
                init_string.push_str(&char_string);
                init_display = init_display.push(text(init_string));
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
                init_display
            ]
        }
        WidgetType::AddCharacter => {
            let mut actor_display: iced::widget::Column<'a, Message> = column![];

            for character in state.actors.values() {
                actor_display = actor_display.push(text(character.get_name()));
            }

            row![
                column![
                    text_input("name", &state.add_character.name).on_input(|string| {
                        Message::WindowMessage(WindowMessage::AddCharacterChanged(
                            AddCharacterField::Name,
                            string,
                        ))
                    }),
                    checkbox("in battle", state.add_character.in_battle).on_toggle(|_| {
                        Message::WindowMessage(WindowMessage::AddCharacterChanged(
                            AddCharacterField::InBattle,
                            "".to_string(),
                        ))
                    }),
                    text_input("init roll", &state.add_character.initiative).on_input(|string| {
                        Message::WindowMessage(WindowMessage::AddCharacterChanged(
                            AddCharacterField::Initiative,
                            string,
                        ))
                    }),
                    text_input("AC", &state.add_character.ac).on_input(|string| {
                        Message::WindowMessage(WindowMessage::AddCharacterChanged(
                            AddCharacterField::AC,
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
                ),
                container(actor_display).padding(12).style(|theme: &Theme| {
                    let palette = theme.extended_palette();

                    container::bordered_box(theme)
                        .background(palette.secondary.base.color)
                        .color(palette.secondary.base.text)
                })
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
    pub ac: String,
    pub primary_max_hp: String,
    pub in_battle: bool,
    pub player_controlled: bool,
    pub initiative: String,
}

#[derive(Debug, Clone)]
pub enum AddCharacterField {
    Name,
    AC,
    InBattle,
    Initiative,
}
