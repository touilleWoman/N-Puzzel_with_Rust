use super::types::Matrix;
use std::collections::VecDeque;
use std::rc::Rc;

///return next possible steps of given puzzel
fn neighbours(current: Rc<Matrix>) -> Vec<Rc<Matrix>> {
    let p = current.position(0);
    // println!("zero position:{:?}", p);

    static NEIGHBOUR: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    NEIGHBOUR
        .iter()
        .map(|(a, b)| (p.0 + a, p.1 + b))
        .filter(|(x, y)| *x >= 0 && *x < current.row && *y >= 0 && *y < current.row)
        .map(|(a, b)| {
            let mut c = (*current).clone();
            let val: i32 = c.data[(a * c.row + b) as usize];
            c.data[(p.0 * c.row + p.0) as usize] = val;
            c.data[(a * c.row + b) as usize] = 0;
            Rc::new(c)
        })
        .collect()
}

pub fn a_star(mut origin: Matrix) {
    let goal: Matrix = Matrix::new(origin.row, origin.make_goal()).unwrap();
    let mut open: VecDeque<Rc<Matrix>> = VecDeque::new();
    let mut closed: VecDeque<Rc<Matrix>> = VecDeque::new();
    let mut g_cost = 0;
    let success: bool = false;
    origin.update_h_cost(&goal);
    open.push_back(Rc::new(origin));
    while !open.is_empty() && !success {
        println!("loop start");
        // current is the Matrix which has the lowest f value
        let current: Rc<Matrix> = open.iter().min_by_key(|x| x.h_cost + x.g_cost).unwrap().to_owned();
        // remove current from open list, add it to closed list
        open.remove(open.iter().position(|r| *r == current).unwrap());
        closed.push_back(current.clone());

        if current.data == goal.data {
            println!("Success");
            return;
        }
        for x in neighbours(current){
            println!("neighbour:{:?}", x.data);
        }

        break;
    }
}
