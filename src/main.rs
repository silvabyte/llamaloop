mod app;
mod ui;
mod api;
mod theme;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{io, time::Duration};
use tokio::time::interval;

use crate::app::{App, CurrentScreen};

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    let mut tick_interval = interval(Duration::from_millis(250));
    
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        tokio::select! {
            _ = tick_interval.tick() => {
                app.on_tick().await;
            }
            _ = tokio::time::sleep(Duration::from_millis(50)) => {
                if event::poll(Duration::from_millis(0))? {
                    if let Event::Key(key) = event::read()? {
                        match app.current_screen {
                            CurrentScreen::Help => {
                                if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                                    app.current_screen = CurrentScreen::Dashboard;
                                }
                            }
                            _ => {
                                match key.code {
                                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                        return Ok(());
                                    }
                                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                        return Ok(());
                                    }
                                    KeyCode::Tab => {
                                        app.next_tab();
                                    }
                                    KeyCode::BackTab => {
                                        app.previous_tab();
                                    }
                                    KeyCode::Char('?') => {
                                        app.current_screen = CurrentScreen::Help;
                                    }
                                    KeyCode::Char('1') => {
                                        app.selected_tab = 0;
                                        app.current_screen = CurrentScreen::Dashboard;
                                    }
                                    KeyCode::Char('2') => {
                                        app.selected_tab = 1;
                                        app.current_screen = CurrentScreen::Models;
                                    }
                                    KeyCode::Char('3') => {
                                        app.selected_tab = 2;
                                        app.current_screen = CurrentScreen::Logs;
                                    }
                                    KeyCode::Char('4') => {
                                        app.selected_tab = 3;
                                        app.current_screen = CurrentScreen::Chat;
                                    }
                                    KeyCode::Up => {
                                        app.on_up();
                                    }
                                    KeyCode::Down => {
                                        app.on_down();
                                    }
                                    KeyCode::Enter => {
                                        app.on_enter().await;
                                    }
                                    KeyCode::Char('r') => {
                                        app.refresh().await;
                                    }
                                    KeyCode::Char('p') if app.current_screen == CurrentScreen::Models => {
                                        app.toggle_pull_dialog();
                                    }
                                    KeyCode::Char('d') if app.current_screen == CurrentScreen::Models => {
                                        app.delete_selected_model().await;
                                    }
                                    KeyCode::Esc => {
                                        if app.show_pull_dialog {
                                            app.show_pull_dialog = false;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}