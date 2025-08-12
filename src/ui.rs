use crate::app::{App, CurrentScreen, LogLevel};
use humansize::{format_size, BINARY};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Cell, Clear, Gauge, List, ListItem, Paragraph, Row,
        Table, Tabs, Wrap,
    },
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    draw_header(f, app, chunks[0]);
    
    match app.current_screen {
        CurrentScreen::Dashboard => draw_dashboard(f, app, chunks[1]),
        CurrentScreen::Models => draw_models(f, app, chunks[1]),
        CurrentScreen::Logs => draw_logs(f, app, chunks[1]),
        CurrentScreen::Chat => draw_chat(f, app, chunks[1]),
        CurrentScreen::Help => draw_help(f, chunks[1]),
    }
    
    draw_footer(f, app, chunks[2]);

    if app.show_pull_dialog {
        draw_pull_dialog(f, app);
    }
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(0)])
        .split(area);

    let title = Paragraph::new(
        Line::from(vec![
            Span::styled("🦙 ", Style::default()),
            Span::styled(
                "Ollamamon",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(title, header_chunks[0]);

    let tabs = vec!["Dashboard", "Models", "Logs", "Chat"];
    let tabs = Tabs::new(tabs)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .select(app.selected_tab)
        .style(Style::default().fg(Color::Gray))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(tabs, header_chunks[1]);
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    let keybinds = match app.current_screen {
        CurrentScreen::Models => {
            vec![
                ("Tab", "Next Tab"),
                ("↑↓", "Navigate"),
                ("p", "Pull"),
                ("d", "Delete"),
                ("r", "Refresh"),
                ("?", "Help"),
                ("Ctrl-C", "Quit"),
            ]
        }
        CurrentScreen::Dashboard => {
            vec![
                ("Tab", "Next Tab"),
                ("1-4", "Jump to Tab"),
                ("r", "Refresh"),
                ("?", "Help"),
                ("Ctrl-C", "Quit"),
            ]
        }
        _ => {
            vec![
                ("Tab", "Next Tab"),
                ("r", "Refresh"),
                ("?", "Help"),
                ("Ctrl-C", "Quit"),
            ]
        }
    };

    let hints: Vec<Span> = keybinds
        .iter()
        .flat_map(|(key, desc)| {
            vec![
                Span::styled(
                    format!(" {} ", key),
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Gray)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!(" {} ", desc)),
            ]
        })
        .collect();

    let hints = Paragraph::new(Line::from(hints))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
    f.render_widget(hints, footer_chunks[0]);

    let status = if app.status.is_running {
        Line::from(vec![
            Span::styled("● ", Style::default().fg(Color::Green)),
            Span::styled("Connected", Style::default().fg(Color::Green)),
            Span::raw(" | "),
            Span::raw(format!("{} models", app.status.models_loaded)),
        ])
    } else {
        Line::from(vec![
            Span::styled("● ", Style::default().fg(Color::Red)),
            Span::styled("Disconnected", Style::default().fg(Color::Red)),
        ])
    };

    let status = Paragraph::new(status)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .alignment(Alignment::Center);
    f.render_widget(status, footer_chunks[1]);
}

fn draw_dashboard(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(7), Constraint::Min(0)])
        .margin(1)
        .split(area);

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(chunks[0]);

    let status_color = if app.status.is_running {
        Color::Green
    } else {
        Color::Red
    };

    let status_widget = Block::default()
        .title("Status")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(status_color));
    let status_content = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(
                if app.status.is_running { "●" } else { "○" },
                Style::default().fg(status_color),
            ),
            Span::raw(" "),
            Span::raw(if app.status.is_running {
                "Running"
            } else {
                "Stopped"
            }),
        ]),
        Line::from(format!("Version: {}", app.status.version)),
    ])
    .block(status_widget)
    .alignment(Alignment::Center);
    f.render_widget(status_content, top_chunks[0]);

    let models_widget = Block::default()
        .title("Models")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Blue));
    let models_content = Paragraph::new(vec![
        Line::from(vec![Span::styled(
            app.status.models_loaded.to_string(),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("Loaded"),
    ])
    .block(models_widget)
    .alignment(Alignment::Center);
    f.render_widget(models_content, top_chunks[1]);

    let running_widget = Block::default()
        .title("Running")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Yellow));
    let running_content = Paragraph::new(vec![
        Line::from(vec![Span::styled(
            app.running_models.len().to_string(),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("Active"),
    ])
    .block(running_widget)
    .alignment(Alignment::Center);
    f.render_widget(running_content, top_chunks[2]);

    let memory_pct = if app.status.total_memory > 0 {
        (app.status.used_memory as f64 / app.status.total_memory as f64) * 100.0
    } else {
        0.0
    };

    let memory_color = if memory_pct > 80.0 {
        Color::Red
    } else if memory_pct > 60.0 {
        Color::Yellow
    } else {
        Color::Green
    };

    let memory_widget = Gauge::default()
        .block(
            Block::default()
                .title("Memory")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(memory_color)),
        )
        .gauge_style(Style::default().fg(memory_color))
        .percent(memory_pct as u16)
        .label(format!("{:.1}%", memory_pct));
    f.render_widget(memory_widget, top_chunks[3]);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    draw_running_models(f, app, bottom_chunks[0]);
    draw_recent_logs(f, app, bottom_chunks[1]);
}

fn draw_running_models(f: &mut Frame, app: &App, area: Rect) {
    let models: Vec<ListItem> = app
        .running_models
        .iter()
        .map(|m| {
            let size_str = format_size(m.size, BINARY);
            let vram_str = m
                .size_vram
                .map(|v| format!(" | VRAM: {}", format_size(v, BINARY)))
                .unwrap_or_default();
            
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled("▶ ", Style::default().fg(Color::Green)),
                    Span::styled(&m.name, Style::default().fg(Color::Cyan)),
                ]),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled(
                        format!("Size: {}{}", size_str, vram_str),
                        Style::default().fg(Color::Gray),
                    ),
                ]),
            ])
        })
        .collect();

    let models_list = List::new(models)
        .block(
            Block::default()
                .title("Running Models")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(models_list, area);
}

fn draw_recent_logs(f: &mut Frame, app: &App, area: Rect) {
    let logs: Vec<ListItem> = app
        .logs
        .iter()
        .rev()
        .take(10)
        .map(|log| {
            let (symbol, color) = match log.level {
                LogLevel::Info => ("ℹ", Color::Blue),
                LogLevel::Warning => ("⚠", Color::Yellow),
                LogLevel::Error => ("✖", Color::Red),
                LogLevel::Debug => ("●", Color::Gray),
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!("{} ", symbol), Style::default().fg(color)),
                Span::styled(
                    log.timestamp.format("%H:%M:%S").to_string(),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::raw(" "),
                Span::raw(&log.message),
            ]))
        })
        .collect();

    let logs_list = List::new(logs)
        .block(
            Block::default()
                .title("Recent Activity")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

    f.render_widget(logs_list, area);
}

fn draw_models(f: &mut Frame, app: &App, area: Rect) {
    let header_cells = ["Name", "Size", "Parameters", "Quantization", "Modified"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
    let header = Row::new(header_cells)
        .style(Style::default().bg(Color::DarkGray))
        .height(1);

    let rows = app.models.iter().enumerate().map(|(i, model)| {
        let size_str = format_size(model.size, BINARY);
        let params = model
            .details
            .as_ref()
            .map(|d| d.parameter_size.clone())
            .unwrap_or_else(|| "-".to_string());
        let quant = model
            .details
            .as_ref()
            .map(|d| d.quantization_level.clone())
            .unwrap_or_else(|| "-".to_string());
        
        let style = if i == app.selected_model_index {
            Style::default().bg(Color::DarkGray).fg(Color::White)
        } else {
            Style::default()
        };

        Row::new(vec![
            Cell::from(model.name.clone()),
            Cell::from(size_str),
            Cell::from(params),
            Cell::from(quant),
            Cell::from(model.modified_at.clone()),
        ])
        .style(style)
    });

    let table = Table::new(
        rows,
        &[
            Constraint::Percentage(30),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(15),
            Constraint::Percentage(25),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title("Available Models")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
    )
    .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(table, area);
}

fn draw_logs(f: &mut Frame, app: &App, area: Rect) {
    let logs: Vec<ListItem> = app
        .logs
        .iter()
        .rev()
        .map(|log| {
            let (symbol, color) = match log.level {
                LogLevel::Info => ("ℹ", Color::Blue),
                LogLevel::Warning => ("⚠", Color::Yellow),
                LogLevel::Error => ("✖", Color::Red),
                LogLevel::Debug => ("●", Color::Gray),
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!("{} ", symbol), Style::default().fg(color)),
                Span::styled(
                    log.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::raw(" "),
                Span::raw(&log.message),
            ]))
        })
        .collect();

    let logs_list = List::new(logs)
        .block(
            Block::default()
                .title("Logs")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

    f.render_widget(logs_list, area);
}

fn draw_chat(f: &mut Frame, _app: &App, area: Rect) {
    let chat = Paragraph::new("Chat interface coming soon...")
        .block(
            Block::default()
                .title("Chat")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .alignment(Alignment::Center);

    f.render_widget(chat, area);
}

fn draw_help(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from(vec![Span::styled(
            "Ollamamon - TUI for Ollama",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Navigation",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  Tab/Shift-Tab  Navigate between tabs"),
        Line::from("  1-4            Jump to specific tab"),
        Line::from("  ↑/↓            Navigate lists"),
        Line::from("  Enter          Select/Confirm"),
        Line::from("  Esc            Cancel/Back"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Commands",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  r              Refresh data"),
        Line::from("  p              Pull model (in Models tab)"),
        Line::from("  d              Delete model (in Models tab)"),
        Line::from("  ?              Show this help"),
        Line::from("  Ctrl-C/Ctrl-Q  Quit"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Tips",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  • The dashboard refreshes automatically every 5 seconds"),
        Line::from("  • Use the Models tab to manage your Ollama models"),
        Line::from("  • Check the Logs tab for system activity"),
        Line::from(""),
        Line::from("Press Esc or q to close this help"),
    ];

    let help = Paragraph::new(help_text)
        .block(
            Block::default()
                .title("Help")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(help, area);
}

fn draw_pull_dialog(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 20, f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .title("Pull Model")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(inner);

    let hint = Paragraph::new("Enter model name (e.g., llama3.2, mistral)")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(hint, chunks[0]);

    let input = Paragraph::new(app.pull_model_name.as_str())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Gray)),
        );
    f.render_widget(input, chunks[1]);

    let help = Paragraph::new("Press Enter to pull, Esc to cancel")
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}