mod data;
mod state;

use crate::data::todo_item::TodoItem;
use crate::state::AppState;
use color_eyre::Result;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Color, Style, Stylize};
use ratatui::widgets::{Block, BorderType, List, ListDirection, ListItem, Padding, Paragraph, Widget};
use ratatui::{DefaultTerminal, Frame};

fn main() -> Result<()> {
    let mut state = AppState::default();
    state
        .items
        .push(TodoItem { is_done: false, description: String::from("Hello World 1 ") });

    state
        .items
        .push(TodoItem { is_done: false, description: String::from("Hello World 2 ") });

    state
        .items
        .push(TodoItem { is_done: false, description: String::from("Hello World 3 ") });
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal, &mut state);

    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    // Initialize selection
    if !app_state.items.is_empty() {
        app_state.list_state.select(Some(0));
    }

    loop {
        terminal.draw(|frame| render(frame, app_state))?;

        if let Event::Key(key) = event::read()? {
            if app_state.is_new_item {
                if handle_add_new(key, app_state){
                    app_state.is_new_item = false;
                }
            } else {
                if handle_key(key, app_state) {
                    break;
                }
            }
        }
    }
    Ok(())
}

pub fn handle_add_new(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        event::KeyCode::Esc => {
            return true;
        }
        event::KeyCode::Enter => {
            return true;
        }

        _ => {false}
    }
}


pub fn handle_key(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        KeyCode::Esc => { true },
        KeyCode::Char('k') | KeyCode::Up => {
            app_state.list_state.select_previous();
            false
        }
        KeyCode::Char('j') | KeyCode::Down => {
            app_state.list_state.select_next();
            false
        }
        KeyCode::Char('D') => {
            if let Some(index) = app_state.list_state.selected() {
                app_state.items.remove(index);
            }
            false
        }
        KeyCode::Char('A') => {
            app_state.is_new_item = true;
            if let Some(index) = app_state.list_state.selected() {
                //Deserialize input into struct
                // Input struct into field

                // app_state.items.push()
            }
            false
        }
        _ => {false }
    }
}


fn render(frame: &mut Frame, app_state: &mut AppState) {
    let [border_area] =
        Layout::vertical([Constraint::Fill(1)]).margin(1).areas(frame.area());

    let [inner_area] =
        Layout::vertical([Constraint::Fill(1)]).margin(1).areas(border_area);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());

    let list_items: Vec<ListItem> = app_state
        .items
        .iter()
        .map(|item| ListItem::new(item.description.as_str()))
        .collect();

    let list = List::new(list_items)
        .highlight_style(Style::default().fg(Color::Green))
        .highlight_symbol(">")
        .direction(ListDirection::TopToBottom);
    frame.render_stateful_widget(list, inner_area, &mut app_state.list_state);

    if app_state.is_new_item {
        Paragraph::new(app_state.input_value.as_str())
            .block(Block::bordered()
                .fg(Color::Green)
                .padding(Padding::uniform(1))
                .border_type(BorderType::Rounded))
            .render(frame.area(), frame.buffer_mut());
    }
}
