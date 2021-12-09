use super::types::Matrix;

///copy starting board in circled order, return this new board
fn reordered_board(board: &Vec<i32>, row: i32) -> Vec<i32> {
    let mut reordered = Vec::with_capacity((row * row) as usize);
    let mut mark_board = vec![-1; (row * row) as usize];
    let mut x: i32 = 0;
    let mut ix: i32 = 1;
    let mut y: i32 = 0;
    let mut iy: i32 = 0;
    for _ in 0..row * row {
        mark_board[(x + y * row) as usize] = 0;
        reordered.push(board[(x + y * row) as usize]);
        if x + ix == row || x + ix < 0 || (ix != 0 && mark_board[(x + ix + y * row) as usize] == 0)
        {
            iy = ix;
            ix = 0;
        } else if y + iy == row
            || y + iy < 0
            || (iy != 0 && mark_board[(x + (y + iy) * row) as usize] == 0)
        {
            ix = -iy;
            iy = 0;
        }
        x += ix;
        y += iy;
    }
    reordered
}

fn inversion_count(board: &Vec<i32>, row: i32) -> u32 {
    let board = reordered_board(board, row);
    let mut inv_count = 0;
    for i in 0..board.len() {
        for j in i + 1..board.len() {
            if board[i] != 0 && board[j] != 0 && board[i] > board[j] {
                inv_count += 1;
            }
        }
    }
    inv_count
}

pub fn unsolvable_check(m: &Matrix, row: i32) -> bool {
    let count = inversion_count(&m.data, row);
    match count % 2 {
        0 => false,
        1 => true,
        _ => unreachable!(),
    }
}
