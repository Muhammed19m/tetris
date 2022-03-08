const SPEED: u16 = 200;
// пока при компиляции надо менять, чем меньше число тем быстрее падает фигурка

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
use handler::event_handler;
pub use matrix::MatrixPoint4X;
use rand::{thread_rng, Rng};
pub use std::io::stdout;
pub use std::sync::{Arc, Mutex};
use std::time::Duration;
pub use std::{thread, thread::sleep};
pub use types_figures::{get_figure_matrix, get_random_figure, Figures};

fn main() -> crossterm::Result<()> {
    let mut gener_rand = thread_rng();
    let gd = Arc::new(Mutex::new(Grid::new()));
    let size_terminal = Arc::new(Mutex::new(terminal::size().unwrap()));
    let mut point_start: u16;
    let ind_y = 4;
    let coin = Arc::new(Mutex::new(0));
    let where_go = Arc::new(Mutex::new(Side::Stop));
    let mut timer: u16 = 0;
    let timer_end = Arc::new(Mutex::new(5));
    let exi = Arc::new(Mutex::new(false));
    execute!(
        stdout(),
        terminal::EnterAlternateScreen,
        cursor::Hide,
        Clear(ClearType::All)
    )?;

    let (size_terminal_clone, where_go_clone, gd_clone, coin_clone, timer_end_clone) = (
        size_terminal.clone(),
        where_go.clone(),
        gd.clone(),
        coin.clone(),
        timer_end.clone(),
    );
    let exi_clone = exi.clone();

    thread::spawn(|| {
        event_handler(
            size_terminal_clone,
            where_go_clone,
            gd_clone,
            coin_clone,
            timer_end_clone,
            exi_clone,
        )
    });
    let lock_s = size_terminal.lock().unwrap();
    execute!(
        stdout(),
        cursor::MoveTo(lock_s.0 / 2 - 10 - 20, lock_s.1 - 1)
    )
    .unwrap();

    execute!(stdout(), SetForegroundColor(Color::White)).unwrap();
    execute!(
        stdout(),
        Print("←→↑↓/adws/jlik for movement! p - pause! Esc - restart! CTRL-C to quit!")
    )
    .unwrap();
    drop(lock_s);

    loop {
        timer += 1;
        {
            execute!(stdout(), SetForegroundColor(Color::Green))?;
            execute!(
                stdout(),
                cursor::MoveTo((size_terminal.lock().unwrap()).0 / 2, 2)
            )?;

            let coin_lock = *coin.lock().unwrap();
            if coin_lock == 0 {
                execute!(stdout(), Print("0           "))?;
            } else {
                execute!(stdout(), Print(coin_lock))?;
            }
            drop(coin_lock);

            execute!(stdout(), SetForegroundColor(Color::Yellow))?;
            point_start = (size_terminal.lock().unwrap()).0 / 2 - 10;

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
        }

        let mut gd_lock = gd.lock().unwrap();

        if gd_lock.current_cord.is_none()
            || gd_lock
                .move_down(&mut timer, *timer_end.lock().unwrap())
                .is_none()
        {
            gd_lock.add_figure(get_random_figure(gener_rand.gen::<u8>() % 7));
            gd_lock.ready_clean(coin.lock().unwrap());
        } else {
            gd_lock.move_to_side(where_go.lock().unwrap());
        }
        drop(gd_lock);

        if *exi.lock().unwrap() {
            break;
        }
    }
    Ok(())
}
