//! A* algo for searching solution of N-puzzel
use super::types::Matrix;
use super::Heuristic;
use std::rc::Rc;

///return next possible steps of a given puzzel
fn neighbours(current: Rc<Matrix>) -> Vec<Rc<Matrix>> {
    let p = current.position(0);

    static NEIGHBOUR: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    NEIGHBOUR
        .iter()
        .map(|(a, b)| (p.0 + a, p.1 + b))
        .filter(|(x, y)| *x >= 0 && *x < current.row && *y >= 0 && *y < current.row)
        .map(|(a, b)| {
            let mut c = (*current).clone();
            let val: i32 = c.data[(b * c.row + a) as usize];
            c.data[(p.1 * c.row + p.0) as usize] = val;
            c.data[(b * c.row + a) as usize] = 0;
            Rc::new(c)
        })
        .collect()
}

/// A* algo with 3 optional heuristics : manhanttan distance, euclidean distance or nb of tiles out of places
pub fn a_star(mut origin: Matrix, heu: Heuristic) {
    let goal: Matrix = Matrix::new(origin.row, origin.make_goal()).unwrap();
    let mut open: Vec<Rc<Matrix>> = Vec::new();
    let mut closed: Vec<Rc<Matrix>> = Vec::new();
    let mut g_cost = 0;
    let success: bool = false;
    let mut max_nb: usize = 0; // in open list or open+ closed ?????

    // add origin matrix in open
    origin.update_h_cost(&goal, &heu);
    open.push(Rc::new(origin));
    let mut open_counter = 1;

    while !open.is_empty() && !success {
        // println!("loop start");

        // select the Matrix with the lowest f_cost in open list
        let current: Rc<Matrix> = open
            .iter()
            .min_by_key(|x| x.h_cost + x.g_cost)
            .unwrap()
            .to_owned();

        // remove current from open list, add it to closed list
        open.remove(open.iter().position(|r| *r == current).unwrap());
        closed.push(current.clone());
        // println!("current{:?}", current.data);

        if current.data == goal.data {
            return solution_found(open_counter, g_cost, max_nb, current);
        }

        g_cost += 1;
        for mut neighbour in neighbours(current.clone()) {
            // println!("neighbour{:?} ", neighbour.data);

            if closed.iter().find(|r| (***r).data == (*neighbour).data) != None {
                // if neighbour matrix is in closed list, skip to next
                continue;
            }

            let in_open = open.iter().find(|r| **r == neighbour);

            let mut neighbour = Rc::get_mut(&mut neighbour).unwrap();
            neighbour.update_h_cost(&goal, &heu);

            // if neighbour matrix has lower f_cost(f = h + g) OR neighbour in open list
            if neighbour.h_cost + g_cost < current.h_cost + current.g_cost || in_open == None {
                neighbour.update_g_cost(g_cost);
                neighbour.parent = Some(current.clone()); // set parent of neighbour is current
                if in_open == None {
                    //if neighbour not in open, then add to open list
                    let nei = neighbour.clone();
                    open.push(Rc::new(nei));
                    open_counter += 1;
                }
            }
        }

        max_nb = match max_nb < open.len() {
            true => open.len(),
            false => max_nb,
        }
    }
}

/** Puzzle resolved, print infomation:\n
complexity in time =>   Total number of states ever selected in the "opened" set
complexity in size =>   Maximum number of states ever represented in memory at the same time during the search (complexity in size)
Nb of moves =>          Number of moves required to transition from the initial state to the final state, according to the search

*/
fn solution_found(open_counter: i32, g_cost: i32, max_nb: usize, cur: Rc<Matrix>) {
    println!("Solution found !");
    println!("complexity in time: {}", open_counter);
    println!("complexity in size: {}", max_nb);
    println!("Nb of moves: {}", g_cost);
    println!("\nOrdered sequence of states =>");
    recursive_print_parent(cur);
}

fn recursive_print_parent(cur: Rc<Matrix>) {
    match cur.parent.as_ref() {
        None => {}
        Some(next) => {
            recursive_print_parent((*next).clone());
        }
    };
    let mut v: Vec<i32> = cur.data.clone();

    for _ in 0..cur.row {
        println!("{:?}", v.drain(0..cur.row as usize).as_slice());
    }
    println!();
}
