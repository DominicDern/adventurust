use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum WidgetType {
    Blank,
    WidgetA,
    WidgetB,
}

impl WidgetType {
    pub const ALL: &'static [WidgetType] =
        &[WidgetType::Blank, WidgetType::WidgetA, WidgetType::WidgetB];
}

impl fmt::Display for WidgetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WidgetType::Blank => write!(f, "blank"),
            WidgetType::WidgetA => write!(f, "A"),
            WidgetType::WidgetB => write!(f, "B"),
        }
    }
}
