use std::io;
use crossterm::{
    event::{self, Event, KeyCode,KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use sysinfo::{System}; // Importante: SystemExt para refresh_all

// --- MÓDULOS ---
// Esto le dice a Rust: "Busca los ficheros app.rs y ui.rs"
mod app;
mod ui;

use app::AppState;

use crate::app::InputMode;

fn main() -> Result<(), io::Error> {
    // 1. SETUP
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Inicializamos estado y sistema
    let mut sys = System::new_all();
    let mut app_state = AppState::new();

    // 2. LOOP
    loop {
        sys.refresh_all();

        let mut procesos: Vec<_> = sys.processes().values().filter(|proceso|{
            proceso.name().to_string_lossy().to_lowercase().contains(&app_state.input.to_lowercase())
        }).collect();

        procesos.sort_by(|a,b| {
            b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal)
        });

        app_state.on_tick(sys.global_cpu_usage() as f64);

        terminal.draw(|f| {
            // Llamamos a la función ui que está en el módulo ui
            ui::ui(f, &sys, &mut app_state,&procesos);
        })?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match app_state.mode {
                    InputMode::Normal => {
                            match key.code {
                                KeyCode::Char('q') =>  {
                                break;
                            }, KeyCode::Down =>  {
                                app_state.next(sys.processes().len());
                            }, KeyCode::Up => {
                                app_state.previous();
                            }, KeyCode::Char('k') => {
                                let index = app_state.state.selected();
                                match index {
                                    Some(i) => {
                                        let process_to_kill = procesos[i];
                                        app_state.pid_to_kill = Some(process_to_kill.pid());
                                        app_state.mode = InputMode::Popup;
                                    },
                                    None => {

                                    }
                                }
                            }, KeyCode::Char(c) => {
                                app_state.input.push(c);
                            }, KeyCode::Backspace => {
                                app_state.input.pop();
                            }, KeyCode::Esc => {
                                app_state.input.clear();
                            }
                            _ => {}
                        }
                    },
                    InputMode::Popup => {
                        match key.code {
                            KeyCode::Char('y') => {
                                if let Some(pid) = app_state.pid_to_kill {
                                    if let Some(process) = sys.processes().get(&pid){
                                        process.kill();
                                    }
                                }
                                app_state.mode = InputMode::Normal;
                                app_state.pid_to_kill = None;
                            }, KeyCode::Char('c')=> {
                                app_state.mode = InputMode::Normal;
                                app_state.pid_to_kill = None;
                            }, _ => {

                            }
                        }
                    }
                }
               
            }
        }
    }

    // 3. SHUTDOWN
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}