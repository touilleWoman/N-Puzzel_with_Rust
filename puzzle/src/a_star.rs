use super::types::Matrix;
use std::collections::VecDeque;
use std::rc::Rc;

///return next possible steps of a given puzzel
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
            // println!("c.data{:?}", c.data);

            let val: i32 = c.data[(b * c.row + a) as usize];
            // println!("val{}p{:?}a{}b{}row{}", val, p, a, b, c.row);
            c.data[(p.1 * c.row + p.0) as usize] = val;
            c.data[(b * c.row + a) as usize] = 0;
            // println!("c.data after{:?}", c.data);

            Rc::new(c)
        })
        .collect()
}

/// main algo
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
        let current: Rc<Matrix> = open
            .iter()
            .min_by_key(|x| x.h_cost + x.g_cost)
            .unwrap()
            .to_owned();
        // remove current from open list, add it to closed list
        open.remove(open.iter().position(|r| *r == current).unwrap());
        closed.push_back(current.clone());
        println!("current{:?}", current.data);

        if current.data == goal.data {
            println!("Success");
            return;
        }
        g_cost += 1;
        for mut neighbour in neighbours(current.clone()) {
            println!("neighbour{:?} ", neighbour.data);

            if closed.iter().find(|r| (***r).data == (*neighbour).data) != None {
                // if neighbour in closed list, skip to next neighbour
                println!("neighbour{:?} in closed", neighbour.data);
                continue;
            }
            let in_open = open.iter().find(|r| **r == neighbour);
            let mut mut_nei = Rc::get_mut(&mut neighbour).unwrap();



            mut_nei.update_h_cost(&goal);
            if mut_nei.h_cost + g_cost < current.h_cost + current.g_cost || in_open == None {
                mut_nei.g_cost = g_cost;
                mut_nei.parent = Some(current.clone());
                if in_open == None {
                    let nei = mut_nei.clone();
                    open.push_back(Rc::new(nei));
                }
            }
        }

    }
}
