use super::types::Matrix;
use std::collections::VecDeque;

pub fn a_star(mut origin: Matrix) {
    let goal: Matrix = Matrix::new(origin.row, origin.make_goal()).unwrap();
    let mut open: VecDeque<Matrix> = VecDeque::new();
    let mut closed: VecDeque<Matrix> = VecDeque::new();
    let success: bool = false;
    origin.update_h_cost(goal);
    open.push_back(origin);
    while !open.is_empty() && !success {
        println!("loop");
        
    }
}
