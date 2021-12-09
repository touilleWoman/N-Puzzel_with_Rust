//! A* algo for searching solution of N-puzzel
use super::parser::make_goal;
use super::types::{Matrix, Open};
use super::Heuristic;
use std::collections::HashSet;
use std::rc::Rc;

///return next possible steps of a given puzzel
fn neighbours(current: Rc<Matrix>, row: i32) -> Vec<Matrix> {
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

/// A* search algo with 3 optional heuristics : manhanttan distance, euclidean distance or nb of tiles out of places
pub fn a_star(mut origin: Matrix, heu: Heuristic, row: i32) -> Option<Vec<i32>> {
    let goal: Matrix = Matrix::new(row, make_goal(row)).unwrap();
    let mut open: Open = Open::new();
    let mut closed: HashSet<Vec<i32>> = HashSet::new();
    let mut strong_ref: Vec<Rc<Matrix>> = Vec::new();
    let mut max_open: usize = 0; // Maximum number of states ever represented in memory

    // add origin matrix in open
    origin.update_h_cost(&goal, &heu, row);
    let fcost = origin.h_cost + origin.g_cost;
    let rc = Rc::new(origin);
    open.insert(fcost, rc.clone());
    strong_ref.push(rc);
    let mut open_counter = 1;

    while !open.btree.is_empty() {
        // select the Matrix with the lowest f_cost in open list
        let current = open.pop_first();
        if current.data == goal.data {
            return Some(solution_found(open_counter, max_open, current.as_ref(), row));
        }
        closed.insert(current.data.clone());

        for mut neighbour in neighbours(current.clone(), row) {
            if closed.contains(&neighbour.data) {
                continue;
            }

            let in_open = open.hashmap.contains_key(&neighbour.data);

            neighbour.update_h_cost(&goal, &heu,row);

            // if neighbour matrix has lower f_cost(f = h + g) OR neighbour in open list
            if neighbour.h_cost + neighbour.g_cost < current.h_cost + current.g_cost || !in_open {
                neighbour.g_cost += 1;
                neighbour.parent = Some(Rc::downgrade(&current)); // set parent of neighbour is current
                let fcost = neighbour.h_cost + neighbour.g_cost;
                let nei = Rc::new(neighbour);
                open.insert(fcost, Rc::clone(&nei));
                open_counter += 1;
                strong_ref.push(nei);
            }
        }

        max_open = match max_open < open.hashmap.len() {
            true => open.hashmap.len(),
            false => max_open,
        }
    }
    return None;
}

/** Puzzle resolved, print infomation:\n
complexity in time =>   Total number of states ever selected in the "opened" set
complexity in size =>   Maximum number of states ever represented in memory at the same time during the search (complexity in size)
Nb of moves =>          Number of moves required to transition from the initial state to the final state, according to the search

*/
fn solution_found(open_counter: i32, max_nb: usize, cur: &Matrix, row:i32) -> Vec<i32> {
    println!("Solution found !");
    println!("complexity in time: {}", open_counter);
    println!("complexity in size: {}", max_nb);
    println!("Nb of moves: {}", cur.g_cost);
    println!("\nOrdered sequence of states =>");
    recursive_print_parent(cur, row);
    return cur.data.clone();
}

fn recursive_print_parent(cur: &Matrix, row: i32) {
    match cur.parent.as_ref() {
        None => {}
        Some(next) => {
            recursive_print_parent(&next.upgrade().unwrap(), row);
        }
    };
    let mut v: Vec<i32> = cur.data.clone();

    for _ in 0..row {
        println!("{:?}", v.drain(0..row as usize).as_slice());
    }
    println!();
}
