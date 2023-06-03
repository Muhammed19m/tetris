use std::sync::{atomic::AtomicBool, Arc, Mutex};

use crate::Side;

use super::size::SizeTerminal;

pub struct State {
    pub size_terminal: SizeTerminal,
    pub point_start: u16,
    pub ind_y: u16,
    pub coin: usize,
    pub where_go: Side,
    pub exi: bool,
    pub info: Arc<Mutex<Option<()>>>,
    pub mixer: i16,
    pub field_for_second_player: Arc<AtomicBool>, // self player every time true
                                                  // pub resize: Option<(u16, u16)>,
}
impl State {
    pub(crate) fn new(
        size_terminal: SizeTerminal,
        info: Arc<Mutex<Option<()>>>,
        mixer: i16,
        field_for_second_player: bool, /* self player every time true */
    ) -> State {
        State {
            size_terminal,
            point_start: size_terminal.0 / 2 - 10,
            ind_y: 4,
            coin: 0,
            where_go: Side::Stop,
            exi: false,
            info,
            mixer,
            field_for_second_player: Arc::new(AtomicBool::new(field_for_second_player)),
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
