use crate::app::{App, CurrentScreen, LogLevel, ModelsTabView, ModelsViewMode};
use crate::chat::{InputMode, MessageRole};
use crate::theme::{self, TokyoNight};
use humansize::{format_size, BINARY};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Gauge, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    // Set background
    let bg_block = Block::default().style(Style::default().bg(TokyoNight::BG));
    f.render_widget(bg_block, f.area());

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
        CurrentScreen::Models => draw_models_unified(f, app, chunks[1]),
        CurrentScreen::Logs => draw_logs(f, app, chunks[1]),
        CurrentScreen::Chat => draw_chat(f, app, chunks[1]),
        CurrentScreen::Help => draw_help(f, chunks[1]),
    }

    draw_footer(f, app, chunks[2]);

    if app.show_pull_dialog {
        draw_pull_dialog(f, app);
    }

    if app.show_delete_confirmation {
        draw_delete_confirmation(f, app);
    }

    // Draw sparkles on top
    for (x, y, char, color) in app.sparkle.get_sparkles() {
        if x < f.area().width && y < f.area().height {
            let sparkle = Span::styled(char.to_string(), Style::default().fg(color));
            let sparkle_widget = Paragraph::new(sparkle);
            let area = Rect::new(x, y, 1, 1);
            f.render_widget(sparkle_widget, area);
        }
    }
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(0)])
        .split(area);

    // Title with gradient effect
    let title_text = "🔄 llamaloop";
    let gradient_chars = theme::gradient_text(title_text, TokyoNight::CYAN, TokyoNight::MAGENTA);
    let title_spans: Vec<Span> = gradient_chars
        .iter()
        .map(|(c, color)| {
            Span::styled(
                c.to_string(),
                Style::default().fg(*color).add_modifier(Modifier::BOLD),
            )
        })
        .collect();

    let title = Paragraph::new(Line::from(title_spans))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(
                    Style::default().fg(theme::pulse_color(TokyoNight::CYAN, app.animation_tick)),
                )
                .style(Style::default().bg(TokyoNight::BG_DARK)),
        )
        .alignment(Alignment::Center);
    f.render_widget(title, header_chunks[0]);

    let tabs = vec!["⚡ Dashboard", "📦 Models & API", "📜 Logs", "💬 Chat"];
    let tabs = Tabs::new(tabs)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().bg(TokyoNight::BG_DARK))
                .border_style(Style::default().fg(TokyoNight::TERMINAL_BLACK)),
        )
        .select(app.selected_tab)
        .style(Style::default().fg(TokyoNight::COMMENT))
        .highlight_style(
            Style::default()
                .fg(TokyoNight::CYAN)
                .add_modifier(Modifier::BOLD)
                .bg(TokyoNight::BG_HIGHLIGHT),
        );
    f.render_widget(tabs, header_chunks[1]);
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect) {
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    let keybinds = match app.current_screen {
        CurrentScreen::Models => match app.models_tab_view {
            ModelsTabView::Library => {
                vec![
                    ("Tab", "Next"),
                    ("t", "Switch Tab"),
                    ("↑↓", "Navigate"),
                    ("v", "View Mode"),
                    ("i", "Install"),
                    ("p", "Pull"),
                    ("d", "Delete"),
                    ("?", "Help"),
                ]
            }
            ModelsTabView::ApiExplorer => {
                vec![
                    ("Tab", "Next"),
                    ("t", "Switch Tab"),
                    ("↑↓", "Navigate"),
                    ("Enter", "Execute"),
                    ("c", "Clear"),
                    ("n", "Network"),
                    ("?", "Help"),
                ]
            }
            ModelsTabView::Network => {
                vec![
                    ("Tab", "Next"),
                    ("t", "Switch Tab"),
                    ("n", "Discover URLs"),
                    ("d", "Scan Network"),
                    ("r", "Refresh"),
                    ("?", "Help"),
                ]
            }
        },
        CurrentScreen::Dashboard => {
            vec![
                ("Tab", "Next"),
                ("1-4", "Jump"),
                ("r", "Refresh"),
                ("?", "Help"),
                ("^C", "Quit"),
            ]
        }
        _ => {
            vec![
                ("Tab", "Next"),
                ("r", "Refresh"),
                ("?", "Help"),
                ("^C", "Quit"),
            ]
        }
    };

    let hints: Vec<Span> = keybinds
        .iter()
        .flat_map(|(key, desc)| {
            vec![
                Span::styled(
                    format!(" {key} "),
                    Style::default()
                        .fg(TokyoNight::BG)
                        .bg(TokyoNight::MAGENTA)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!(" {desc} "),
                    Style::default().fg(TokyoNight::FG_DARK),
                ),
            ]
        })
        .collect();

    let hints = Paragraph::new(Line::from(hints)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(TokyoNight::TERMINAL_BLACK))
            .style(Style::default().bg(TokyoNight::BG_DARK)),
    );
    f.render_widget(hints, footer_chunks[0]);

    let status = if app.status.is_running {
        Line::from(vec![
            Span::styled("● ", Style::default().fg(TokyoNight::GREEN)),
            Span::styled("Connected", Style::default().fg(TokyoNight::GREEN1)),
            Span::styled(" │ ", Style::default().fg(TokyoNight::DARK3)),
            Span::styled(
                format!("{} models", app.status.models_loaded),
                Style::default().fg(TokyoNight::FG),
            ),
        ])
    } else {
        Line::from(vec![
            Span::styled(
                "● ",
                Style::default().fg(theme::pulse_color(TokyoNight::RED, app.animation_tick)),
            ),
            Span::styled("Disconnected", Style::default().fg(TokyoNight::RED)),
        ])
    };

    let status = Paragraph::new(status)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(TokyoNight::TERMINAL_BLACK))
                .style(Style::default().bg(TokyoNight::BG_DARK)),
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

    // Status widget with pulsing effect
    let status_color = if app.status.is_running {
        theme::pulse_color(TokyoNight::GREEN, app.animation_tick)
    } else {
        theme::pulse_color(TokyoNight::RED, app.animation_tick)
    };

    let status_widget = Block::default()
        .title("⚡ Status")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(status_color))
        .style(Style::default().bg(TokyoNight::BG_DARK));

    let status_content = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(
                if app.status.is_running { "●" } else { "○" },
                Style::default()
                    .fg(status_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(
                if app.status.is_running {
                    "Running"
                } else {
                    "Stopped"
                },
                Style::default().fg(TokyoNight::FG),
            ),
        ]),
        Line::from(Span::styled(
            format!("v{}", app.status.version),
            Style::default().fg(TokyoNight::COMMENT),
        )),
    ])
    .block(status_widget)
    .alignment(Alignment::Center);
    f.render_widget(status_content, top_chunks[0]);

    // Models widget
    let models_widget = Block::default()
        .title("📦 Models")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(TokyoNight::BLUE))
        .style(Style::default().bg(TokyoNight::BG_DARK));

    let models_content = Paragraph::new(vec![
        Line::from(vec![Span::styled(
            app.status.models_loaded.to_string(),
            Style::default()
                .fg(TokyoNight::CYAN)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(Span::styled(
            "Loaded",
            Style::default().fg(TokyoNight::FG_DARK),
        )),
    ])
    .block(models_widget)
    .alignment(Alignment::Center);
    f.render_widget(models_content, top_chunks[1]);

    // Running widget with sparkle
    let running_widget = Block::default()
        .title("✨ Running")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(TokyoNight::YELLOW))
        .style(Style::default().bg(TokyoNight::BG_DARK));

    let running_content = Paragraph::new(vec![
        Line::from(vec![Span::styled(
            app.running_models.len().to_string(),
            Style::default()
                .fg(TokyoNight::YELLOW)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(Span::styled(
            "Active",
            Style::default().fg(TokyoNight::FG_DARK),
        )),
    ])
    .block(running_widget)
    .alignment(Alignment::Center);
    f.render_widget(running_content, top_chunks[2]);

    // Memory gauge with gradient
    let memory_pct = if app.status.total_memory > 0 {
        (app.status.used_memory as f64 / app.status.total_memory as f64) * 100.0
    } else {
        0.0
    };

    let memory_color = if memory_pct > 80.0 {
        TokyoNight::RED
    } else if memory_pct > 60.0 {
        TokyoNight::YELLOW
    } else {
        TokyoNight::GREEN
    };

    let memory_widget = Gauge::default()
        .block(
            Block::default()
                .title("💾 Memory")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(memory_color))
                .style(Style::default().bg(TokyoNight::BG_DARK)),
        )
        .gauge_style(
            Style::default()
                .fg(memory_color)
                .bg(TokyoNight::BG_HIGHLIGHT),
        )
        .percent(memory_pct as u16)
        .label(format!("{memory_pct:.1}%"));
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
                .map(|v| format!(" │ VRAM: {}", format_size(v, BINARY)))
                .unwrap_or_default();

            ListItem::new(vec![
                Line::from(vec![
                    Span::styled("▶ ", Style::default().fg(TokyoNight::GREEN)),
                    Span::styled(
                        &m.name,
                        Style::default()
                            .fg(TokyoNight::CYAN)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::raw("  "),
                    Span::styled(
                        format!("Size: {size_str}{vram_str}"),
                        Style::default().fg(TokyoNight::COMMENT),
                    ),
                ]),
            ])
        })
        .collect();

    let models_list = List::new(models)
        .block(
            Block::default()
                .title("🚀 Running Models")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(TokyoNight::TERMINAL_BLACK))
                .style(Style::default().bg(TokyoNight::BG_DARK)),
        )
        .style(Style::default().fg(TokyoNight::FG));

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
                LogLevel::Info => ("ℹ", TokyoNight::BLUE),
                LogLevel::Warning => ("⚠", TokyoNight::YELLOW),
                LogLevel::Error => ("✖", TokyoNight::RED),
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!("{symbol} "), Style::default().fg(color)),
                Span::styled(
                    log.timestamp.format("%H:%M:%S").to_string(),
                    Style::default().fg(TokyoNight::DARK5),
                ),
                Span::raw(" "),
                Span::styled(&log.message, Style::default().fg(TokyoNight::FG)),
            ]))
        })
        .collect();

    let logs_list = List::new(logs).block(
        Block::default()
            .title("📊 Recent Activity")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(TokyoNight::TERMINAL_BLACK))
            .style(Style::default().bg(TokyoNight::BG_DARK)),
    );

    f.render_widget(logs_list, area);
}

fn draw_models_unified(f: &mut Frame, app: &mut App, area: Rect) {
    // Split into header and content
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    // Draw tab selector at the top
    draw_models_tab_selector(f, app, chunks[0]);

    // Draw content based on selected tab
    match app.models_tab_view {
        ModelsTabView::Library => {
            // Split for library view with mode selector
            let lib_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(chunks[1]);

            draw_view_mode_selector(f, app, lib_chunks[0]);
            draw_models_list(f, app, lib_chunks[1]);
        }
        ModelsTabView::ApiExplorer => {
            draw_api_explorer_view(f, app, chunks[1]);
        }
        ModelsTabView::Network => {
            draw_network_view(f, app, chunks[1]);
        }
    }
}

fn draw_models_tab_selector(f: &mut Frame, app: &App, area: Rect) {
    let tabs = vec!["📚 Library", "🔌 API Explorer", "🌐 Network"];
    let selected = match app.models_tab_view {
        ModelsTabView::Library => 0,
        ModelsTabView::ApiExplorer => 1,
        ModelsTabView::Network => 2,
    };

    let tabs = Tabs::new(tabs)
        .block(
            Block::default()
                .title("Models & API")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(TokyoNight::BLUE))
                .style(Style::default().bg(TokyoNight::BG_DARK)),
        )
        .select(selected)
        .style(Style::default().fg(TokyoNight::COMMENT))
        .highlight_style(
            Style::default()
                .fg(TokyoNight::CYAN)
                .add_modifier(Modifier::BOLD)
                .bg(TokyoNight::BG_HIGHLIGHT),
        );

    f.render_widget(tabs, area);
}

fn draw_api_explorer_view(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(area);

    // Left panel - API endpoints
    draw_api_endpoints_list(f, app, chunks[0]);

    // Right panel - Request/Response
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(chunks[1]);

    draw_request_editor(f, app, right_chunks[0]);
    draw_response_viewer(f, app, right_chunks[1]);
}

fn draw_network_view(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Left - Network URLs
    draw_network_urls(f, app, chunks[0]);

    // Right - Discovered Services
    draw_discovered_services(f, app, chunks[1]);
}

fn draw_network_urls(f: &mut Frame, app: &App, area: Rect) {
    let mut lines = vec![
        Line::from(vec![Span::styled(
            "Local Network URLs",
            Style::default()
                .fg(TokyoNight::CYAN)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if app.api_explorer_state.network_urls.is_empty() {
        lines.push(Line::from(Span::styled(
            "Press 'n' to discover network URLs",
            Style::default()
                .fg(TokyoNight::COMMENT)
                .add_modifier(Modifier::ITALIC),
        )));
    } else {
        for url in &app.api_explorer_state.network_urls {
            lines.push(Line::from(vec![
                Span::styled("  • ", Style::default().fg(TokyoNight::GREEN)),
                Span::styled(url, Style::default().fg(TokyoNight::FG)),
            ]));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("Tip: ", Style::default().fg(TokyoNight::YELLOW)),
        Span::styled(
            "These URLs can be used to access Ollama from other devices",
            Style::default().fg(TokyoNight::FG_DARK),
        ),
    ]));

    let network_urls = Paragraph::new(lines)
        .block(
            Block::default()
                .title("🌐 Network Access Points")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(TokyoNight::GREEN))
                .style(Style::default().bg(TokyoNight::BG_DARK)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(network_urls, area);
}

fn draw_discovered_services(f: &mut Frame, app: &App, area: Rect) {
    let mut lines = vec![
        Line::from(vec![Span::styled(
            "Discovered Ollama Services",
            Style::default()
                .fg(TokyoNight::MAGENTA)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
    ];

    if app.discovered_services.is_empty() {
        lines.push(Line::from(Span::styled(
            "Press 'd' to scan local network",
            Style::default()
                .fg(TokyoNight::COMMENT)
                .add_modifier(Modifier::ITALIC),
        )));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "This will scan 192.168.1.x for Ollama instances",
            Style::default().fg(TokyoNight::FG_DARK),
        )));
    } else {
        for service in &app.discovered_services {
            let status_icon = if service.is_reachable { "✅" } else { "❌" };
            lines.push(Line::from(vec![
                Span::styled(format!("  {status_icon} "), Style::default()),
                Span::styled(
                    &service.name,
                    Style::default()
                        .fg(TokyoNight::CYAN)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::raw("     "),
                Span::styled(
                    format!("{}:{}", service.ip, service.port),
                    Style::default().fg(TokyoNight::COMMENT),
                ),
            ]));
            if let Some(version) = &service.version {
                lines.push(Line::from(vec![
                    Span::raw("     "),
                    Span::styled(
                        format!("Version: {version}"),
                        Style::default().fg(TokyoNight::FG_DARK),
                    ),
                ]));
            }
            lines.push(Line::from(""));
        }
    }

    let discovered_services = Paragraph::new(lines)
        .block(
            Block::default()
                .title("🔍 Network Discovery")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(TokyoNight::MAGENTA))
                .style(Style::default().bg(TokyoNight::BG_DARK)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(discovered_services, area);
}

fn draw_view_mode_selector(f: &mut Frame, app: &App, area: Rect) {
    let mode_text = match app.models_view_mode {
        ModelsViewMode::All => "📦 All Models",
        ModelsViewMode::Installed => "✅ Installed Only",
        ModelsViewMode::Available => "🌐 Available Only",
    };

    let installed_count = app.models.len();
    let available_count = app.available_models.len();

    let stats = vec![Line::from(vec![
        Span::styled(
            mode_text,
            Style::default()
                .fg(TokyoNight::CYAN)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  │  "),
        Span::styled(
            format!("Installed: {installed_count}"),
            Style::default().fg(TokyoNight::GREEN),
        ),
        Span::raw("  │  "),
        Span::styled(
            format!("Available: {available_count}"),
            Style::default().fg(TokyoNight::YELLOW),
        ),
        Span::raw("  │  "),
        Span::styled(
            "Press 'v' to switch view",
            Style::default().fg(TokyoNight::COMMENT),
        ),
    ])];

    let selector = Paragraph::new(stats)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(TokyoNight::TERMINAL_BLACK))
                .style(Style::default().bg(TokyoNight::BG_DARK)),
        )
        .alignment(Alignment::Center);

    f.render_widget(selector, area);
}

fn draw_models_list(f: &mut Frame, app: &mut App, area: Rect) {
    let mut items: Vec<ListItem> = Vec::new();

    // Add installed models
    if (app.models_view_mode == ModelsViewMode::All
        || app.models_view_mode == ModelsViewMode::Installed)
        && !app.models.is_empty()
    {
        for model in &app.models {
            let size_str = format_size(model.size, BINARY);
            let params = model
                .details
                .as_ref()
                .map(|d| d.parameter_size.clone())
                .unwrap_or_else(|| "Unknown".to_string());

            items.push(ListItem::new(vec![
                Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled("✅ ", Style::default().fg(TokyoNight::GREEN)),
                    Span::styled(
                        &model.name,
                        Style::default()
                            .fg(TokyoNight::CYAN)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![
                    Span::raw("     "),
                    Span::styled(
                        format!("Size: {size_str} │ Params: {params}"),
                        Style::default().fg(TokyoNight::COMMENT),
                    ),
                ]),
            ]));
        }
    }

    // Add available models
    if (app.models_view_mode == ModelsViewMode::All
        || app.models_view_mode == ModelsViewMode::Available)
        && !app.available_models.is_empty()
    {
        let installed_names: Vec<String> = app.models.iter().map(|m| m.name.clone()).collect();

        for model in &app.available_models {
            let is_already_installed = installed_names.iter().any(|name| name.contains(&model.id));

            let icon = if is_already_installed {
                "🔄"
            } else {
                "☁️"
            };
            let status = if is_already_installed {
                " (update available)"
            } else {
                ""
            };

            let context = model
                .limit
                .as_ref()
                .map(|l| format!("{}k", l.context / 1000))
                .unwrap_or_else(|| "?".to_string());
            let is_open_weights = model.open_weights.unwrap_or(false);
            let weight_status = if is_open_weights { "Open" } else { "Closed" };

            items.push(ListItem::new(vec![
                Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(format!("{icon} "), Style::default()),
                    Span::styled(
                        &model.id,
                        Style::default()
                            .fg(TokyoNight::YELLOW)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        status,
                        Style::default()
                            .fg(TokyoNight::BLUE)
                            .add_modifier(Modifier::ITALIC),
                    ),
                ]),
                Line::from(vec![
                    Span::raw("     "),
                    Span::styled(
                        format!("Weights: {weight_status} │ Context: {context}"),
                        Style::default().fg(TokyoNight::COMMENT),
                    ),
                ]),
            ]));
        }
    }

    // Handle empty states
    if items.is_empty() {
        items.push(ListItem::new(vec![Line::from(vec![Span::styled(
            "No models found",
            Style::default()
                .fg(TokyoNight::COMMENT)
                .add_modifier(Modifier::ITALIC),
        )])]));
    }

    let models_list = List::new(items)
        .block(
            Block::default()
                .title("📦 Model Library")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(TokyoNight::BLUE))
                .style(Style::default().bg(TokyoNight::BG_DARK)),
        )
        .highlight_style(
            Style::default()
                .bg(TokyoNight::BG_HIGHLIGHT)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(models_list, area, &mut app.models_list_state);
}

fn draw_logs(f: &mut Frame, app: &App, area: Rect) {
    let logs: Vec<ListItem> = app
        .logs
        .iter()
        .rev()
        .map(|log| {
            let (symbol, color) = match log.level {
                LogLevel::Info => ("ℹ", TokyoNight::BLUE),
                LogLevel::Warning => ("⚠", TokyoNight::YELLOW),
                LogLevel::Error => ("✖", TokyoNight::RED),
            };

            ListItem::new(Line::from(vec![
                Span::styled(format!("{symbol} "), Style::default().fg(color)),
                Span::styled(
                    log.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
                    Style::default().fg(TokyoNight::DARK5),
                ),
                Span::raw(" "),
                Span::styled(&log.message, Style::default().fg(TokyoNight::FG)),
            ]))
        })
        .collect();

    let logs_list = List::new(logs).block(
        Block::default()
            .title("📜 System Logs")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(TokyoNight::TERMINAL_BLACK))
            .style(Style::default().bg(TokyoNight::BG_DARK)),
    );

    f.render_widget(logs_list, area);
}

fn draw_chat(f: &mut Frame, app: &App, area: Rect) {
    // Check if models are available
    if app.models.is_empty() {
        draw_no_models_message(f, area);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Messages area
            Constraint::Length(3), // Input area
            Constraint::Length(1), // Status bar
        ])
        .split(area);

    // Draw messages area
    let current_model = if app.chat_state.sessions.is_empty() {
        "No model".to_string()
    } else {
        app.chat_state.sessions[app.chat_state.active_session_index]
            .current_model
            .clone()
    };

    let messages_block = Block::default()
        .title(format!(
            "💬 Chat - {} [Session {}]",
            current_model,
            app.chat_state.active_session_index + 1
        ))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(TokyoNight::PURPLE))
        .style(Style::default().bg(TokyoNight::BG_DARK));

    let messages_inner = messages_block.inner(chunks[0]);
    f.render_widget(messages_block, chunks[0]);

    // Build messages list
    if app.chat_state.sessions.is_empty() {
        let welcome = Paragraph::new("Welcome to Chat! Press 'i' to start typing.")
            .style(Style::default().fg(TokyoNight::FG_DARK))
            .alignment(Alignment::Center);
        f.render_widget(welcome, messages_inner);
    } else {
        let session = &app.chat_state.sessions[app.chat_state.active_session_index];

        // Calculate available width for text wrapping
        let available_width = messages_inner.width.saturating_sub(4) as usize; // Account for padding

        let mut all_lines: Vec<Line> = Vec::new();

        for msg in &session.messages {
            let (prefix, style) = match msg.role {
                MessageRole::System => ("🔧 System", Style::default().fg(TokyoNight::YELLOW)),
                MessageRole::User => ("👤 You", Style::default().fg(TokyoNight::CYAN)),
                MessageRole::Assistant => ("🤖 Assistant", Style::default().fg(TokyoNight::GREEN)),
            };

            let time = msg.timestamp.format("%H:%M:%S").to_string();
            let header = format!("{prefix} [{time}]:");

            // Add header
            all_lines.push(Line::from(Span::styled(
                header,
                style.add_modifier(Modifier::BOLD),
            )));

            // Wrap message content
            let wrapped_lines = wrap_text(&msg.content, available_width);
            for line in wrapped_lines {
                all_lines.push(Line::from(Span::raw(line)));
            }

            // Add spacing
            all_lines.push(Line::from(""));
        }

        // Add current streaming response if any
        if session.is_streaming && !session.current_response.is_empty() {
            all_lines.push(Line::from(Span::styled(
                "🤖 Assistant [streaming...]:",
                Style::default()
                    .fg(TokyoNight::GREEN)
                    .add_modifier(Modifier::BOLD),
            )));

            let wrapped_lines = wrap_text(&session.current_response, available_width);
            for line in wrapped_lines {
                all_lines.push(Line::from(Span::raw(line)));
            }
            all_lines.push(Line::from(""));
        }

        // Scroll to bottom - show only the last messages that fit
        let visible_height = messages_inner.height as usize;
        let start_line = if all_lines.len() > visible_height {
            all_lines.len() - visible_height
        } else {
            0
        };

        let visible_lines: Vec<Line> = all_lines
            .into_iter()
            .skip(start_line)
            .take(visible_height)
            .collect();

        let messages_paragraph = Paragraph::new(visible_lines)
            .style(Style::default().fg(TokyoNight::FG))
            .wrap(Wrap { trim: false });

        f.render_widget(messages_paragraph, messages_inner);
    }

    // Draw input area
    let input_style = if app.chat_state.input_mode == InputMode::Editing {
        Style::default()
            .fg(TokyoNight::CYAN)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(TokyoNight::FG_DARK)
    };

    let input_title = match app.chat_state.input_mode {
        InputMode::Editing => "📝 Input (ESC to exit, Enter to send)",
        InputMode::Normal => "📝 Input (press 'i' to type)",
        InputMode::ModelSelection => "📝 Input (selecting model...)",
    };

    let input_text = if !app.chat_state.sessions.is_empty() {
        app.chat_state.sessions[app.chat_state.active_session_index]
            .input_buffer
            .as_str()
    } else {
        ""
    };

    let input = Paragraph::new(input_text)
        .block(
            Block::default()
                .title(input_title)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(input_style)
                .style(Style::default().bg(TokyoNight::BG_DARK)),
        )
        .style(Style::default().fg(TokyoNight::FG))
        .wrap(Wrap { trim: false });

    f.render_widget(input, chunks[1]);

    // Draw status bar
    let tokens = if !app.chat_state.sessions.is_empty() {
        app.chat_state.sessions[app.chat_state.active_session_index].total_tokens
    } else {
        0
    };

    let status_text = format!(
        " Tokens: {} | Mode: {} | Commands: (i)nput (c)lear (m)odel (n)ew session ",
        tokens,
        match app.chat_state.input_mode {
            InputMode::Normal => "Normal",
            InputMode::Editing => "Editing",
            InputMode::ModelSelection => "Model Selection",
        }
    );

    let status = Paragraph::new(status_text).style(
        Style::default()
            .fg(TokyoNight::FG_DARK)
            .bg(TokyoNight::BG_HIGHLIGHT),
    );

    f.render_widget(status, chunks[2]);

    // Draw model selector if active
    if app.chat_state.show_model_selector {
        draw_model_selector(f, app);
    }
}

fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut wrapped = Vec::new();

    for paragraph in text.split('\n') {
        if paragraph.is_empty() {
            wrapped.push(String::new());
            continue;
        }

        let words: Vec<&str> = paragraph.split_whitespace().collect();
        if words.is_empty() {
            wrapped.push(String::new());
            continue;
        }

        let mut current_line = String::new();

        for word in words {
            let word_len = word.chars().count();

            // If word itself is longer than max_width, break it
            if word_len > max_width {
                // Flush current line if not empty
                if !current_line.is_empty() {
                    wrapped.push(current_line.clone());
                    current_line.clear();
                }

                // Break long word into chunks
                let mut chars = word.chars();
                while !chars.as_str().is_empty() {
                    let chunk: String = chars.by_ref().take(max_width).collect();
                    wrapped.push(chunk);
                }
            } else {
                // Check if adding this word would exceed the limit
                let space_needed = if current_line.is_empty() { 0 } else { 1 };
                if current_line.chars().count() + space_needed + word_len > max_width {
                    // Start a new line
                    if !current_line.is_empty() {
                        wrapped.push(current_line.clone());
                        current_line.clear();
                    }
                    current_line.push_str(word);
                } else {
                    // Add to current line
                    if !current_line.is_empty() {
                        current_line.push(' ');
                    }
                    current_line.push_str(word);
                }
            }
        }

        // Don't forget the last line
        if !current_line.is_empty() {
            wrapped.push(current_line);
        }
    }

    wrapped
}

fn draw_no_models_message(f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(area);

    let block = Block::default()
        .title("💬 Chat - Setup Required")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(TokyoNight::YELLOW))
        .style(Style::default().bg(TokyoNight::BG_DARK));

    let inner = block.inner(chunks[1]);
    f.render_widget(block, chunks[1]);

    let help_text = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "⚠️  No models found!",
            Style::default()
                .fg(TokyoNight::YELLOW)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "To start chatting, you need to pull a model first:",
            Style::default().fg(TokyoNight::FG),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled("1. Press ", Style::default().fg(TokyoNight::FG_DARK)),
            Span::styled(
                "2",
                Style::default()
                    .fg(TokyoNight::CYAN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " to go to Models tab",
                Style::default().fg(TokyoNight::FG_DARK),
            ),
        ]),
        Line::from(vec![
            Span::styled("2. Press ", Style::default().fg(TokyoNight::FG_DARK)),
            Span::styled(
                "p",
                Style::default()
                    .fg(TokyoNight::CYAN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" to pull a model", Style::default().fg(TokyoNight::FG_DARK)),
        ]),
        Line::from(vec![
            Span::styled("3. Try: ", Style::default().fg(TokyoNight::FG_DARK)),
            Span::styled(
                "llama3.1:8b",
                Style::default()
                    .fg(TokyoNight::GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" or ", Style::default().fg(TokyoNight::FG_DARK)),
            Span::styled(
                "qwen2.5:7b",
                Style::default()
                    .fg(TokyoNight::GREEN)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Recommended starter models:",
            Style::default()
                .fg(TokyoNight::FG)
                .add_modifier(Modifier::UNDERLINED),
        )]),
        Line::from(vec![
            Span::styled("  • llama3.1:8b   ", Style::default().fg(TokyoNight::GREEN)),
            Span::styled(
                "(4.7GB, great for general chat)",
                Style::default().fg(TokyoNight::FG_DARK),
            ),
        ]),
        Line::from(vec![
            Span::styled("  • mistral:7b    ", Style::default().fg(TokyoNight::GREEN)),
            Span::styled(
                "(4.1GB, fast and efficient)",
                Style::default().fg(TokyoNight::FG_DARK),
            ),
        ]),
        Line::from(vec![
            Span::styled("  • qwen2.5:7b    ", Style::default().fg(TokyoNight::GREEN)),
            Span::styled(
                "(4.4GB, good for coding)",
                Style::default().fg(TokyoNight::FG_DARK),
            ),
        ]),
        Line::from(vec![
            Span::styled("  • gemma2:9b     ", Style::default().fg(TokyoNight::GREEN)),
            Span::styled(
                "(5.5GB, Google's model)",
                Style::default().fg(TokyoNight::FG_DARK),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(help_text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, inner);
}

fn draw_delete_confirmation(f: &mut Frame, app: &App) {
    let area = centered_rect(50, 30, f.area());
    f.render_widget(Clear, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

    let block = Block::default()
        .title("⚠️  Confirm Deletion")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(TokyoNight::RED))
        .style(Style::default().bg(TokyoNight::BG_DARK));

    f.render_widget(block, area);

    let model_name = app
        .model_to_delete
        .clone()
        .unwrap_or_else(|| String::from("Unknown"));

    let warning_text = vec![
        Line::from(""),
        Line::from(vec![Span::styled(
            "Are you sure you want to delete:",
            Style::default().fg(TokyoNight::FG),
        )]),
        Line::from(vec![Span::styled(
            &model_name,
            Style::default()
                .fg(TokyoNight::YELLOW)
                .add_modifier(Modifier::BOLD),
        )]),
    ];

    let warning = Paragraph::new(warning_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(TokyoNight::FG));

    f.render_widget(warning, chunks[1]);

    let size_info = if let Some(model) = app.models.iter().find(|m| m.name == model_name) {
        format!("Size: {}", humansize::format_size(model.size, BINARY))
    } else {
        String::from("This action cannot be undone!")
    };

    let info = Paragraph::new(size_info)
        .alignment(Alignment::Center)
        .style(Style::default().fg(TokyoNight::FG_DARK));

    f.render_widget(info, chunks[2]);

    let controls = Paragraph::new("[Y] Yes, Delete  |  [N] No, Cancel  |  [ESC] Cancel")
        .alignment(Alignment::Center)
        .style(Style::default().fg(TokyoNight::CYAN));

    f.render_widget(controls, chunks[4]);
}

fn draw_model_selector(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 60, f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .title("🎯 Select Model")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(TokyoNight::CYAN))
        .style(Style::default().bg(TokyoNight::BG_DARK));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let items: Vec<ListItem> = app
        .chat_state
        .available_models
        .iter()
        .enumerate()
        .map(|(i, model)| {
            let style = if i == app.chat_state.selected_model_index {
                Style::default()
                    .fg(TokyoNight::CYAN)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(TokyoNight::FG)
            };
            ListItem::new(model.as_str()).style(style)
        })
        .collect();

    let list = List::new(items).highlight_style(Style::default().bg(TokyoNight::BG_HIGHLIGHT));

    f.render_widget(list, inner);
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

fn draw_help(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from(vec![Span::styled(
            "✨ llamaloop - Where AI Models Dance in Infinite Elegance",
            Style::default()
                .fg(TokyoNight::CYAN)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Navigation",
            Style::default()
                .fg(TokyoNight::MAGENTA)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(Span::styled(
            "  Tab/Shift-Tab  Navigate between tabs",
            Style::default().fg(TokyoNight::FG),
        )),
        Line::from(Span::styled(
            "  1-4            Jump to specific tab",
            Style::default().fg(TokyoNight::FG),
        )),
        Line::from(Span::styled(
            "  ↑/↓            Navigate lists",
            Style::default().fg(TokyoNight::FG),
        )),
        Line::from(Span::styled(
            "  Enter          Select/Confirm",
            Style::default().fg(TokyoNight::FG),
        )),
        Line::from(Span::styled(
            "  Esc            Cancel/Back",
            Style::default().fg(TokyoNight::FG),
        )),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Commands",
            Style::default()
                .fg(TokyoNight::MAGENTA)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(Span::styled(
            "  r              Refresh data",
            Style::default().fg(TokyoNight::FG),
        )),
        Line::from(Span::styled(
            "  p              Pull model (in Models tab)",
            Style::default().fg(TokyoNight::FG),
        )),
        Line::from(Span::styled(
            "  d              Delete model (in Models tab)",
            Style::default().fg(TokyoNight::FG),
        )),
        Line::from(Span::styled(
            "  ?              Show this help",
            Style::default().fg(TokyoNight::FG),
        )),
        Line::from(Span::styled(
            "  Ctrl-C/Ctrl-Q  Quit",
            Style::default().fg(TokyoNight::FG),
        )),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Tips",
            Style::default()
                .fg(TokyoNight::MAGENTA)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(Span::styled(
            "  • Dashboard refreshes automatically",
            Style::default().fg(TokyoNight::FG_DARK),
        )),
        Line::from(Span::styled(
            "  • Watch the sparkles ✨",
            Style::default().fg(TokyoNight::FG_DARK),
        )),
        Line::from(Span::styled(
            "  • Tokyo Night theme for your eyes",
            Style::default().fg(TokyoNight::FG_DARK),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Press Esc or q to close",
            Style::default().fg(TokyoNight::COMMENT),
        )),
    ];

    let help = Paragraph::new(help_text)
        .block(
            Block::default()
                .title("🌙 Help")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(TokyoNight::CYAN))
                .style(Style::default().bg(TokyoNight::BG_DARK)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(help, area);
}

fn draw_pull_dialog(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 20, f.area());
    f.render_widget(Clear, area);

    let block = Block::default()
        .title("✨ Pull Model")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme::pulse_color(TokyoNight::CYAN, app.animation_tick)))
        .style(Style::default().bg(TokyoNight::BG_DARK));

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
        .style(Style::default().fg(TokyoNight::COMMENT))
        .alignment(Alignment::Center);
    f.render_widget(hint, chunks[0]);

    let input = Paragraph::new(app.pull_model_name.as_str())
        .style(Style::default().fg(TokyoNight::YELLOW))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(TokyoNight::TERMINAL_BLACK)),
        );
    f.render_widget(input, chunks[1]);

    let help = Paragraph::new("Press Enter to pull, Esc to cancel")
        .style(Style::default().fg(TokyoNight::DARK5))
        .alignment(Alignment::Center);
    f.render_widget(help, chunks[2]);
}

fn draw_api_endpoints_list(f: &mut Frame, app: &mut App, area: Rect) {
    let endpoints = App::get_api_endpoints();
    let items: Vec<ListItem> = endpoints
        .iter()
        .map(|endpoint| {
            let method_color = match endpoint.method.as_str() {
                "GET" => TokyoNight::GREEN,
                "POST" => TokyoNight::BLUE,
                "DELETE" => TokyoNight::RED,
                _ => TokyoNight::YELLOW,
            };

            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(
                        format!("{:7}", endpoint.method),
                        Style::default()
                            .fg(method_color)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" "),
                    Span::styled(&endpoint.name, Style::default().fg(TokyoNight::CYAN)),
                ]),
                Line::from(vec![
                    Span::raw("        "),
                    Span::styled(&endpoint.path, Style::default().fg(TokyoNight::COMMENT)),
                ]),
            ])
        })
        .collect();

    let endpoints_list = List::new(items)
        .block(
            Block::default()
                .title("🔌 API Endpoints")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(TokyoNight::BLUE))
                .style(Style::default().bg(TokyoNight::BG_DARK)),
        )
        .highlight_style(
            Style::default()
                .bg(TokyoNight::BG_HIGHLIGHT)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(
        endpoints_list,
        area,
        &mut app.api_explorer_state.endpoints_list_state,
    );
}

fn draw_request_editor(f: &mut Frame, app: &App, area: Rect) {
    let endpoints = App::get_api_endpoints();
    let selected_endpoint = endpoints.get(app.api_explorer_state.selected_endpoint);

    let content = if let Some(endpoint) = selected_endpoint {
        let mut lines = vec![
            Line::from(vec![
                Span::styled("Method: ", Style::default().fg(TokyoNight::DARK5)),
                Span::styled(
                    &endpoint.method,
                    Style::default()
                        .fg(TokyoNight::CYAN)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Path: ", Style::default().fg(TokyoNight::DARK5)),
                Span::styled(&endpoint.path, Style::default().fg(TokyoNight::GREEN)),
            ]),
            Line::from(vec![
                Span::styled("Description: ", Style::default().fg(TokyoNight::DARK5)),
                Span::styled(&endpoint.description, Style::default().fg(TokyoNight::FG)),
            ]),
            Line::from(""),
        ];

        if let Some(example) = &endpoint.example_body {
            lines.push(Line::from(Span::styled(
                "Example Request:",
                Style::default().fg(TokyoNight::YELLOW),
            )));
            for line in example.lines() {
                lines.push(Line::from(Span::styled(
                    line,
                    Style::default().fg(TokyoNight::FG_DARK),
                )));
            }
        } else {
            lines.push(Line::from(Span::styled(
                "No request body needed",
                Style::default().fg(TokyoNight::COMMENT),
            )));
        }

        lines
    } else {
        vec![Line::from(Span::styled(
            "Select an endpoint",
            Style::default().fg(TokyoNight::COMMENT),
        ))]
    };

    let request_editor = Paragraph::new(content)
        .block(
            Block::default()
                .title("📝 Request")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(TokyoNight::BLUE))
                .style(Style::default().bg(TokyoNight::BG_DARK)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(request_editor, area);
}

fn draw_response_viewer(f: &mut Frame, app: &App, area: Rect) {
    let content = if app.api_explorer_state.response_body.is_empty() {
        vec![
            Line::from(""),
            Line::from(Span::styled(
                "Press Enter to execute selected endpoint",
                Style::default().fg(TokyoNight::COMMENT),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Keyboard shortcuts:",
                Style::default()
                    .fg(TokyoNight::MAGENTA)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                "  ↑/↓     Navigate endpoints",
                Style::default().fg(TokyoNight::FG_DARK),
            )),
            Line::from(Span::styled(
                "  Enter   Execute endpoint",
                Style::default().fg(TokyoNight::FG_DARK),
            )),
            Line::from(Span::styled(
                "  e       Edit request body",
                Style::default().fg(TokyoNight::FG_DARK),
            )),
            Line::from(Span::styled(
                "  n       Discover network URLs",
                Style::default().fg(TokyoNight::FG_DARK),
            )),
            Line::from(Span::styled(
                "  d       Scan for local services",
                Style::default().fg(TokyoNight::FG_DARK),
            )),
            Line::from(Span::styled(
                "  c       Clear response",
                Style::default().fg(TokyoNight::FG_DARK),
            )),
        ]
    } else {
        app.api_explorer_state
            .response_body
            .lines()
            .map(|line| Line::from(Span::styled(line, Style::default().fg(TokyoNight::FG))))
            .collect()
    };

    let response_viewer = Paragraph::new(content)
        .block(
            Block::default()
                .title("📥 Response")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(TokyoNight::GREEN))
                .style(Style::default().bg(TokyoNight::BG_DARK)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(response_viewer, area);
}
