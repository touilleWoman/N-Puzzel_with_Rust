use super::types::Matrix;
use std::rc::Rc;

///return next possible steps of a given puzzel
pub fn neighbours(current: Rc<Matrix>, row: i32) -> Vec<Matrix> {
    let p = position(&current.data, 0, row);

    static NEIGHBOUR: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    NEIGHBOUR
        .iter()
        .map(|(a, b)| (p.0 + a, p.1 + b))
        .filter(|(x, y)| *x >= 0 && *x < row && *y >= 0 && *y < row)
        .map(|(a, b)| {
            let mut c = (*current).clone();
            let val: i32 = c.data[(b * row + a) as usize];
            c.data[(p.1 * row + p.0) as usize] = val;
            c.data[(b * row + a) as usize] = 0;
            c
        })
        .collect()
}

/// print puzzle board
pub fn nice_print(mut board: Vec<i32>, row: i32) {
    for _ in 0..row {
        println!("{:?}", board.drain(0..row as usize).as_slice());
    }
    println!();
}

/// find position of value in puzzle board
pub fn position(board: &Vec<i32>, value: i32, row: i32) -> (i32, i32) {
    let p: i32 = board.iter().position(|&x| x == value).unwrap() as i32;
    (p % row, p / row)
}
