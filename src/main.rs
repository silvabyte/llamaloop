mod api;
mod app;
mod chat;
mod theme;
mod ui;

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
use crate::chat::InputMode;

#[tokio::main]
async fn main() -> Result<()> {
    // Print epic ASCII art before entering TUI mode
    println!("\n{}", theme::LLAMALOOP_ASCII);
    println!("{}", theme::STARTUP_MESSAGE);
    std::thread::sleep(Duration::from_millis(1500));

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
        eprintln!("Error: {err:?}");
    }

    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    let mut tick_interval = interval(Duration::from_millis(250));

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        // Process chat responses immediately if streaming (for smooth experience)
        if app.current_screen == CurrentScreen::Chat
            && app.chat_state.current_session().is_streaming
        {
            app.process_chat_response().await;
        }

        tokio::select! {
            _ = tick_interval.tick() => {
                app.on_tick().await;
            }
            _ = tokio::time::sleep(Duration::from_millis(10)) => {
                if event::poll(Duration::from_millis(0))? {
                    if let Event::Key(key) = event::read()? {
                        match app.current_screen {
                            CurrentScreen::Help => {
                                if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                                    app.current_screen = CurrentScreen::Dashboard;
                                }
                            }
                            CurrentScreen::Chat => {
                                // Handle chat-specific input
                                match app.chat_state.input_mode {
                                    InputMode::Editing => {
                                        match key.code {
                                            KeyCode::Enter => {
                                                app.send_chat_message().await;
                                            }
                                            KeyCode::Esc => {
                                                app.toggle_chat_input_mode();
                                            }
                                            KeyCode::Backspace => {
                                                app.handle_chat_backspace();
                                            }
                                            KeyCode::Char(c) => {
                                                app.handle_chat_input(c);
                                            }
                                            _ => {}
                                        }
                                    }
                                    InputMode::Normal => {
                                        match key.code {
                                            KeyCode::Char('i') | KeyCode::Char('e') => {
                                                app.toggle_chat_input_mode();
                                            }
                                            KeyCode::Char('c') => {
                                                app.chat_state.current_session().clear_session();
                                            }
                                            KeyCode::Char('m') => {
                                                app.chat_state.toggle_model_selector();
                                            }
                                            KeyCode::Char('n') => {
                                                app.chat_state.new_session();
                                            }
                                            KeyCode::Tab => {
                                                app.next_tab();
                                            }
                                            KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                                                return Ok(());
                                            }
                                            _ => {}
                                        }
                                    }
                                    InputMode::ModelSelection => {
                                        match key.code {
                                            KeyCode::Up => {
                                                if app.chat_state.selected_model_index > 0 {
                                                    app.chat_state.selected_model_index -= 1;
                                                }
                                            }
                                            KeyCode::Down => {
                                                if app.chat_state.selected_model_index < app.chat_state.available_models.len() - 1 {
                                                    app.chat_state.selected_model_index += 1;
                                                }
                                            }
                                            KeyCode::Enter => {
                                                app.chat_state.select_model();
                                            }
                                            KeyCode::Esc => {
                                                app.chat_state.toggle_model_selector();
                                            }
                                            _ => {}
                                        }
                                    }
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
                                        // Refresh models first to ensure we have the latest list
                                        if app.models.is_empty() {
                                            app.refresh().await;
                                        }
                                        app.initialize_chat();
                                    }
                                    KeyCode::Up => {
                                        if app.current_screen == app::CurrentScreen::Models && app.models_tab_view == app::ModelsTabView::Network {
                                            app.select_prev_url();
                                        } else if app.current_screen == app::CurrentScreen::Models && app.models_tab_view == app::ModelsTabView::ApiExplorer {
                                            let endpoints_count = app::App::get_api_endpoints().len();
                                            if endpoints_count > 0 {
                                                let current = app.api_explorer_state.endpoints_list_state.selected().unwrap_or(0);
                                                if current > 0 {
                                                    app.api_explorer_state.endpoints_list_state.select(Some(current - 1));
                                                    app.api_explorer_state.selected_endpoint = current - 1;
                                                }
                                            }
                                        } else {
                                            app.on_up();
                                        }
                                    }
                                    KeyCode::Down => {
                                        if app.current_screen == app::CurrentScreen::Models && app.models_tab_view == app::ModelsTabView::Network {
                                            app.select_next_url();
                                        } else if app.current_screen == app::CurrentScreen::Models && app.models_tab_view == app::ModelsTabView::ApiExplorer {
                                            let endpoints_count = app::App::get_api_endpoints().len();
                                            if endpoints_count > 0 {
                                                let current = app.api_explorer_state.endpoints_list_state.selected().unwrap_or(0);
                                                if current < endpoints_count - 1 {
                                                    app.api_explorer_state.endpoints_list_state.select(Some(current + 1));
                                                    app.api_explorer_state.selected_endpoint = current + 1;
                                                }
                                            }
                                        } else {
                                            app.on_down();
                                        }
                                    }
                                    KeyCode::Enter => {
                                        if app.current_screen == CurrentScreen::Models && app.models_tab_view == app::ModelsTabView::Network {
                                            app.copy_selected_url();
                                        } else if app.current_screen == CurrentScreen::Models && app.models_tab_view == app::ModelsTabView::ApiExplorer {
                                            app.execute_selected_endpoint().await;
                                        } else {
                                            app.on_enter().await;
                                        }
                                    }
                                    KeyCode::Char('r') => {
                                        app.refresh().await;
                                    }
                                    KeyCode::Char('p') if app.current_screen == CurrentScreen::Models => {
                                        app.toggle_pull_dialog();
                                    }
                                    KeyCode::Char('D') if app.current_screen == CurrentScreen::Models && app.models_tab_view == app::ModelsTabView::Library => {
                                        // Capital D (Shift+D) for delete - safer!
                                        app.request_delete_model();
                                    }
                                    KeyCode::Char('i') if app.current_screen == CurrentScreen::Models => {
                                        app.install_selected_available_model().await;
                                    }
                                    KeyCode::Char('t') if app.current_screen == CurrentScreen::Models => {
                                        app.toggle_models_tab();
                                    }
                                    KeyCode::Char('n') if app.current_screen == CurrentScreen::Models && app.models_tab_view == app::ModelsTabView::Network => {
                                        app.discover_network_urls().await;
                                    }
                                    KeyCode::Char('c') if app.current_screen == CurrentScreen::Models && app.models_tab_view == app::ModelsTabView::Network => {
                                        app.copy_selected_url();
                                    }
                                    KeyCode::Char('c') if app.current_screen == CurrentScreen::Models && app.models_tab_view == app::ModelsTabView::ApiExplorer => {
                                        app.api_explorer_state.response_body.clear();
                                    }
                                    KeyCode::Char('v') if app.current_screen == CurrentScreen::Models => {
                                        app.toggle_models_view();
                                    }
                                    KeyCode::Esc => {
                                        if app.show_pull_dialog {
                                            app.show_pull_dialog = false;
                                        } else if app.show_delete_confirmation {
                                            app.cancel_delete();
                                        }
                                    }
                                    KeyCode::Char('y') | KeyCode::Char('Y') if app.show_delete_confirmation => {
                                        app.confirm_delete_model().await;
                                    }
                                    KeyCode::Char('n') | KeyCode::Char('N') if app.show_delete_confirmation => {
                                        app.cancel_delete();
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
