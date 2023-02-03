// sudoku solver by Tars Nijman, 2023
// TODO: make board into a struct with board, bitfield_matrix, solvabile, backtrack_count, iterations

use std::time;


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
    bitfield_matrix: &mut [[u16; 9]; 9],
	board: &[[u8; 9]; 9],
    cell_x: usize,
    cell_y: usize,
    value: u8,
) -> () {
	
	if value == 0 {
		if board[cell_y][cell_x] == 0 {
			return;
		}
		
		for i in 0..9 {
		    let bitmask_disable: u16 = !(1 << (board[cell_y][cell_x] - 1));
		    let block_position_x: usize = (cell_x / 3) * 3;
		    let block_position_y: usize = (cell_y / 3) * 3;
			
	        (*bitfield_matrix)[cell_y][i] &= bitmask_disable;
	        (*bitfield_matrix)[i][cell_x] &= bitmask_disable;
	        (*bitfield_matrix)[block_position_y + i / 3][block_position_x + i % 3] &= bitmask_disable;
	    }
		return;
	}
	
    // remove the value from every cell in the given and column row with a for loop in the following way:
    // create a number with all ones except the flag to set, which is a zero
    // store this in a variable so we only have to calculate it once
    // doing a bitwise AND will set the flag bit to zero
    let bitmask_disable: u16 = !(1 << (value - 1));
    let block_position_x: usize = (cell_x / 3) * 3;
    let block_position_y: usize = (cell_y / 3) * 3;

    for i in 0..9 {
        (*bitfield_matrix)[cell_y][i] &= bitmask_disable;
        (*bitfield_matrix)[i][cell_x] &= bitmask_disable;
        (*bitfield_matrix)[block_position_y + i / 3][block_position_x + i % 3] &= bitmask_disable;
    }
    // finally, set the bitfield at the given position to 0, since this cell is filled and so
    // there are no more possible values for it
    (*bitfield_matrix)[cell_y][cell_x] = 0u16;
}

// TODO: implement more rules for solving
fn update_board(board: &mut [[u8; 9]; 9], bitfield_matrix: &[[u16; 9]; 9]) -> () {
    // see if we can already place some cells based on the bitfield_matrix

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
                (*board)[y][x] = bitfield_matrix[y][x].trailing_zeros() as u8 + 1;
            }
            count2 |= cell_bitfield & bitfield_matrix[y][x];
            count1 |= bitfield_matrix[y][x];
        }
        // check if there is a bit that has appeared once or more but not twice or more
        // meaning it has appeared only once
        for x in 0..9 {
            let bitflag = x as u16 & count1 & !count2;

            if bitflag != 0 {
                (*board)[y][x] = bitflag.trailing_zeros() as u8 + 1;
            }
        }
    }
}

fn generate_bitfield_matrix(board: &[[u8; 9]; 9]) -> [[u16; 9]; 9] {
    let mut bitfield_matrix = [[u16::MAX; 9]; 9];
    for y in 0..9 {
        for x in 0..9 {
            if board[y][x] != 0 {
                update_bitfield_matrix(&mut bitfield_matrix, &board, x, y, board[y][x]);
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
    board: &mut [[u8; 9]; 9],
    bitfield_matrix: &mut [[u16; 9]; 9],
) -> bool {
    // make an x and y, which will be the positions of the next empty cell
    let (cell_x, cell_y);
    // if there are no empty cells left, exit with success
    if let Some((x, y)) = find_next_empty_cell(&board) {
        (cell_x, cell_y) = (x, y);
    } else {
        return true;
    }

    // go over all values that can be in a cell
    for value in 1..=9 {
        // if that value is valid, continue
        // else go to the next value
        if is_valid_cell(&bitfield_matrix, cell_x, cell_y, value) {
            // set the new value
            (*board)[cell_y][cell_x] = value;

			// println!("{:?}\n", board);
			// update the bitfield matrix
			update_bitfield_matrix(&mut *bitfield_matrix, &board, cell_x, cell_y, value);

            //update_board(&mut *board, &bitfield_matrix);

            // try to solve it; if possible return the solved board			
            if solve_sudoku(&mut *board, &mut *bitfield_matrix) {
                return true;
            } else {
				(*board)[cell_y][cell_x] = 0u8;
				update_bitfield_matrix(&mut *bitfield_matrix, &board, cell_x, cell_y, 0);
			}
        }
    }
    // if no values fit then this board is not solvable and it's returned
    false
}

#[allow(unused)]
fn main() {
    let mut normal_sudoku = [
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

	   let mut normal_sudoku_bitfield_matrix = generate_bitfield_matrix(&normal_sudoku);

    // local runtime:
    // 250ms on dev build
    // 6.37ms on release build
    let mut hardest_sudoku = [
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

    let mut hardest_sudoku_bitfield_matrix = generate_bitfield_matrix(&hardest_sudoku);

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

	let mut empty_sudoku = [[0u8; 9]; 9];
	let mut empty_sudoku_bitfield_matrix = generate_bitfield_matrix(&empty_sudoku);

    // let mut test = [[0i32; 9]; 9];
    // test[2][2] = 1i32;
    // test[6][8] = 4i32;

    let iterations = 1;
    let now = time::Instant::now();


	// solve_sudoku(&mut hardest_sudoku, &mut hardest_sudoku_bitfield_matrix);

	update_bitfield_matrix(&mut empty_sudoku_bitfield_matrix, &mut empty_sudoku, 3, 3, 5);

    println!(
        "{:?} \n {:?}",
        empty_sudoku, empty_sudoku_bitfield_matrix
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
