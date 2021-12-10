//! struct Matrix and methods
use std::collections::BTreeMap;
use std::rc::{Rc, Weak};
use super::tools;

#[derive(Clone)]
pub struct Matrix {
    pub data: Vec<i32>,
    pub parent: Option<Weak<Matrix>>,
    pub h_cost: i32,
    pub g_cost: i32,
}

pub struct Open {
    pub btree: BTreeMap<i32, Vec<Rc<Matrix>>>,
}

impl Open {
    pub fn new() -> Self {
        Self {
            btree: BTreeMap::new(),
        }
    }
    pub fn insert(&mut self, fcost: i32, matrix: Rc<Matrix>) {
        match self.btree.get_mut(&fcost) {
            None => {
                self.btree.insert(fcost, vec![matrix]);
            }
            Some(x) => (*x).push(matrix),
        }
    }
    ///The first value in BtreeMap is a vector which contains one or more matrix with the minimum fcost.
    /// Pop out a matrix from vec. If vec is empty after pop, delete vec.
    pub fn pop_first(&mut self) -> Rc<Matrix> {
        let (&first_k, _matrix_vec) = self.btree.iter().next().unwrap();
        let matrix_vec = self.btree.get_mut(&first_k).unwrap();
        let matrix = (*matrix_vec).pop().unwrap();
        if (*matrix_vec).is_empty() {
            self.btree.remove(&first_k);
        }
        matrix
    }
}

impl Matrix {
    pub fn new(row: i32, data: Vec<i32>) -> Result<Matrix, &'static str> {
        if row < 3 {
            return Err("Puzzel size wrong");
        }
        if data.len() as i32 != row * row {
            return Err("Puzzel content nb wrong");
        };
        let ordered: Vec<i32> = (0..row * row).collect();
        let mut sort_data = data.clone();
        sort_data.sort();
        if sort_data != ordered {
            return Err("Puzzel content wrong");
        }
        let m = Self {
            data: data,
            parent: None,
            h_cost: 0,
            g_cost: 0,
        };
        return Ok(m);
    }

    pub fn update_cost(&mut self, goal: &Vec<i32>, algo: &Algo, heu: &Heuristic, row: i32) {
        match *algo {
            Algo::Astar => {
                self.h_cost = calculate_hcost(&self.data, goal, heu, row);
                self.g_cost += 1;
            }
            Algo::Greedy => {
                self.h_cost = calculate_hcost(&self.data, goal, heu, row);
            }
            Algo::Uniform => {
                self.g_cost += 1;
            }
        }
    }
}

fn calculate_hcost(board: &Vec<i32>, goal: &Vec<i32>, heu: &Heuristic, row: i32) -> i32 {
    let mut total = 0;
    for value in board.iter() {
        if *value == 0 {
            continue;
        }
        let po_goal = tools::position(goal, *value, row);
        let po_current = tools::position(&board, *value, row);
        total += match heu {
            Heuristic::Manhattan => manhattan(po_current, po_goal),
            Heuristic::TilesOut => tiles_out_of_place(po_current, po_goal),
            Heuristic::Euclidean => euclidean(po_current, po_goal),
        };
    }
    total
}

fn tiles_out_of_place(p: (i32, i32), goal: (i32, i32)) -> i32 {
    match p == goal {
        true => 0,
        false => 1,
    }
}

fn euclidean(p: (i32, i32), goal: (i32, i32)) -> i32 {
    let x = ((goal.0 - p.0).pow(2) + (goal.1 - p.1).pow(2)) as f64;
    x.sqrt() as i32
}

fn manhattan(p: (i32, i32), goal: (i32, i32)) -> i32 {
    (goal.0 - p.0).abs() + (goal.1 - p.1).abs()
}

#[derive(Debug)]
pub enum Heuristic {
    Manhattan,
    Euclidean,
    TilesOut,
}

impl Heuristic {
    pub fn from_str(s: &str) -> Result<Heuristic, &'static str> {
        match s {
            "Manhattan" | "manhattan" | "man" => Ok(Heuristic::Manhattan),
            "Euclidean" | "euclidean" | "euc" => Ok(Heuristic::Euclidean),
            "TilesOut" | "tiles out" | "til" => Ok(Heuristic::TilesOut),
            _ => Err("Wrong heuristic input, choose from Manhattan, Euclidean or TileOut"),
        }
    }
}

pub enum Algo {
    Astar,
    Greedy,
    Uniform,
}

impl Algo {
    pub fn from_str(s: &str) -> Result<Algo, &'static str> {
        match s {
            "Astar" | "a_star" | "astar" => Ok(Algo::Astar),
            "Greedy" | "greedy" => Ok(Algo::Greedy),
            "Uniform" | "uniform" => Ok(Algo::Uniform),
            _ => Err("Wrong algo input, choose from Astar, Greedy or Uniform"),
        }
    }
}
