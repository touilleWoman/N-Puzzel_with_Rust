struct Matrix {
    row: i32,
    data: Vec<i32>,
}

impl Matrix {
    pub fn selfcheck(self) {
        if self.row < 3 {
            panic!("Puzzel size wrong")
        }
        if self.data.len() as i32 != self.row * self.row {
            panic!("Puzzel content nb wrong");
        };
        let ordered: Vec<i32> = (0..self.row * self.row).map(|x| x).collect();
        let mut sort_data = self.data.clone();
        sort_data.sort();
        if sort_data != ordered {
            panic!("Puzzel content wrong");
        }
    }
}

use std::fs::File;
use std::io::{BufRead, BufReader};
fn parse() -> Matrix {
    let filename = "../test1";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut row: i32 = 0;
    let mut data = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.chars().next() == Some('#') || line.chars().next().is_none() {
            continue;
        } else if row == 0 {
            row = line.parse::<i32>().unwrap();
        } else {
            data.push(line.parse::<i32>().unwrap())
        }
    }
    Matrix {
        row: row,
        data: data,
    }
}

fn main() {
    let m = parse();
    println!("{}", m.row);
    println!("{:?}", m.data);
    m.selfcheck();
}
