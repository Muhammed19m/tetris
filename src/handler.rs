use crossterm::event::KeyModifiers;

use crate::{
    cursor, execute, read, stdout, terminal, Clear, ClearType, Event, Grid, KeyCode, Side, GRID,
};

pub fn event_handler_poll(where_go: &mut Side, gd: &mut Grid, coin: &mut usize, exi: &mut bool) {
    terminal::enable_raw_mode().unwrap();

    if let Ok(event) = read() {
        terminal::disable_raw_mode().unwrap();
        if gd.figure.is_some() {
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
                    KeyCode::Down => while let Some(_) = gd.move_down(&mut 0) {},
                    KeyCode::Esc => {
                        gd.grid = GRID;
                        *coin = 0;
                    }
                    KeyCode::Char('C' | 'c') if key.modifiers == KeyModifiers::CONTROL => {
                        terminal::disable_raw_mode().unwrap();
                        *exi = true;
                        return;
                    }
                    KeyCode::Char(c) => match c.to_ascii_lowercase() {
                        'a' | 'j' => {
                            if gd.current_cord.unwrap()[0] > 0 {
                                gd.current_cord.unwrap()[0] -= 1;
                                *where_go = Side::Left;
                            }
                        }
                        'd' | 'l' => {
                            if gd.current_cord.unwrap()[0] < 19 {
                                gd.current_cord.unwrap()[0] += 1;
                                *where_go = Side::Right;
                            }
                        }
                        's' | 'k' => while let Some(_) = gd.move_down(&mut 0) {},
                        'w' | 'i' => *where_go = Side::Up,

                        'p' | 'P' => {
                            let mut resize = false;
                            loop {
                                let k = read().unwrap();
                                if let Event::Key(key) = k {
                                    if let KeyCode::Char('p' | 'P') = key.code {
                                        break;
                                    }
                                    if let KeyCode::Char('C' | 'c') = key.code {
                                        if key.modifiers == KeyModifiers::CONTROL {
                                            terminal::disable_raw_mode().unwrap();
                                            *exi = true;
                                            return;
                                        }
                                    }
                                } else if let Event::Resize(..) = k {
                                    resize = true;
                                }
                            }
                            if resize {
                                execute!(stdout(), cursor::Hide, Clear(ClearType::All)).unwrap();
                            }
                        }

                        _ => (),
                    },

                    _ => (),
                },
                Event::Mouse(_) => (),
                Event::Resize(..) => {
                    execute!(stdout(), cursor::Hide, Clear(ClearType::All)).unwrap();
                }
            }
        }
    }
    terminal::enable_raw_mode().unwrap();
}
