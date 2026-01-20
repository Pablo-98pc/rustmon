use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Clear, Gauge, Paragraph, Row, Sparkline, Table},
};
use sysinfo::{CpuRefreshKind, RefreshKind, System};

use crate::app::{AppState, InputMode};

// Nota: Importamos SystemExt y ProcessExt para tener acceso a los métodos .name(), .cpu_usage(), etc.

pub fn ui(
    f: &mut ratatui::Frame,
    sys: &System,
    state: &mut AppState,
    procesos: &Vec<&sysinfo::Process>,
) {
    // 1. Layout corregido (20% arriba, 80% abajo)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10), // Header
                Constraint::Percentage(20), // Body
                Constraint::Percentage(70),
            ]
            .as_ref(),
        )
        .split(f.area());

    let ram_usada = sys.used_memory() / 1024 / 1024;
    let ram_total = sys.total_memory() / 1024 / 1024;
    let percentage_used = (ram_usada as f64 / ram_total as f64) * 100.0;
    let sistema_operativo = sysinfo::System::name().unwrap_or("Desconocido".to_string());

    let texto_ram = format!(
        "RAM: {} MB / {} MB | SO: {}",
        ram_usada, ram_total, sistema_operativo
    );

    let info_ram = Gauge::default()
        .block(Block::default().title(" Memoria ").borders(Borders::ALL))
        .gauge_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan)) // Color opcional pero queda bonito
        .percent(percentage_used as u16) // Convertimos el f64 a u16 para el Gauge
        .label(texto_ram);

    let info_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let info_history = Sparkline::default()
        .block(Block::default().title("CPU (1 min)").borders(Borders::ALL))
        .data(&state.cpu_history)
        .style(Style::default().fg(Color::Yellow));

    let filas: Vec<Row> = procesos
        .iter()
        .map(|proceso| {
            let uso = proceso.cpu_usage();
            let color = if uso > 50.0 {
                ratatui::style::Color::Red
            } else if uso > 5.0 {
                ratatui::style::Color::Yellow
            } else {
                ratatui::style::Color::Green
            };

            Row::new(vec![
                Cell::from(proceso.pid().to_string()),
                Cell::from(format!("{:.1}%", uso))
                    .style(ratatui::style::Style::default().fg(color)),
                Cell::from(proceso.name().to_string_lossy()),
            ])
        })
        .collect();

    let anchos = [
        Constraint::Percentage(20), // PID
        Constraint::Percentage(20), // CPU
        Constraint::Percentage(60), // Nombre
    ];

    // 2. Creamos la Tabla pasando filas Y anchos

    let tabla_title = format!(" Procesos  |  Buscar:{}   ", state.input);
    let tabla = Table::new(filas, anchos) // <--- ¡AQUÍ ESTABA EL ERROR!
        .block(Block::default().title(tabla_title).borders(Borders::ALL))
        .row_highlight_style(
            ratatui::style::Style::default()
                .fg(ratatui::style::Color::Black)
                .bg(ratatui::style::Color::White),
        )
        // Opcional: Estilo de la cabecera o filas
        .header(
            Row::new(vec!["PID", "CPU", "Nombre"])
                .style(ratatui::style::Style::default().fg(ratatui::style::Color::Yellow)),
        );

    // 2. Creamos DOS bloques distintos
    let header = Block::default()
        .title(" 🦀 RustMon - Monitor de Sistema ")
        .borders(Borders::ALL);

    // 3. Renderizamos cada uno en su sitio
    f.render_widget(header, chunks[0]);
    f.render_widget(info_ram, info_chunks[0]);
    f.render_widget(info_history, info_chunks[1]);
    f.render_stateful_widget(tabla, chunks[2], &mut state.state);

    match state.mode {
        InputMode::Popup => {
            let area = centered_rect(90, 20, f.area());
            f.render_widget(Clear, area);
            let mut nombre_proceso = "Desconocido".to_string();

            // 2. Buscamos el PID guardado en el estado
            if let Some(pid) = state.pid_to_kill {
                // 3. Buscamos ese PID en la lista del sistema
                if let Some(proc) = sys.processes().get(&pid) {
                    // Si existe, actualizamos el nombre
                    nombre_proceso = proc.name().to_string_lossy().to_string();
                }
            }
            let texto_popup = format!(
                "¿Seguro que quieres matar este proceso : {} (y/n)?",
                nombre_proceso
            );

            let popup = Paragraph::new(texto_popup)
                .block(Block::default().borders(Borders::ALL).title(" Alerta "))
                .style(Style::default().fg(Color::Red));
            f.render_widget(popup, area);
        }
        InputMode::Normal => {}
    }
}

// Helper para el futuro popup
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
