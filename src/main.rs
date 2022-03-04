use std::io::stdout;

use rand::{thread_rng, Rng};

use std::time::Duration;

use crossterm::event::KeyCode;

use crossterm::{
    cursor,
    event::{poll, read, Event},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    Result,
};

mod matrix;
use matrix::MatrixPoint4X;

mod types_figures;
pub use types_figures::{get_figure_matrix, get_random_figure, Figures};

mod grid;
use grid::{Grid, Side, GRID};

fn main() -> Result<()> {
    let mut gener_rand = thread_rng();
    let mut gd = Grid::new();

    let (mut xs, mut ys) = terminal::size().unwrap();

    let mut point_start: u16;
    let ind_y = 4;

    let mut coin: usize = 0;

    let mut where_go: Side = Side::Stop;

    execute!(
        stdout(),
        terminal::EnterAlternateScreen,
        cursor::Hide,
        Clear(ClearType::All)
    )?;

    let mut timer: u16 = 0;

    loop {
        timer += 1;
        {
            execute!(stdout(), SetForegroundColor(Color::Green))?;
            execute!(stdout(), cursor::MoveTo(xs / 2, 2))?;
            execute!(stdout(), Print(coin))?;
            execute!(stdout(), SetForegroundColor(Color::Yellow))?;
            point_start = xs / 2 - gd.grid[0].len() as u16 / 2;
            execute!(stdout(), cursor::MoveTo(point_start, ind_y))?;
            execute!(stdout(), Print("____________________"))?;
            for line in gd.grid.iter().enumerate() {
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
            match gd.current_cord {
                Some([x, _]) => {
                    execute!(
                        stdout(),
                        cursor::MoveTo(point_start + x as u16, 21 + ind_y + 1)
                    )?;
                    execute!(stdout(), Print("^"))?;
                }
                _ => (),
            }
            execute!(stdout(), cursor::MoveTo(point_start - 17, ys - 1))?;
            execute!(stdout(), SetForegroundColor(Color::White))?;
            execute!(
                stdout(),
                Print("←, →, ↑, ↓ for movement! Esc - restart! CTRL-C to quit!")
            )?;
        }

        if gd.current_cord.is_none() || gd.move_down(&mut timer).is_none() {
            gd.add_figure(get_random_figure(gener_rand.gen::<u8>() % 7));
            gd.ready_clean(&mut coin);
        } else {
            gd.move_to_side(&mut where_go);
        }

        if let Ok(true) = poll(Duration::from_millis(5)) {
            handle_event((&mut xs, &mut ys), &mut where_go, &mut gd, &mut coin);
        }
    }
}

fn handle_event(
    size_terminal: (&mut u16, &mut u16),
    where_go: &mut Side,
    gd: &mut Grid,
    coin: &mut usize,
) {
    if let Ok(event) = read() {
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Left => {
                    if gd.current_cord.unwrap()[0] > 0 {
                        gd.current_cord.unwrap()[0] -= 1;
                        *where_go = Side::Left;
                    }
                }
                KeyCode::Right => {
                    if gd.current_cord.unwrap()[0] < 19 {
                        gd.current_cord.unwrap()[0] += 1;
                        *where_go = Side::Right;
                    }
                }
                KeyCode::Up => {
                    *where_go = Side::Up;
                }
                KeyCode::Down => {
                    while let Some(_) = gd.move_down(&mut 5) {}
                    gd.move_down(&mut 5);
                }
                KeyCode::Esc => {
                    gd.grid = GRID;
                    *coin = 0;
                    execute!(stdout(), cursor::MoveTo(*size_terminal.0 / 2, 2)).unwrap();
                    execute!(stdout(), Print("            ")).unwrap();
                }
                _ => (),
            },
            Event::Mouse(_) => (),
            Event::Resize(x, y) => {
                if *size_terminal.0 == x || *size_terminal.1 == y {
                    execute!(stdout(), cursor::Hide, Clear(ClearType::All)).unwrap();
                }
                *size_terminal.0 = x;
                *size_terminal.1 = y;
            }
        }
    }
}
