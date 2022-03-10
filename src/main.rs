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
pub use grid::{Grid, Side, GRID};
use handler::event_handler_poll;
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
    let mut point_start: u16;
    let ind_y = 4;
    let mut coin = 0;
    let mut where_go = Side::Stop;
    let mut exi = false;
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

    let info = Arc::new(Mutex::new(None));

    let _thread_move_down = Grid::move_down_sleep(gd.clone(), info.clone());

    loop {
        size_terminal = terminal::size().unwrap();

        {
            execute!(stdout(), SetForegroundColor(Color::Green))?;
            execute!(stdout(), cursor::MoveTo(size_terminal.0 / 2, 2))?;

            if coin == 0 {
                execute!(stdout(), Print("0           "))?;
            } else {
                execute!(stdout(), Print(coin))?;
            }

            execute!(stdout(), SetForegroundColor(Color::Yellow))?;
            point_start = size_terminal.0 / 2 - 10;

            execute!(stdout(), cursor::MoveTo(point_start, ind_y))?;
            execute!(stdout(), Print("____________________"))?;

            for line in (gd.lock().unwrap()).grid.iter().enumerate() {
                execute!(
                    stdout(),
                    cursor::MoveTo(point_start - 1, (line.0 + 1) as u16 + ind_y)
                )?;
                execute!(stdout(), Print('|'))?;
                for item in line.1 {
                    if *item == 1 {
                        execute!(stdout(), Print("O"))?;
                    } else {
                        execute!(stdout(), Print(" "))?;
                    }
                }
                execute!(stdout(), SetForegroundColor(Color::Yellow))?;
                execute!(
                    stdout(),
                    cursor::MoveTo(point_start + 20, (line.0 + 1) as u16 + ind_y)
                )?;
                execute!(stdout(), Print('|'))?;
            }

            execute!(stdout(), cursor::MoveTo(point_start - 1, 21 + ind_y))?;
            execute!(stdout(), Print("——————————————————————"))?;
            execute!(stdout(), cursor::MoveTo(point_start - 1, 21 + ind_y + 1))?;
            execute!(stdout(), Print("                      "))?;
            match (gd.lock().unwrap()).current_cord {
                Some([x, _]) => {
                    execute!(
                        stdout(),
                        cursor::MoveTo(point_start + x as u16, 21 + ind_y + 1)
                    )?;
                    execute!(stdout(), Print("^"))?;
                }
                _ => (),
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
        }
        let mut lock_gd = gd.lock().unwrap();
        if lock_gd.current_cord.is_none() || (info.lock().unwrap()).is_none() {
            lock_gd.add_figure(get_random_figure(gener_rand.gen::<u8>() % 7));
            lock_gd.ready_clean(&mut coin);
        } else {
            lock_gd.move_to_side(&mut where_go);
        }
        terminal::enable_raw_mode().unwrap();
        if poll(Duration::from_millis(10)).unwrap() {
            event_handler_poll(&mut where_go, &mut lock_gd, &mut coin, &mut exi)
        }

        if exi || size_terminal.0 < 23 {
            execute!(stdout(), terminal::LeaveAlternateScreen, cursor::Show)?;
            terminal::disable_raw_mode().unwrap();
            break;
        }
    }
    Ok(())
}
