// sudoku solver by Tars Nijman, 2023
// TODO: make board into a struct with board, bitfield_matrix, solvabile, backtrack_count, iterations


use std::time;

struct Board<'a> {
    board: &'a [[u8; 9]; 9],
    bitfield_matrix: [[u16; 9]; 9],
    iterations: i32,
    backtrack_iterations: i32,
    is_solvable: bool,
}


fn find_next_empty_cell(board: &[[u8; 9]; 9]) -> Option<(usize, usize)> {
    // loop over the board until an empty cell is found
    // return it in an option
    for y in 0..9 {
        for x in 0..9 {
            if board[y][x] == 0 {
                return Some((x, y));
            }
        }
    }
    // if there isn't one, return None
    None
}

// update the given bitfield matrix to remove the flags of the given value from the bitfields
// in the row and column it's in
fn update_bitfield_matrix(
    bitfield_matrix: &[[u16; 9]; 9],
    cell_x: usize,
    cell_y: usize,
    value: u8,
) -> [[u16; 9]; 9] {
    let mut new_bitfield_matrix = bitfield_matrix.clone();
    // remove the value from every cell in the given and column row with a for loop in the following way:
    // create a number with all ones except the flag to set, which is a zero
    // store this in a variable so we only have to calculate it once
    // doing a bitwise AND will set the flag bit to zero
    let bitmask_disable: u16 = !(1 << (value - 1));
    let block_position_x: usize = (cell_x / 3) * 3;
    let block_position_y: usize = (cell_y / 3) * 3;

    for i in 0..9 {
        new_bitfield_matrix[cell_y][i] &= bitmask_disable;
        new_bitfield_matrix[i][cell_x] &= bitmask_disable;
        new_bitfield_matrix[block_position_y + i / 3][block_position_x + i % 3] &= bitmask_disable;
    }
    // finally, set the bitfield at the given position to 0, since this cell is filled and so
    // there are no more possible values for it
    new_bitfield_matrix[cell_y][cell_x] = 0u16;

    new_bitfield_matrix
}

// TODO: implement more rules for solving
fn update_board(board: &[[u8; 9]; 9], bitfield_matrix: &[[u16; 9]; 9]) -> [[u8; 9]; 9] {
    // see if we can already place some cells based on the bitfield_matrix
    let mut new_board = board.clone();

    for y in 0..9 {
        // count1 counts how many times a bitflag has appeared once or more
        // similarly, count2 counts how many times a bitflag has appeared twice or more
        let mut count1 = 0u16;
        let mut count2 = 0u16;

        for x in 0..9 {
            let cell_bitfield = bitfield_matrix[y][x];
            // if a bitfield only has one bit set to one, it means that's the only option
            // so we set the board at that index to that flag's corresponding value
            // if a bitfield only has one flag, that makes it a power of 2
            // we can do this simply by taking x & (x - 1)
            if (cell_bitfield & (cell_bitfield.wrapping_sub(1))) == 0 && cell_bitfield != 0 {
                new_board[y][x] = bitfield_matrix[y][x].trailing_zeros() as u8 + 1;
            }
            count2 |= cell_bitfield & bitfield_matrix[y][x];
            count1 |= bitfield_matrix[y][x];
        }
        // check if there is a bit that has appeared once or more but not twice or more
        // meaning it has appeared only once
        for x in 0..9 {
            let bitflag = x as u16 & count1 & !count2;

            if bitflag != 0 {
                new_board[y][x] = bitflag.trailing_zeros() as u8 + 1;
            }
        }
    }
    new_board
}

fn generate_bitfield_matrix(board: &[[u8; 9]; 9]) -> [[u16; 9]; 9] {
    let mut bitfield_matrix = [[u16::MAX; 9]; 9];
    for y in 0..9 {
        for x in 0..9 {
            if board[y][x] != 0 {
                bitfield_matrix = update_bitfield_matrix(&bitfield_matrix, x, y, board[y][x]);
            }
        }
    }
    bitfield_matrix
}

fn is_valid_cell(bitfield_matrix: &[[u16; 9]; 9], x: usize, y: usize, value: u8) -> bool {
    ((bitfield_matrix[y][x] >> (value - 1)) & 1) == 1
}

fn print_board(board: &[[u8; 9]; 9]) {
    // loop over row and prints each one
    for row in board {
        println!("{:?}", row);
    }
}

fn solve_sudoku(
    board: [[u8; 9]; 9],
    bitfield_matrix: [[u16; 9]; 9],
) -> ([[u8; 9]; 9], [[u16; 9]; 9], bool) {
    // make an x and y, which will be the positions of the next empty cell
    let (cell_x, cell_y);
    // if there are no empty cells left, exit with success
    if let Some((x, y)) = find_next_empty_cell(&board) {
        (cell_x, cell_y) = (x, y);
    } else {
        return (board, bitfield_matrix, true);
    }

    // go over all values that can be in a cell
    for value in 1..=9 {
        // if that value is valid, continue
        // else go to the next value
        if is_valid_cell(&bitfield_matrix, cell_x, cell_y, value) {
            // clone the board and set the new value
            let mut new_board = board;
            new_board[cell_y][cell_x] = value;

            // clone the bitfield matrix and update it with the new board
            let new_bitfield_matrix =
                update_bitfield_matrix(&bitfield_matrix, cell_x, cell_y, value);

            new_board = update_board(&new_board, &new_bitfield_matrix);

            // try to solve it; if possible return the solved board
            let solved = solve_sudoku(new_board, new_bitfield_matrix);
            if solved.2 {
                return (solved.0, solved.1, true);
            }
        }
    }
    // if no values fit then this board is not solvable and it's returned
    (board, bitfield_matrix, false)
}

#[allow(unused)]
fn main() {
    let normal_sudoku = [
        [5, 1, 7, 6, 0, 0, 0, 3, 4],
        [2, 8, 9, 0, 0, 4, 0, 0, 0],
        [3, 4, 6, 2, 0, 5, 0, 9, 0],
        [6, 0, 2, 0, 0, 0, 0, 1, 0],
        [0, 3, 8, 0, 0, 6, 0, 4, 7],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 9, 0, 0, 0, 0, 0, 7, 8],
        [7, 0, 3, 4, 0, 0, 5, 6, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    // local runtime:
    // 250ms on dev build
    // 6.37ms on release build
    let hardest_sudoku = [
        [8, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 3, 6, 0, 0, 0, 0, 0],
        [0, 7, 0, 0, 9, 0, 2, 0, 0],
        [0, 5, 0, 0, 0, 7, 0, 0, 0],
        [0, 0, 0, 0, 4, 5, 7, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 3, 0],
        [0, 0, 1, 0, 0, 0, 0, 6, 8],
        [0, 0, 8, 5, 0, 0, 0, 1, 0],
        [0, 9, 0, 0, 0, 0, 4, 0, 0],
    ];

    let hardest_sudoku_bitfield_matrix = generate_bitfield_matrix(&hardest_sudoku);

    let against_brute_force = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 3, 0, 8, 5],
        [0, 0, 1, 0, 2, 0, 0, 0, 0],
        [0, 0, 0, 5, 0, 7, 0, 0, 0],
        [0, 0, 4, 0, 0, 0, 1, 0, 0],
        [0, 9, 0, 0, 0, 0, 0, 0, 0],
        [5, 0, 0, 0, 0, 0, 0, 7, 3],
        [0, 0, 2, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 4, 0, 0, 0, 9],
    ];

    // let mut test = [[0i32; 9]; 9];
    // test[2][2] = 1i32;
    // test[6][8] = 4i32;

    let iterations = 10;
    let now = time::Instant::now();

    for _ in 0..iterations {
        solve_sudoku(hardest_sudoku, hardest_sudoku_bitfield_matrix);
    }

    println!(
        "{:?}",
        solve_sudoku(hardest_sudoku, hardest_sudoku_bitfield_matrix)
    );
    // let new_bitfield_matrix = generate_bitfield_matrix(&test);
    // println!("{:#?}", new_bitfield_matrix);

    let elapsed = now.elapsed();
    println!(
        "took {}ms for {} iterations, {}ms ({}us) per iter",
        elapsed.as_micros() as f64 / 1000.0,
        iterations,
        elapsed.as_micros() as f64 / (1000.0 * iterations as f64),
        elapsed.as_nanos() as f64 / (1000.0 * iterations as f64)
    );
}
