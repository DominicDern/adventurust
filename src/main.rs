mod actor;
mod character;
mod condition;
mod health;
mod initiative_queue;
mod ui;
mod widgets;

use ui::app_state::AppState;

fn main() -> iced::Result {
    iced::daemon(AppState::title, AppState::update, AppState::view)
        .subscription(AppState::subscription)
        .theme(AppState::theme)
        .run_with(AppState::new)
}
