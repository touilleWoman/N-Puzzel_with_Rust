//! struct Matrix and methods
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub struct Matrix {
    pub row: i32,
    pub data: Vec<i32>,
    pub parent: Option<Rc<Matrix>>,
    pub h_cost: i32,
    pub g_cost: i32,
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
            row: row,
            data: data,
            parent: None,
            h_cost: 0,
            g_cost: 0,
        };
        return Ok(m);
    }
    pub fn position(&self, value: i32) -> (i32, i32) {
        let p: i32 = self.data.iter().position(|&x| x == value).unwrap() as i32;
        // println!("position of value({}) =>{}", value, p);
        (p % self.row, p / self.row)
    }

    pub fn make_goal(&self) -> Vec<i32> {
        let mut g = vec![-1; (self.row * self.row) as usize];
        let mut cur: i32 = 1;
        let mut x: i32 = 0;
        let mut ix: i32 = 1;
        let mut y: i32 = 0;
        let mut iy: i32 = 0;
        loop {
            g[(x + y * self.row) as usize] = cur;
            if cur == 0 {
                println!("goal:{:?}", g);
                break g;
            }
            cur += 1;
            if x + ix == self.row
                || x + ix < 0
                || (ix != 0 && g[(x + ix + y * self.row) as usize] != -1)
            {
                iy = ix;
                ix = 0;
            } else if y + iy == self.row
                || y + iy < 0
                || (iy != 0 && g[(x + (y + iy) * self.row) as usize] != -1)
            {
                ix = -iy;
                iy = 0;
            }
            x += ix;
            y += iy;
            if cur == self.row * self.row {
                cur = 0
            }
        }
    }
    pub fn update_h_cost(&mut self, goal: &Matrix, heu: &Heuristic) {
        let mut total = 0;
        for value in self.data.iter() {
            if *value == 0 {
                continue;
            }
            let po_goal = goal.position(*value);
            let po_current = self.position(*value);
            // total += (po_goal.0 - po_current.0).abs() + (po_goal.1 - po_current.1).abs();
            total += match heu {
                Heuristic::Manhattan =>manhattan(po_current, po_goal),
                Heuristic::TilesOut =>tiles_out_of_place(po_current, po_goal),
                Heuristic::Euclidean => euclidean(po_current, po_goal),
            }
             
        }
        self.h_cost = total;
        // println!("hcost updated:{}", self.h_cost);
    }
    pub fn update_g_cost(&mut self, g: i32) {
        self.g_cost = g;
    }
}

fn tiles_out_of_place(p:(i32, i32), goal:(i32, i32)) -> i32 {
    match p == goal {
        true => 0,
        false => 1
    }
}

fn euclidean(p:(i32, i32), goal:(i32, i32)) -> i32 {
    let x = ((goal.0 - p.0).pow(2) + (goal.1 - p.1).pow(2)) as f64;
    x.sqrt() as i32
}

fn manhattan(p:(i32, i32), goal:(i32, i32)) -> i32 {
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
            _ => Err("Wrong heuristic input, choose from Manhattan, Euclidean or TileOut")
        }
    }
}