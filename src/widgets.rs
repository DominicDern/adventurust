use std::fmt;

use iced::Element;
use iced::widget::{
    button, checkbox, column, container, pane_grid, row, scrollable, text, text_input,
};
use iced::window;
use iced::{Center, Fill, Length, Size, Theme};

use crate::AppState;
use crate::actor::Actor;
use crate::messages::Message;
use crate::multiwindow::style;
use crate::widgets;
use crate::window::WindowMessage;

// Where to define new widget types
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
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
            row![]
        }
        WidgetType::InitQueue => {
            let mut init_display: iced::widget::Column<'a, Message> = column![].spacing(5);

            for (index, (id, roll)) in state.initiative_queue.queue.iter().enumerate() {
                let character = state.actors.get(id).unwrap();
                let label = format!("{roll} : {}", character.get_name());
                let is_current = state.initiative_queue.position == Some(index);

                let remove_button = button(
                    "Remove",
                    Message::WindowMessage(WindowMessage::RemoveFromInit(index)),
                );
                init_display = init_display.push(
                    container(row![text(label), remove_button])
                        .padding(6)
                        .width(Fill)
                        .style(style::init_entry(state.party.contains(id), is_current)),
                );
            }

            let mut party_display: iced::widget::Column<'a, Message> = column![];

            for id in &state.party {
                let character = state.actors.get(id).unwrap();
                let char_string = character.get_name();
                party_display = party_display.push(
                    container(text(char_string))
                        .padding(6)
                        .width(Fill)
                        .style(style::init_entry(state.party.contains(id), false)),
                );
            }

            row![column![
                row![
                    container(init_display)
                        .padding(6)
                        .width(Fill)
                        .style(style::primary_box()),
                    container(party_display)
                        .padding(6)
                        .width(Fill)
                        .style(style::primary_box()),
                ],
                button("Next turn", Message::WindowMessage(WindowMessage::NextTurn))
            ]]
        }
        WidgetType::AddCharacter => {
            let mut actor_display: iced::widget::Column<'a, Message> = column![];

            for character in state.actors.values() {
                actor_display = actor_display.push(text(character.get_name()));
            }

            row![
                row![
                    column![
                        text_input("name", &state.add_character.name).on_input(|string| {
                            Message::WindowMessage(WindowMessage::AddCharacterChanged(
                                AddCharacterField::Name,
                                string,
                            ))
                        }),
                        row![
                            checkbox("in party", state.add_character.in_party).on_toggle(|_| {
                                Message::WindowMessage(WindowMessage::AddCharacterChanged(
                                    AddCharacterField::InParty,
                                    "".to_string(),
                                ))
                            }),
                            checkbox("in battle", state.add_character.in_battle).on_toggle(|_| {
                                Message::WindowMessage(WindowMessage::AddCharacterChanged(
                                    AddCharacterField::InBattle,
                                    "".to_string(),
                                ))
                            }),
                        ],
                        row![
                            text_input("AC", &state.add_character.ac).on_input(|string| {
                                Message::WindowMessage(WindowMessage::AddCharacterChanged(
                                    AddCharacterField::AC,
                                    string,
                                ))
                            }),
                            if state.add_character.in_battle {
                                text_input("init roll", &state.add_character.initiative).on_input(
                                    |string| {
                                        Message::WindowMessage(WindowMessage::AddCharacterChanged(
                                            AddCharacterField::Initiative,
                                            string,
                                        ))
                                    },
                                )
                            } else {
                                text_input("init roll", &state.add_character.initiative)
                            },
                        ],
                        row![
                            text_input("Max HP", &state.add_character.max_hp).on_input(|string| {
                                Message::WindowMessage(WindowMessage::AddCharacterChanged(
                                    AddCharacterField::HP,
                                    string,
                                ))
                            }),
                            text_input("Speed", &state.add_character.speed).on_input(|string| {
                                Message::WindowMessage(WindowMessage::AddCharacterChanged(
                                    AddCharacterField::Speed,
                                    string,
                                ))
                            }),
                        ],
                        row![
                            text_input("Str", &state.add_character.str).on_input(|string| {
                                Message::WindowMessage(WindowMessage::AddCharacterChanged(
                                    AddCharacterField::Str,
                                    string,
                                ))
                            }),
                            text_input("Dex", &state.add_character.dex).on_input(|string| {
                                Message::WindowMessage(WindowMessage::AddCharacterChanged(
                                    AddCharacterField::Dex,
                                    string,
                                ))
                            }),
                        ],
                        row![
                            text_input("Con", &state.add_character.con).on_input(|string| {
                                Message::WindowMessage(WindowMessage::AddCharacterChanged(
                                    AddCharacterField::Con,
                                    string,
                                ))
                            }),
                            text_input("Int", &state.add_character.int).on_input(|string| {
                                Message::WindowMessage(WindowMessage::AddCharacterChanged(
                                    AddCharacterField::Int,
                                    string,
                                ))
                            }),
                        ],
                        row![
                            text_input("Wis", &state.add_character.wis).on_input(|string| {
                                Message::WindowMessage(WindowMessage::AddCharacterChanged(
                                    AddCharacterField::Wis,
                                    string,
                                ))
                            }),
                            text_input("Cha", &state.add_character.cha).on_input(|string| {
                                Message::WindowMessage(WindowMessage::AddCharacterChanged(
                                    AddCharacterField::Cha,
                                    string,
                                ))
                            }),
                        ],
                        button(
                            "Add character",
                            Message::WindowMessage(WindowMessage::AddCharacter)
                        )
                    ]
                    .width(Length::FillPortion(2)),
                    scrollable(container(scrollable(actor_display)).padding(12).style(
                        |theme: &Theme| {
                            let palette = theme.extended_palette();

                            container::bordered_box(theme)
                                .background(palette.secondary.base.color)
                                .color(palette.secondary.base.text)
                        }
                    ))
                    .width(Length::FillPortion(1)),
                ]
                .width(Fill),
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
    pub in_party: bool,
    pub in_battle: bool,
    pub initiative: String,
    pub max_hp: String,
    pub speed: String,
    pub str: String,
    pub dex: String,
    pub con: String,
    pub int: String,
    pub wis: String,
    pub cha: String,
}

#[derive(Debug, Clone)]
pub enum AddCharacterField {
    Name,
    AC,
    InParty,
    InBattle,
    Initiative,
    HP,
    Speed,
    Str,
    Dex,
    Con,
    Int,
    Wis,
    Cha,
}
