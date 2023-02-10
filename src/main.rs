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

pub use grid::{Grid, Side, GRID, State, SizeTerminal};
pub use matrix::MatrixPoint4X;
use rand::{thread_rng, Rng};
pub use std::io::stdout;
pub use std::sync::{Arc, Mutex};
pub use std::time::Duration;
pub use std::{thread, thread::sleep};
pub use types_figures::{get_figure_matrix, get_random_figure, Figures};

fn main() -> crossterm::Result<()> {
    let mut size_terminal = terminal::size().unwrap();
    init(size_terminal)?;


    let mut gener_rand = thread_rng();
    let gd = Arc::new(Mutex::new(Grid::new()));
    let info = Arc::new(Mutex::new(None));
    let _thread_move_down = Grid::move_down_sleep(gd.clone(), info.clone());
    let mut state = State::new(SizeTerminal::new(), info, 0);
    #[allow(unused_mut)] let mut other_gd = Arc::new(Mutex::new(Grid::new()));
    let mut other_state = State::new(SizeTerminal::new(), Arc::new(Mutex::new(None)), 0);



    let game = loop {
        break Game::Online;
    };
    
    if let Game::Online = game {
        state.set_mixer(-20);
        other_state.set_mixer(20);
    } else {
        // state.set_mixer(0) - default
    }
    

    loop {
        match game {
            Game::Offline => {
                let res = state.size_terminal.update();
                Grid::run_offline(&gd, &mut state, gener_rand.gen::<u8>())?;
            }
            Game::Online => {
                let res1 = state.size_terminal.update();            
                let res2 = other_state.size_terminal.update();

                Grid::run_online(&gd, &mut state, gener_rand.gen::<u8>(), &other_gd, &mut other_state)?;
            }
        }


        size_terminal = terminal::size().unwrap();

        if size_terminal.0 >= 70 {
            execute!(
                stdout(),
                cursor::MoveTo(size_terminal.0 / 2 - 35, size_terminal.1-1),
                SetForegroundColor(Color::White),
                Print("←→↑↓/adws/jlik for movement! p - pause! Esc - restart! CTRL-C to quit!")
            )?;
        }

        if state.is_exi() || size_terminal.0 < 23 {
            execute!(stdout(), terminal::LeaveAlternateScreen, cursor::Show)?;
            terminal::disable_raw_mode()?;
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
        Clear(ClearType::All),
        cursor::MoveTo(size_terminal.0 / 2 - 35, size_terminal.1-1),
        SetForegroundColor(Color::White),
        Print("←→↑↓/adws/jlik for movement! p - pause! Esc - restart! CTRL-C to quit!")
    )?;
    terminal::enable_raw_mode()?;
    Ok(())
}
