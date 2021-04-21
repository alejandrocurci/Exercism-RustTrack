use std::char;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let numeric_board = get_board(minefield);
    let numeric_board = count_mines(&numeric_board);
    rebuild_board(&numeric_board)
}

// converts the string board into a numeric board
fn get_board(board: &[&str]) -> Vec<Vec<i32>> {
    board
        .iter()
        .map(|row| {
            row.chars()
                .map(|letter| match letter {
                    '*' => -1,
                    _ => 0,
                })
                .collect()
        })
        .collect()
}

// finds the location of every mine and updates the surrounding numbers 
fn count_mines(board: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut new_board = board.clone();
    board
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, cell)| (i, j, cell)))
        .filter(|(_, _, cell)| **cell == -1)
        .for_each(|(i, j, _)| update_board(&mut new_board, (i, j)));
    new_board
}

// updates the numeric cells surrounding mine cells
fn update_board(board: &mut Vec<Vec<i32>>, (i, j): (usize, usize)) {
    board
        .iter_mut()
        .enumerate()
        .filter(|(x, _)| (*x as i32 - i as i32).abs() <= 1 )
        .flat_map(|(_, row)| {
            row.iter_mut()
                .enumerate()
                .filter(|(y, _)| (*y as i32 - j as i32).abs() <= 1)
                .filter(|(_, cell)| **cell != -1)
                .map(|(_, cell)| cell)
        })
        .for_each(|cell| *cell += 1);
}

// rebuilds the string board from the numeric board
fn rebuild_board(board: &Vec<Vec<i32>>) -> Vec<String> {
    board
        .iter()
        .map(|row| {
            row.iter()
                .map(|n| match n {
                    -1 => '*',
                    0 => ' ',
                    _ => char::from_digit(*n as u32, 10).unwrap(),
                })
                .collect()
        })
        .collect()
}
