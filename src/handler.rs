use crossterm::event::KeyModifiers;

use crate::{
    cursor, execute, read, sleep, stdout, terminal, Arc, Clear, ClearType, Color, Duration, Event,
    Grid, KeyCode, Mutex, Print, SetForegroundColor, Side, GRID,
};

pub fn event_handler(
    size_terminal: Arc<Mutex<(u16, u16)>>,
    where_go: Arc<Mutex<Side>>,
    gd: Arc<Mutex<Grid>>,
    coin: Arc<Mutex<usize>>,
    timer_end: Arc<Mutex<u16>>,
    exi: Arc<Mutex<bool>>,
) {
    terminal::enable_raw_mode().unwrap();
    loop {
        if let Ok(event) = read() {
            terminal::disable_raw_mode().unwrap();
            if (gd.lock().unwrap()).figure.is_some() {
                match event {
                    Event::Key(key) => match key.code {
                        KeyCode::Left => {
                            let gd_unlock = gd.lock().unwrap();

                            if gd_unlock.current_cord.unwrap()[0] > 0 {
                                gd_unlock.current_cord.unwrap()[0] -= 1;
                                *(where_go.lock().unwrap()) = Side::Left;
                            }
                        }
                        KeyCode::Right => {
                            let gd_unlock = gd.lock().unwrap();
                            if gd_unlock.current_cord.unwrap()[0] < 19 {
                                gd_unlock.current_cord.unwrap()[0] += 1;
                                *(where_go.lock().unwrap()) = Side::Right;
                            }
                        }
                        KeyCode::Up => {
                            *(where_go.lock().unwrap()) = Side::Up;
                        }
                        KeyCode::Down => {
                            while let Some(_) =
                                (gd.lock().unwrap()).move_down(&mut 0, *(timer_end.lock().unwrap()))
                            {
                            }
                        }
                        KeyCode::Esc => {
                            (gd.lock().unwrap()).grid = GRID;
                            *(coin.lock().unwrap()) = 0;
                        }
                        KeyCode::Char('C' | 'c') if key.modifiers == KeyModifiers::CONTROL => {
                            terminal::disable_raw_mode().unwrap();
                            *exi.lock().unwrap() = true;
                            return;
                        }
                        KeyCode::Char(c) => match c.to_ascii_lowercase() {
                            'a' | 'j' => {
                                let gd_unlock = gd.lock().unwrap();
                                if gd_unlock.current_cord.unwrap()[0] > 0 {
                                    gd_unlock.current_cord.unwrap()[0] -= 1;
                                    *(where_go.lock().unwrap()) = Side::Left;
                                }
                            }
                            'd' | 'l' => {
                                let gd_unlock = gd.lock().unwrap();
                                if gd_unlock.current_cord.unwrap()[0] < 19 {
                                    gd_unlock.current_cord.unwrap()[0] += 1;
                                    *(where_go.lock().unwrap()) = Side::Right;
                                }
                            }
                            's' | 'k' => {
                                while let Some(_) = (gd.lock().unwrap())
                                    .move_down(&mut 0, *(timer_end.lock().unwrap()))
                                {
                                }
                            }
                            'w' | 'i' => *(where_go.lock().unwrap()) = Side::Up,

                            'p' | 'P' => {
                                let _a = gd.lock();
                                loop {
                                    let k = read().unwrap();
                                    if let Event::Key(key) = k {
                                        if let KeyCode::Char('p' | 'P') = key.code {
                                            break;
                                        }
                                        if let KeyCode::Char('C' | 'c') = key.code {
                                            if key.modifiers == KeyModifiers::CONTROL {
                                                terminal::disable_raw_mode().unwrap();
                                                *exi.lock().unwrap() = true;
                                                return;
                                            }
                                        }
                                    } else if let Event::Resize(x, y) = k {
                                        let mut lock_size = size_terminal.lock().unwrap();
                                        if lock_size.0 == x || lock_size.1 == y {
                                            execute!(stdout(), cursor::Hide, Clear(ClearType::All))
                                                .unwrap();
                                        }
                                        *lock_size = (x, y);
                                        execute!(
                                            stdout(),
                                            cursor::MoveTo(
                                                lock_size.0 / 2 - 10 - 20,
                                                lock_size.1 - 1
                                            )
                                        )
                                        .unwrap();
                                        drop(lock_size);
                                        execute!(stdout(), SetForegroundColor(Color::White))
                                            .unwrap();
                                        execute!(
                                                stdout(),
                                                Print("←→↑↓/adws/jlik for movement! p - pause! Esc - restart! CTRL-C to quit!")
                                            )
                                            .unwrap();
                                    }
                                }
                            }

                            _ => (),
                        },

                        _ => (),
                    },
                    Event::Mouse(_) => (),
                    Event::Resize(x, y) => {
                        let mut lock_size = size_terminal.lock().unwrap();
                        if lock_size.0 == x || lock_size.1 == y {
                            execute!(stdout(), cursor::Hide, Clear(ClearType::All)).unwrap();
                        }
                        *lock_size = (x, y);
                        execute!(
                            stdout(),
                            cursor::MoveTo(lock_size.0 / 2 - 10 - 20, lock_size.1 - 1)
                        )
                        .unwrap();
                        drop(lock_size);
                        execute!(stdout(), SetForegroundColor(Color::White)).unwrap();
                        execute!(
                            stdout(),
                            Print("←→↑↓/adws/jlik for movement! p - pause! Esc - restart! CTRL-C to quit!")
                        )
                        .unwrap();
                    }
                }
            }
        }
        terminal::enable_raw_mode().unwrap();
    }
}
