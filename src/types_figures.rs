pub enum Figures {
    J,
    I,
    O,
    L,
    Z,
    T,
    S,
}

pub fn get_figure_matrix(typ: Figures) -> [[u8; 4]; 4] {
    match typ {
        Figures::J => [[0, 1, 0, 0], [0, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
        Figures::I => [[1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0], [1, 0, 0, 0]],
        Figures::O => [[1, 1, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        Figures::L => [[1, 0, 0, 0], [1, 0, 0, 0], [1, 1, 0, 0], [0, 0, 0, 0]],
        Figures::Z => [[1, 1, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        Figures::T => [[1, 1, 1, 0], [0, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
        Figures::S => [[0, 1, 1, 0], [1, 1, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
    }
}

pub fn get_random_figure(number: u8) -> Figures {
    match number {
        0 => Figures::I,
        1 => Figures::J,
        2 => Figures::L,
        3 => Figures::O,
        4 => Figures::S,
        5 => Figures::T,
        6 => Figures::Z,
        _ => Figures::O,
    }
}
