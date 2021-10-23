pub struct Matrix {
    pub row: i32,
    pub data: Vec<i32>,
}

impl Matrix {
    pub fn new(row: i32, data: Vec<i32>) -> Result<Matrix, &'static str> {
        if row < 3 {
            return Err("Puzzel size wrong");
        }
        if data.len() as i32 != row * row {
            return Err("Puzzel content nb wrong");
        };
        let ordered: Vec<i32> = (0..row * row).collect();
        let mut sort_data = data.clone();
        sort_data.sort();
        if sort_data != ordered {
            return Err("Puzzel content wrong");
        }
        let m = Self { row, data };
        return Ok(m);
    }
}
