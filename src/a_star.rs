//! A* algo for searching solution of N-puzzel
use super::parser::make_goal;
use super::types::Matrix;
use super::Heuristic;
use std::rc::Rc;
use std::collections::HashSet;
use std::collections::BTreeMap;
use std::rc::Weak;

///return next possible steps of a given puzzel
fn neighbours(current: Rc<Matrix>) -> Vec<Matrix> {
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
            c
        })
        .collect()
}

/// A* algo with 3 optional heuristics : manhanttan distance, euclidean distance or nb of tiles out of places
pub fn a_star(mut origin: Matrix, heu: Heuristic) -> Option<Vec<i32>> {
    let goal: Matrix = Matrix::new(origin.row, make_goal(origin.row)).unwrap();
    let mut open: BTreeMap<i32, Rc<Matrix>> = BTreeMap::new();
    let mut closed: HashSet<Vec<i32>> = HashSet::new();
    let mut matrices: Vec<Rc<Matrix>> = Vec::new();
    let success: bool = false;
    let mut max_nb: usize = 0; // Maximum number of states ever represented in memory

    // add origin matrix in open
    origin.update_h_cost(&goal, &heu);
    let key = origin.h_cost + origin.g_cost;
    let rc = Rc::new(origin);
    open.insert(key, rc.clone());
    matrices.push(rc);
    let mut open_counter = 1;

    while !open.is_empty() && !success {
        
        // select the Matrix with the lowest f_cost in open list
        // Since BtreeMap is ordered according to key value, min is the first item
        let min_fcost: i32 = *(open.keys().next().unwrap());
        let current = open.remove(&min_fcost).unwrap();
        closed.insert(current.data.clone());

        if current.data == goal.data {
            return Some(solution_found(open_counter, max_nb, current.as_ref()));
        }

        for mut neighbour in neighbours(current.clone()) {
            if closed.contains(&neighbour.data) {
                continue;
            }

            let in_open = open.iter().find(|(_key, value)| *value.as_ref().data == neighbour.data);

            neighbour.update_h_cost(&goal, &heu);

            // if neighbour matrix has lower f_cost(f = h + g) OR neighbour in open list
            if neighbour.h_cost + neighbour.g_cost < current.h_cost + current.g_cost
                || in_open.is_none()
            {
                neighbour.g_cost += 1;
                neighbour.parent = Some(Rc::downgrade(&current)); // set parent of neighbour is current
                if in_open.is_none() {
                    //if neighbour not in open, then add to open list
                    // open.push(Rc::new(nei));
                    let key = neighbour.h_cost + neighbour.g_cost;
                    let rc = Rc::new(neighbour);
                    open.insert(key, rc.clone());
                    matrices.push(rc);
                    open_counter += 1;
                }
            }
        }

        max_nb = match max_nb < open.len() {
            true => open.len(),
            false => max_nb,
        }
    }
    return None;
}

/** Puzzle resolved, print infomation:\n
complexity in time =>   Total number of states ever selected in the "opened" set
complexity in size =>   Maximum number of states ever represented in memory at the same time during the search (complexity in size)
Nb of moves =>          Number of moves required to transition from the initial state to the final state, according to the search

*/
fn solution_found(open_counter: i32, max_nb: usize, cur: &Matrix) -> Vec<i32> {
    println!("Solution found !");
    println!("complexity in time: {}", open_counter);
    println!("complexity in size: {}", max_nb);
    println!("Nb of moves: {}", cur.g_cost);
    println!("\nOrdered sequence of states =>");
    recursive_print_parent(cur);
    return cur.data.clone();
}

fn recursive_print_parent(cur: &Matrix) {
    match cur.parent.as_ref() {
        None => {}
        Some(next) => {
            recursive_print_parent(&next.upgrade().unwrap());
        }
    };
    let mut v: Vec<i32> = cur.data.clone();

    for _ in 0..cur.row {
        println!("{:?}", v.drain(0..cur.row as usize).as_slice());
    }
    println!();
}
