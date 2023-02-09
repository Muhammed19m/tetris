pub const GRID: [[u8; 20]; 20] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

use std::io::stdout;

use crossterm::{
    cursor,
    event::poll,
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal,
};

use crate::{
    get_figure_matrix, get_random_figure, handler::event_handler_poll, sleep, thread, Arc,
    Duration, Figures, MatrixPoint4X, Mutex,
};

#[derive(PartialEq)]
pub enum Side {
    Left,
    Right,
    Stop,
    Up,
}

pub struct Grid {
    pub grid: [[u8; 20]; 20],
    pub current_cord: Option<[u8; 2]>,
    pub figure: Option<MatrixPoint4X>,
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            grid: GRID,
            current_cord: None,
            figure: None,
        }
    }
    pub fn add_figure(&mut self, type_figure: Figures) {
        self.current_cord = Some([10, 0]);
        self.figure = Some(MatrixPoint4X::new(get_figure_matrix(type_figure)));
    }

    pub fn move_down(&mut self, value: u8, current_value: u8) -> Option<()> {
        if let Some(ref mut c) = self.current_cord {
            draw_points(&mut self.grid, &self.figure.unwrap().arr, *c, 0);
            for line in self.figure.unwrap().arr.into_iter().enumerate() {
                for item in line.1.into_iter().enumerate() {
                    if item.1 == 1 {
                        if c[1] + (line.0 as u8) + 1 < 20 {
                            if self.grid[c[1] as usize + 1 + line.0][c[0] as usize + item.0]
                                == current_value
                            {
                                draw_points(&mut self.grid, &self.figure.unwrap().arr, *c, value);
                                return None;
                            }
                        } else {
                            draw_points(&mut self.grid, &self.figure.unwrap().arr, *c, value);
                            return None;
                        }
                    }
                }
            }

            c[1] += 1;

            draw_points(&mut self.grid, &self.figure.unwrap().arr, *c, value);
            return Some(());
        }
        return None;
    }

    pub fn move_to_side(&mut self, l_r: &mut Side) {
        let backup_cord = self.current_cord.clone();
        self.grid
            .iter_mut()
            .map(|line| line.iter_mut().filter(|i| **i == 2).map(|i| *i = 0).count())
            .count();
        while let Some(_) = self.move_down(2, 1) {}
        self.current_cord = backup_cord;

        if let Some(ref mut c) = self.current_cord {
            match *l_r {
                Side::Left => {
                    draw_points(&mut self.grid, &self.figure.unwrap().arr, *c, 0);

                    if is_side(self.grid, &self.figure.unwrap(), *c, -1) {
                        c[0] -= 1;
                    }
                    draw_points(&mut self.grid, &self.figure.unwrap().arr, *c, 1);
                    *l_r = Side::Stop;
                }

                Side::Right => {
                    draw_points(&mut self.grid, &self.figure.unwrap().arr, *c, 0);
                    if is_side(self.grid, &self.figure.unwrap(), *c, 1) {
                        c[0] += 1;
                    }
                    draw_points(&mut self.grid, &self.figure.unwrap().arr, *c, 1);

                    *l_r = Side::Stop;
                }
                Side::Up => {
                    if let Some(ref mut a) = self.figure {
                        draw_points(&mut self.grid, &a.arr, *c, 0);

                        a.upend();
                        for line in a.arr.into_iter().enumerate() {
                            for item in line.1.into_iter().enumerate() {
                                if item.1 == 1 {
                                    if c[0] as usize + item.0 < 20 && c[1] as usize + line.0 < 20 {
                                        if self.grid[c[1] as usize + line.0][c[0] as usize + item.0]
                                            == 1
                                        {
                                            a.upend();
                                            a.upend();
                                            a.upend();
                                            draw_points(&mut self.grid, &a.arr, *c, 1);
                                            *l_r = Side::Stop;
                                            return;
                                        }
                                    } else {
                                        a.upend();
                                        a.upend();
                                        a.upend();
                                        draw_points(&mut self.grid, &a.arr, *c, 1);
                                        *l_r = Side::Stop;
                                        return;
                                    }
                                }
                            }
                        }

                        draw_points(&mut self.grid, &a.arr, *c, 1);
                    };
                    *l_r = Side::Stop;
                }
                _ => draw_points(&mut self.grid, &self.figure.unwrap().arr, *c, 1),
            }
        }
    }

    pub fn ready_clean(&mut self, coin: &mut usize) {
        for line in self.grid.clone().into_iter().enumerate() {
            if line.1.into_iter().all(|i| i == 1) {
                for i in (0..=line.0).rev() {
                    if i > 0 {
                        self.grid[i] = self.grid[i - 1];
                    } else {
                        self.grid[i] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
                    }
                }
                *coin += 1;
            }
        }
    }
    pub fn move_down_sleep(
        gd: Arc<Mutex<Grid>>,
        info: Arc<Mutex<Option<()>>>,
    ) -> thread::JoinHandle<()> {
        thread::spawn(move || loop {
            sleep(Duration::from_millis(300));
            *(info.lock().unwrap()) = (gd.lock().unwrap()).move_down(1, 1);
        })
    }

    pub fn render(&self, state: &mut State, rand: u8) -> crossterm::Result<()> { 
        unimplemented!();
        execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            cursor::MoveTo(state.size_terminal.0 / 2, 2)
        )?;
        state.set_start(state.size_terminal.0);

        if state.coin == 0 {
            execute!(stdout(), Print("0           "))?;
        } else {
            execute!(stdout(), Print(state.coin))?;
        }
        state.point_start = state.size_terminal.0 / 2 - 10;

        execute!(
            stdout(),
            SetForegroundColor(Color::Yellow),
            cursor::MoveTo(state.point_start, state.ind_y),
            Print("____________________")
        )?;

        for line in self.grid.iter().enumerate() {
            execute!(
                stdout(),
                cursor::MoveTo(state.point_start - 1, (line.0 + 1) as u16 + state.ind_y),
                Print('|')
            )?;
            for item in line.1 {
                execute!(stdout(), SetForegroundColor(Color::Yellow))?;
                if *item == 1 {
                    execute!(stdout(), Print("O"))?;
                } else if *item == 2 {
                    execute!(stdout(), SetForegroundColor(Color::Grey))?;
                    execute!(stdout(), Print("O"))?;
                } else {
                    execute!(stdout(), Print(" "))?;
                }
            }
            execute!(
                stdout(),
                SetForegroundColor(Color::Yellow),
                cursor::MoveTo(state.point_start + 20, (line.0 + 1) as u16 + state.ind_y),
                Print('|')
            )?;
        }

        execute!(
            stdout(),
            cursor::MoveTo(state.point_start - 1, 21 + state.ind_y),
            Print("——————————————————————"),
            cursor::MoveTo(state.point_start - 1, 21 + state.ind_y + 1),
            Print("                      ")
        )?;
        match self.current_cord {
            Some([x, _]) => {
                execute!(
                    stdout(),
                    cursor::MoveTo(state.point_start + x as u16, 21 + state.ind_y + 1),
                    Print("^")
                )?;
            }
            _ => (),
        }

        
        terminal::enable_raw_mode().unwrap();
        
        Ok(())
    }

    pub fn run_offline(
        gd: &Arc<Mutex<Grid>>,
        state: &mut State,
        rand: u8,
    ) -> crossterm::Result<()> {
        execute!(
            stdout(),
            SetForegroundColor(Color::Green),
            cursor::MoveTo(state.size_terminal.0 / 2, 2)
        )?;
        state.set_start(state.size_terminal.0);

        if state.coin == 0 {
            execute!(stdout(), Print("0           "))?;
        } else {
            execute!(stdout(), Print(state.coin))?;
        }
        state.point_start = state.size_terminal.0 / 2 - 10;

        execute!(
            stdout(),
            SetForegroundColor(Color::Yellow),
            cursor::MoveTo(state.point_start, state.ind_y),
            Print("____________________")
        )?;

        for line in (gd.lock().unwrap()).grid.iter().enumerate() {
            execute!(
                stdout(),
                cursor::MoveTo(state.point_start - 1, (line.0 + 1) as u16 + state.ind_y),
                Print('|')
            )?;
            for item in line.1 {
                execute!(stdout(), SetForegroundColor(Color::Yellow))?;
                if *item == 1 {
                    execute!(stdout(), Print("O"))?;
                } else if *item == 2 {
                    execute!(stdout(), SetForegroundColor(Color::Grey))?;
                    execute!(stdout(), Print("O"))?;
                } else {
                    execute!(stdout(), Print(" "))?;
                }
            }
            execute!(
                stdout(),
                SetForegroundColor(Color::Yellow),
                cursor::MoveTo(state.point_start + 20, (line.0 + 1) as u16 + state.ind_y),
                Print('|')
            )?;
        }

        execute!(
            stdout(),
            cursor::MoveTo(state.point_start - 1, 21 + state.ind_y),
            Print("——————————————————————"),
            cursor::MoveTo(state.point_start - 1, 21 + state.ind_y + 1),
            Print("                      ")
        )?;
        match (gd.lock().unwrap()).current_cord {
            Some([x, _]) => {
                execute!(
                    stdout(),
                    cursor::MoveTo(state.point_start + x as u16, 21 + state.ind_y + 1),
                    Print("^")
                )?;
            }
            _ => (),
        }

        let mut lock_gd = gd.lock().unwrap();
        if lock_gd.current_cord.is_none() || (state.info.lock().unwrap()).is_none() {
            lock_gd.add_figure(get_random_figure(rand % 7));
            lock_gd.ready_clean(&mut state.coin);
        } else {
            lock_gd.move_to_side(&mut state.where_go);
        }
        terminal::enable_raw_mode().unwrap();
        if poll(Duration::from_millis(1)).unwrap() {
            event_handler_poll(
                &mut state.where_go,
                &mut lock_gd,
                &mut state.coin,
                &mut state.exi,
            );
        }
        Ok(())
    }

    pub fn run_online(gd: &Arc<Mutex<Grid>>, state: &mut State, rand: u8, other_gd: &mut GridFriend) {

    }
}

fn draw_points(grid: &mut [[u8; 20]; 20], points: &[[u8; 4]; 4], cord: [u8; 2], value: u8) {
    for line in points.iter().enumerate() {
        for item in line.1.iter().enumerate() {
            if *item.1 == 1 && cord[1] > 0 {
                grid[cord[1] as usize + line.0][cord[0] as usize + item.0] = value;
            }
        }
    }
}

fn is_side(grid: [[u8; 20]; 20], figure: &MatrixPoint4X, c: [u8; 2], side: i8) -> bool {
    for line in figure.arr.iter().enumerate() {
        for item in line.1.iter().enumerate() {
            if *item.1 == 1 {
                if (c[0] + item.0 as u8) <= 19 {
                    if !(c[0] + item.0 as u8 == 19 && side == 1) {
                        if grid[c[1] as usize + line.0][item.0 + (c[0] as i8 + side) as usize] == 1
                        {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
    }
    true
}

pub struct State {
    pub size_terminal: (u16, u16),
    point_start: u16,
    ind_y: u16,
    coin: usize,
    where_go: Side,
    exi: bool,
    info: Arc<Mutex<Option<()>>>,
    mixer: i16,
}
impl State {
    pub(crate) fn new(size_terminal: (u16, u16), info: Arc<Mutex<Option<()>>>, mixer: i16) -> State {
        State {
            size_terminal,
            point_start: size_terminal.0 / 2 - 10,
            ind_y: 4,
            coin: 0,
            where_go: Side::Stop,
            exi: false,
            info,
            mixer,
        }
    }

    pub(crate) fn is_exi(&self) -> bool {
        self.exi
    }

    pub fn set_start(&mut self, size: u16) {
        self.point_start = size / 2 - 10;
    }

    

    pub fn set_mixer(&mut self, mixer: i16) {
        self.mixer = mixer;
    }
}




pub struct GridFriend{
    grid: [[u8; 20]; 20],
    coin: usize,
}

impl GridFriend {
    pub fn new() -> GridFriend {
        GridFriend {
            grid: GRID,
            coin: 0
        }
    }    
}


