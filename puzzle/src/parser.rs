use super::types::Matrix;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse() -> Matrix {
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
