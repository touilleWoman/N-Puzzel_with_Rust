use super::types::Matrix;
use std::collections::VecDeque;

fn copy(origin: &Matrix) -> Matrix {
    Matrix {
        row: origin.row,
        data: origin.data.clone(),
        parent: vec![],
        h_cost: origin.h_cost,
        g_cost: origin.g_cost,
    }
}

// fn neighbours(current: &Matrix) {
fn neighbours<'a>(current : &'static Matrix) -> impl Iterator<Item = Matrix> {
    let p = current.position(0);
    println!("zero position:{:?}", p);

    static NEIGHBOUR: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let neib = NEIGHBOUR
        .iter()
        .map(|(a, b)| (p.0 + a, p.1 + b))
        .filter(|(x, y)| *x >= 0 && *x < current.row && *y >= 0 && *y < current.row);

    neib.map(|(a, b)| {
        let mut c = copy(current);
        let val: i32 = c.data[(a * c.row + b) as usize];
        c.data[(p.0 * c.row + p.0) as usize] = val;
        c.data[(a * c.row + b) as usize] = 0;
        return c;
    })
    // for x in bla {
    //     println!("bla:{:?}", x.data);
    // }
}

pub fn a_star(mut origin: Matrix) {
    let goal: Matrix = Matrix::new(origin.row, origin.make_goal()).unwrap();
    let mut open: VecDeque<&Matrix> = VecDeque::new();
    let mut closed: VecDeque<&Matrix> = VecDeque::new();
    let mut g_cost = 0;
    let success: bool = false;
    origin.update_h_cost(&goal);
    open.push_back(&origin);
    while !open.is_empty() && !success {
        println!("loop start");
        // current is the Matrix which has the lowest f value
        let current: &Matrix = open.iter().min_by_key(|x| x.h_cost + x.g_cost).unwrap();
        // remove current from open list, add it to closed list
        open.remove(open.iter().position(|&r| r == current).unwrap());
        closed.push_back(current);

        if *current.data == goal.data {
            println!("Success");
            return;
        }
        neighbours(current);

        break;
    }
}
