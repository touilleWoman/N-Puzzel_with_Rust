//! A* algo for searching solution of N-puzzel
use std::collections::HashSet;
use std::rc::Rc;

use super::parser::make_goal;
use super::tools;
use super::types::{Matrix, Open};
use super::Heuristic;

/// A* search algo with 3 optional heuristics : manhanttan distance, euclidean distance or nb of tiles out of places
pub fn a_star(mut origin: Matrix, heu: Heuristic, row: i32) -> Option<Vec<i32>> {
    let goal = make_goal(row);
    let mut open: Open = Open::new();
    let mut closed: HashSet<Vec<i32>> = HashSet::new();

    // To avoid stack overflow caused by recursive free of parent,
    // I use weak ref for parent, so I need strong_ref to store strong ref
    let mut strong_ref: Vec<Rc<Matrix>> = Vec::new();

    // add origin matrix in open
    origin.update_h_cost(&goal, &heu, row);
    let fcost = origin.h_cost + origin.g_cost;
    let rc = Rc::new(origin);
    open.insert(fcost, rc.clone());
    strong_ref.push(rc);

    let mut open_total: u32 = 1;
    let mut open_now: u32 = 1;

    while !open.btree.is_empty() {
        // pop out the matrix having lowest f_cost in open, add to closed
        let current = open.pop_first();
        open_now -= 1;
        closed.insert(current.data.clone());

        if current.data == goal {
            return Some(solution_found(open_total, open_now, current.as_ref(), row));
        }

        //get next possible states of current
        for mut neighbour in tools::neighbours(current.clone(), row) {
            if closed.contains(&neighbour.data) {
                continue;
            }
            
            neighbour.update_h_cost(&goal, &heu, row);
            neighbour.g_cost += 1;
            let fcost = neighbour.h_cost + neighbour.g_cost;
            neighbour.parent = Some(Rc::downgrade(&current)); // set parent of neighbour is current
            let nei = Rc::new(neighbour);
            open.insert(fcost, Rc::clone(&nei));
            open_total += 1;
            open_now += 1;
            strong_ref.push(nei);
        }
    }
    return None;
}

/** Puzzle resolved, print infomation:\n
complexity in time =>   Total number of states ever selected in the "opened" set
complexity in size =>   Maximum number of states ever represented in memory at the same time during the search (complexity in size)
Nb of moves =>          Number of moves required to transition from the initial state to the final state, according to the search

*/
fn solution_found(open_total: u32, open_now: u32, cur: &Matrix, row: i32) -> Vec<i32> {
    println!("Solution found !");
    println!("complexity in time: {}", open_total);
    println!("complexity in size: {}", open_now);
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
    let board: Vec<i32> = cur.data.clone();
    tools::nice_print(board, row);
}
