use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum WidgetType {
    Blank,
    InitQueue,
    WidgetB,
}

impl WidgetType {
    pub const ALL: &'static [WidgetType] = &[
        WidgetType::Blank,
        WidgetType::InitQueue,
        WidgetType::WidgetB,
    ];
}

impl fmt::Display for WidgetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WidgetType::Blank => write!(f, "blank"),
            WidgetType::InitQueue => write!(f, "Initiative Queue"),
            WidgetType::WidgetB => write!(f, "B"),
        }
    }
}
