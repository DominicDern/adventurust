pub mod app_state;
mod app_window;
mod pane;
pub mod session;
mod widgets;

use crate::ui::widgets::WidgetType;

use crate::ui::app_state::Message;
use crate::ui::session::SessionState;

use iced::Subscription;

use iced::Color;

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

fn initiative_list(state: &mut SessionState) -> Vec<(String, u16)> {
    if let (Some(current_queue), Some(actors)) = (&state.current_queue, &state.actors) {
        if let Some(queue) = state.queues.get_mut(current_queue) {
            queue
                .get_queue()
                .unwrap_or_default()
                .iter()
                .filter_map(|(id, roll)| actors.get(id).map(|actor| (actor.name.clone(), *roll)))
                .collect()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
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
