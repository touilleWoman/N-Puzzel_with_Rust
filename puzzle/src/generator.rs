use super::types::Matrix;
use super::parser::make_goal;
use rand::seq::SliceRandom;



fn swap_empty(v: &mut Vec<i32>, row: i32){
    let p: i32 = v.iter().position(|&x| x == 0).unwrap() as i32;
    let p: (i32, i32) = (p % row, p / row);

    static NEIGHBOUR: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let neighbours : Vec<(i32, i32)> = NEIGHBOUR
        .iter()
        .map(|(a, b)| (p.0 + a, p.1 + b))
        .filter(|(x, y)| *x >= 0 && *x < row && *y >= 0 && *y < row)
        .collect();

    let mut rng = rand::thread_rng();
    let random = neighbours.choose(&mut rng).unwrap();
    let val: i32 = v[(random.1 * row + random.0) as usize];
    v[(p.1 * row + p.0) as usize] = val;
    v[(random.1 * row + random.0) as usize] = 0;
}

pub fn generator(row: i32, iteration: i32) -> Matrix {
    let mut v = make_goal(row);
    for _ in 0..iteration {
        swap_empty(&mut v, row);
    }
    Matrix::new(row, v).unwrap()
}