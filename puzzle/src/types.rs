pub struct Matrix {
    pub row: i32,
    pub data: Vec<i32>,
}

impl Matrix {
    pub fn selfcheck(self) {
        if self.row < 3 {
            panic!("Puzzel size wrong")
        }
        if self.data.len() as i32 != self.row * self.row {
            panic!("Puzzel content nb wrong");
        };
        let ordered: Vec<i32> = (0..self.row * self.row).collect();
        let mut sort_data = self.data.clone();
        sort_data.sort();
        if sort_data != ordered {
            panic!("Puzzel content wrong");
        }
    }
}
