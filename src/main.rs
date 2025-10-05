mod data;
mod state;

use crate::data::todo_item::TodoItem;
use crate::state::AppState;
use color_eyre::Result;
use ratatui::crossterm::event;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::{Color, Style, Stylize};
use ratatui::widgets::{Block, BorderType, List, ListItem, Widget};
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
        .push(TodoItem { is_done: false, description: String::from("Hello World 1 ") });
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
            match key.code {
                event::KeyCode::Esc => break,
                event::KeyCode::Char(char) => match char {
                    'D' => {
                        if let Some(index) = app_state.list_state.selected() {
                            if index < app_state.items.len() {
                                app_state.items.remove(index);

                                if app_state.items.is_empty() {
                                    app_state.list_state.select(None);
                                } else if index >= app_state.items.len() {
                                    // Deleted last item, move selection up
                                    app_state.list_state.select(Some(app_state.items.len() - 1));
                                }
                                // else: keep same index (now points to next item)
                            }
                        }
                    }
                    'k' => {
                        app_state.list_state.select_previous();
                    }
                    'j' => {
                        app_state.list_state.select_next();
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app_state: &mut AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)]).margin(1).areas(frame.area());

    let [inner_area] = Layout::vertical([Constraint::Fill(1)]).margin(1).areas(border_area);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());

    let list_items = app_state.items.iter().map(|item| ListItem::from(item.description.clone()));

    let list = List::new(list_items)
        .highlight_style(Style::default().fg(Color::Green))
        .highlight_symbol(">");
    frame.render_stateful_widget(list, inner_area, &mut app_state.list_state)
    // Paragraph::new("Hello from application").render(frame.area(), frame.buffer_mut());
}
