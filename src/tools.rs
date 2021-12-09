use super::types::Matrix;
use std::rc::Rc;

///return next possible steps of a given puzzel
pub fn neighbours(current: Rc<Matrix>, row: i32) -> Vec<Matrix> {
    let p = current.position(0, row);

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