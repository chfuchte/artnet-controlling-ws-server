use crossterm::event;
use crossterm::execute;
use crossterm::style;
use crossterm::terminal;

mod state {
    pub struct AppState {
        cols: u16,
        rows: u16,
    }

    impl AppState {
        pub fn new(term_size: (u16, u16)) -> Self {
            AppState {
                cols: term_size.0,
                rows: term_size.1,
            }
        }

        pub fn term_size(&mut self) -> (u16, u16) {
            (self.cols, self.rows)
        }

        pub fn set_term_size(&mut self, cols: u16, rows: u16) {
            self.cols = cols;
            self.rows = rows;
        }
    }
}

pub fn main_loop(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    let mut state = state::AppState::new(terminal::size()?);

    loop {
        if event::poll(std::time::Duration::from_millis(10))? {
            match event::read()? {
                event::Event::Key(event) => match event.modifiers {
                    event::KeyModifiers::CONTROL => match event.code {
                        event::KeyCode::Char('c') => {
                            break;
                        }
                        _ => {
                            execute!(stdout, style::Print("Ctrl + "))?;
                            execute!(stdout, style::Print(&event.code))?;
                        }
                    },
                    _ => {}
                },
                event::Event::Resize(new_cols, new_rows) => {
                    state.set_term_size(new_cols, new_rows);
                }
                _ => {}
            }
        }
    }

    Ok(())
}
