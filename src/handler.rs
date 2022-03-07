use crate::{
    cursor, execute, read, sleep, stdout, Arc, Clear, ClearType, Duration, Event, Grid, KeyCode,
    Mutex, Print, Side, GRID,
};

pub fn event_handler(
    size_terminal: Arc<Mutex<(u16, u16)>>,
    where_go: Arc<Mutex<Side>>,
    gd: Arc<Mutex<Grid>>,
    coin: Arc<Mutex<usize>>,
    timer_end: Arc<Mutex<u16>>,
) {
    loop {
        if let Ok(event) = read() {
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
                            '+' => {
                                let mut timer_lock = *(timer_end.lock().unwrap());
                                if timer_lock > 2 && timer_lock <= 30 {
                                    timer_lock -= 1;
                                }
                            }
                            '-' => {
                                let mut timer_lock = *(timer_end.lock().unwrap());
                                if timer_lock < 30 {
                                    timer_lock += 1;
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
                    }
                }
            }
        }
    }
}
