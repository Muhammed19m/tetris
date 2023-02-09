mod grid;
mod handler;
mod matrix;
mod types_figures;

pub use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use grid::State;
pub use grid::{Grid, Side, GRID};
pub use matrix::MatrixPoint4X;
use rand::{thread_rng, Rng};
pub use std::io::stdout;
pub use std::sync::{Arc, Mutex};
pub use std::time::Duration;
pub use std::{thread, thread::sleep};
pub use types_figures::{get_figure_matrix, get_random_figure, Figures};

fn main() -> crossterm::Result<()> {
    let mut gener_rand = thread_rng();
    let gd = Arc::new(Mutex::new(Grid::new()));
    let mut size_terminal = terminal::size().unwrap();
    let info = Arc::new(Mutex::new(None));
    let _thread_move_down = Grid::move_down_sleep(gd.clone(), info.clone());
    let mut state = State::new(size_terminal, info, 0);
    let mut other_gd = Grid::new();
    let mut other_state = State::new((100, 4), Arc::new(Mutex::new(None)), 0);

    init(size_terminal)?;

    let game = loop {
        break Game::Offline;
    };
    
    if let Game::Online = game {
        state.set_mixer(-20)
    }

    loop {
        size_terminal = terminal::size().unwrap();

        match game {
            Game::Offline => {
                state.size_terminal = size_terminal;
                Grid::run_offline(&gd, &mut state, gener_rand.gen::<u8>())?;
            }
            Game::Online => {}
        }

        if size_terminal.0 >= 70 {
            execute!(
                stdout(),
                cursor::MoveTo(size_terminal.0 / 2 - 35, size_terminal.1),
                SetForegroundColor(Color::White),
                Print("←→↑↓/adws/jlik for movement! p - pause! Esc - restart! CTRL-C to quit!")
            )
            .unwrap();
        }

        if state.is_exi() || size_terminal.0 < 23 {
            execute!(stdout(), terminal::LeaveAlternateScreen, cursor::Show)?;
            terminal::disable_raw_mode().unwrap();
            break;
        }
    }
    Ok(())
}

enum Game {
    Offline,
    Online,
}

fn init(size_terminal: (u16, u16)) -> crossterm::Result<()> {
    execute!(
        stdout(),
        terminal::EnterAlternateScreen,
        cursor::Hide,
        Clear(ClearType::All)
    )?;

    execute!(
        stdout(),
        cursor::MoveTo(size_terminal.0 / 2 - 35, size_terminal.1)
    )
    .unwrap();

    execute!(stdout(), SetForegroundColor(Color::White)).unwrap();
    execute!(
        stdout(),
        Print("←→↑↓/adws/jlik for movement! p - pause! Esc - restart! CTRL-C to quit!")
    )
    .unwrap();
    terminal::enable_raw_mode().unwrap();
    Ok(())
}
