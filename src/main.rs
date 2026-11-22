mod actor;
mod character;
mod character_message;
mod condition;
mod health;
mod initiative_queue;
mod messages;
mod multiwindow;
mod widgets;
mod window;

use multiwindow::AppState;

fn main() -> iced::Result {
    iced::daemon(AppState::title, AppState::update, AppState::view)
        .subscription(AppState::subscription)
        .theme(AppState::theme)
        .run_with(AppState::new)
}
