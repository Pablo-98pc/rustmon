use ratatui::widgets::TableState;
use sysinfo::Pid;

pub enum InputMode {
    Normal,
    Popup
}

pub struct AppState {
    pub state: TableState,
    pub mode : InputMode,
    pub pid_to_kill: Option<Pid>,
    pub cpu_history : Vec<u64>,
    pub input: String    
    // Aquí pondremos pronto el 'mode' y 'pid_to_kill'
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            state: TableState::default(),
            mode:InputMode::Normal,
            pid_to_kill:None,
            cpu_history: Vec::new(),
            input: String::new()
        }
    }

    pub fn next(&mut self, total_items: usize) {
        match self.state.selected() {
            None => self.state.select(Some(0)),
            Some(i) => {
                if i < total_items - 1 {
                    self.state.select(Some(i + 1));
                }
            }
        }
    }

    pub fn previous(&mut self) {
        match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.state.select(Some(0)); // Opcional: quedarse en 0
                } else {
                    self.state.select(Some(i - 1));
                }
            }
            None => self.state.select(Some(0)),
        }
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn on_tick (&mut self, cpu_usage: f64){
        self.cpu_history.push(cpu_usage as u64);
        if self.cpu_history.len() >= 40 {
            self.cpu_history.remove(0);
        }
    }
}