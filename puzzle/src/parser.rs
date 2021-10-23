use super::types::Matrix;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse(filepath: &'static str) -> Matrix {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut row: i32 = 0;
    let mut data = Vec::new();
    for line in reader.lines() {
        let mut line = line.unwrap();

        // delete comment after #
        let sharp_offset = line.find('#').unwrap_or(line.len());
        let line: String = line.drain(..sharp_offset).collect();

        let line = line.trim();
        if line.is_empty() {
            continue;
        } else if row == 0 {
            row = line.parse::<i32>().unwrap();
        } else {
            let iter = line.split_ascii_whitespace();
            for x in iter {
                data.push(x.parse::<i32>().unwrap())
            }
        }
    }
    Matrix::new(row, data).unwrap()
}
