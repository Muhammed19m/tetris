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

use crate::{get_figure_matrix, sleep, thread, Arc, Duration, Figures, MatrixPoint4X, Mutex};

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

    pub fn move_down(&mut self) -> Option<()> {
        if let Some(ref mut c) = self.current_cord {
            draw_points(&mut self.grid, &self.figure.unwrap().arr, *c, 0);
            for line in self.figure.unwrap().arr.into_iter().enumerate() {
                for item in line.1.into_iter().enumerate() {
                    if item.1 == 1 {
                        if c[1] + (line.0 as u8) + 1 < 20 {
                            if self.grid[c[1] as usize + 1 + line.0][c[0] as usize + item.0] != 0 {
                                draw_points(&mut self.grid, &self.figure.unwrap().arr, *c, 1);
                                return None;
                            }
                        } else {
                            draw_points(&mut self.grid, &self.figure.unwrap().arr, *c, 1);
                            return None;
                        }
                    }
                }
            }

            c[1] += 1;

            draw_points(&mut self.grid, &self.figure.unwrap().arr, *c, 1);
            return Some(());
        }
        return None;
    }

    pub fn move_to_side(&mut self, l_r: &mut Side) {
        if let Some(ref mut c) = self.current_cord {
            match *l_r {
                Side::Left => {
                    draw_points(&mut self.grid, &self.figure.unwrap().arr, *c, 0);

                    if is_side(self.grid, &self.figure.unwrap(), *c, -1) {
                        c[0] -= 1;
                        draw_points(&mut self.grid, &self.figure.unwrap().arr, *c, 1);
                    } else {
                        draw_points(&mut self.grid, &self.figure.unwrap().arr, *c, 1);
                    }

                    *l_r = Side::Stop;
                }

                Side::Right => {
                    draw_points(&mut self.grid, &self.figure.unwrap().arr, *c, 0);

                    if is_side(self.grid, &self.figure.unwrap(), *c, 1) {
                        c[0] += 1;
                        draw_points(&mut self.grid, &self.figure.unwrap().arr, *c, 1);
                    } else {
                        draw_points(&mut self.grid, &self.figure.unwrap().arr, *c, 1);
                    }

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
                _ => (),
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
            *(info.lock().unwrap()) = (gd.lock().unwrap()).move_down();
        })
    }
}

fn draw_points(arr: &mut [[u8; 20]; 20], points: &[[u8; 4]; 4], cord: [u8; 2], value: u8) {
    for line in points.iter().enumerate() {
        for item in line.1.iter().enumerate() {
            if *item.1 == 1 && cord[1] > 0 {
                arr[cord[1] as usize + line.0][cord[0] as usize + item.0] = value;
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
                        if grid[c[1] as usize + line.0][item.0 + (c[0] as i8 + side) as usize] != 0
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
