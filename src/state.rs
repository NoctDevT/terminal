// src/state.rs
use crate::TodoItem;
use ratatui::widgets::ListState;

#[derive(Default)]
pub struct AppState {
    pub items: Vec<TodoItem>,
    pub list_state: ListState,
}
