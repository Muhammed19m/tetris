use crossterm::terminal;



#[derive(Clone, Copy)]
pub struct SizeTerminal(pub u16, pub u16);

impl SizeTerminal {
    pub fn new() -> SizeTerminal {
        let (x, y) = terminal::size().unwrap();
        Self(x, y)        
    }

    pub fn update(&mut self) -> bool {
        let s = terminal::size().unwrap();
        if self == &s {
            false 
        } else {
            self.0 = s.0;
            self.1 = s.1;
            true
        }
    } 

}

impl PartialEq<(u16, u16)> for SizeTerminal {
    fn eq(&self, other: &(u16, u16)) -> bool {
        self.0 == other.0 && self.1 == other.1
    } 
}