use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WidgetType {
    Blank,
    InitiativeQueue,
    ActiveSheet,
    AddCharacter { add_to_current_initiative: bool },
}

impl WidgetType {
    pub const ALL: &'static [WidgetType] = &[
        WidgetType::Blank,
        WidgetType::InitiativeQueue,
        WidgetType::ActiveSheet,
        WidgetType::AddCharacter {
            add_to_current_initiative: false,
        },
    ];
}

impl fmt::Display for WidgetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WidgetType::Blank => write!(f, "Blank widget"),
            WidgetType::InitiativeQueue => write!(f, "Initiative"),
            WidgetType::ActiveSheet => write!(f, "Active Sheet"),
            WidgetType::AddCharacter {
                add_to_current_initiative: _,
            } => write!(f, "Add New Character"),
        }
    }
}
