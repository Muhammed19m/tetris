#[derive(Copy, Clone)]
pub struct MatrixPoint4X {
    pub arr: [[u8; 4]; 4],
}

impl MatrixPoint4X {
    pub fn new(items: [[u8; 4]; 4]) -> Self {
        Self { arr: items }
    }

    pub fn upend(&mut self) {
        let mut copy = self.arr;
        let mut metka = 3;
        for i in copy.iter().enumerate() {
            for item in i.1.into_iter().enumerate() {
                self.arr[item.0][metka] = *item.1;
            }
            if metka > 0 {
                metka -= 1
            }
        }
        let mut steep = 0;
        'l: for i in self.arr.iter().enumerate() {
            for item in i.1 {
                if *item == 1 {
                    steep = i.0;
                    break 'l;
                }
            }
        }
        copy = self.arr;
        if steep > 0 {
            for line in copy.iter().enumerate() {
                for item in line.1.iter().enumerate() {
                    if *item.1 == 1 {
                        self.arr[line.0 - steep][item.0] = 1;
                        self.arr[line.0][item.0] = 0;
                    }
                }
            }
        }
        steep = 3;
        copy = self.arr;
        for line in copy.iter() {
            for item in line.iter().enumerate() {
                if *item.1 == 1 {
                    if steep > item.0 {
                        steep = item.0;
                    }
                }
            }
        }
        if steep > 0 {
            for line in copy.iter().enumerate() {
                for item in line.1.iter().enumerate() {
                    if *item.1 == 1 {
                        self.arr[line.0][item.0 - steep] = 1;
                        self.arr[line.0][item.0] = 0;
                    }
                }
            }
        }
    }
}
