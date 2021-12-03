use super::types::Matrix;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn make_goal(row: i32) -> Vec<i32> {
    let mut g = vec![-1; (row * row) as usize];
    let mut cur: i32 = 1;
    let mut x: i32 = 0;
    let mut ix: i32 = 1;
    let mut y: i32 = 0;
    let mut iy: i32 = 0;
    loop {
        g[(x + y * row) as usize] = cur;
        if cur == 0 {
            // println!("goal=>{:?}", g);
            break g;
        }
        cur += 1;
        if x + ix == row || x + ix < 0 || (ix != 0 && g[(x + ix + y * row) as usize] != -1) {
            iy = ix;
            ix = 0;
        } else if y + iy == row || y + iy < 0 || (iy != 0 && g[(x + (y + iy) * row) as usize] != -1)
        {
            ix = -iy;
            iy = 0;
        }
        x += ix;
        y += iy;
        if cur == row * row {
            cur = 0
        }
    }
}

pub fn parse(filepath: &str) -> Matrix {
    let mut err_msg = String::from("Wrong file path => ");
    err_msg.push_str(filepath);
    let file = File::open(filepath).expect(&err_msg);
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
