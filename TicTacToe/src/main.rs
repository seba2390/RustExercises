#![crate_name = "doc"]

use std::io::Write;

// Type aliases
type GridContentType = char;
// Constant variables
const N_ROWS: u32 = 3;
const N_COLS: u32 = 3;
const N_ROUNDS: u32 = 6;
const N_PLAYERS: u32 = 2;
const NAN_TOKEN: char = 'N';
const __INDEX_MAP__: [[i32; 2]; 9]  = [[0,0], [0,1], [0,2],
                                       [1,0], [1,1], [1,2],
                                       [2,0], [2,1], [2,2]];


fn print_grid(grid: &[[GridContentType; N_COLS as usize]; N_ROWS as usize]) {
    println!("\nCurrent Board:");
    for row in grid.iter() {
        println!("-------------");
        print!("| ");
        for entry in row.iter() {
            if entry != &NAN_TOKEN
            {
                print!("{} | ", entry);
            }
            else
            {
                print!("  | ");
            }
        }
        println!(" ");
    }
    println!("-------------");
}

fn print_description() {
    println!(" ");
    println!(" ################## TicTacToe Game Description ###################");
    println!(" #                                                               #");
    println!(" #    1.) Entries in the board are indexed with integers, as:    #");
    println!(" #                       -------------                           #");
    println!(" #                       | 1 | 2 | 3 |                           #");
    println!(" #                       -------------                           #");
    println!(" #                       | 4 | 5 | 6 |                           #");
    println!(" #                       -------------                           #");
    println!(" #                       | 7 | 8 | 9 |                           #");
    println!(" #                       -------------                           #");
    println!(" #                                                               #");
    println!(" #    2.) The 2 players will take turns to place their piece     #");
    println!(" #        on the board.                                          #");
    println!(" #                                                               #");
    println!(" #    3.) Each of the 2 players will specify the placement of    #");
    println!(" #        their piece by an integer: 1,2,...,9, according to     #");
    println!(" #        the formatting displayed in 1.) above.                 #");
    println!(" #                                                               #");
    println!(" #################################################################");
}


/// Determines if a winning condition has been met on the provided Tic-Tac-Toe grid.
///
/// This function checks for three in a row, either horizontally, vertically, or diagonally, in order to
/// determine if a player has won the game.
///
/// # Arguments
///
/// * `grid` - A reference to a 3x3 array representing the Tic-Tac-Toe grid. Each element of the array is a `char`
///            representing the contents of that cell ('X', 'O', or ' ').
///
/// # Returns
///
/// A tuple of two booleans representing whether each player has won the game. The first element of the tuple
/// is `true` if player 1 has won, and `false` otherwise. The second element of the tuple is `true` if player
/// 2 has won, and `false` otherwise. If neither player has won, both elements of the tuple are `false`.
///
/// # Examples
///
/// ```
/// let grid = [['X', 'O', ' '], [' ', 'X', ' '], ['O', 'O', 'X']];
/// assert_eq!(has_winner(&grid), (true, false));
///
/// let grid = [['O', 'O', 'X'], ['X', 'X', 'O'], ['O', 'X', ' ']];
/// assert_eq!(has_winner(&grid), (false, true));
///
/// let grid = [['X', 'O', 'X'], ['X', 'O', 'O'], ['O', 'X', 'O']];
/// assert_eq!(has_winner(&grid), (false, false));
/// ```
fn has_winner(grid: &[[GridContentType; N_COLS as usize]; N_ROWS as usize]) -> (bool, bool) {
    // Checking rows
    for row in 0..3 {
        if grid[row][0] == grid[row][1] && grid[row][1] == grid[row][2] {
            if grid[row][0] == 'X'{
                return (true, false);
            }
            else if grid[row][0] == 'O' {
                return (false, true)
            }
        }
    }
    // Checking columns
    for col in 0..3 {
        if grid[0][col] == grid[1][col] && grid[1][col] == grid[2][col] {
            if grid[0][col] == 'X'{
                return (true, false);
            }
            else if grid[0][col] == 'O' {
                return (false, true)
            }
        }
    }
    // Checking diagonals
    if grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] {
        if grid[0][0] == 'X'{
            return (true, false);
        }
        else if grid[0][0] == 'O' {
            return (false, true)
        }
    }
    if grid[2][0] == grid[1][1] && grid[1][1] == grid[2][0] {
        if grid[2][0] == 'X'{
            return (true, false);
        }
        else if grid[2][0] == 'O' {
            return (false, true)
        }
    }
    return (false, false);
}

/// Determines if the game is over, i.e., if there are no more empty cells left or if one
/// player has three symbols in a row.
///
/// # Arguments
///
/// * `grid` - a reference to a 2D array of `GridContentType` with dimensions N_ROWS x N_COLS
///
/// # Returns
///
/// * `true` if the game is over, `false` otherwise.
///
/// # Examples
///
/// ```
/// let grid = [['X', 'O', 'X'], ['O', 'O', 'X'], ['O', 'X', 'O']];
/// let result = is_over(&grid);
/// assert_eq!(result, true);
/// ```
fn is_over(grid: &[[GridContentType; N_COLS as usize]; N_ROWS as usize]) -> bool {
    let (mut x_counter, mut o_counter): (u8, u8) = (0, 0);
    for row in grid.iter() {
        for entry in row.iter() {
            if *entry == 'X'{
                x_counter += 1;
            }
            else if *entry == 'O' {
                o_counter += 1;
            }

            if x_counter == 3 || o_counter == 3 {
                return true;
            }
        }
    }
    return false;
}

/// Sets the piece of the given `piece_char` at the index `input_index` in the `grid`.
///
/// # Arguments
///
/// * `grid` - A mutable reference to the grid in which the piece will be set.
/// * `input_index` - The index at which the piece will be set in the grid.
/// * `piece_char` - The character representing the piece to be set ('X' or 'O').
///
/// # Example
///
/// ```
/// use tic_tac_toe::{set_piece, GridContentType, N_COLS, N_ROWS};
///
/// let mut grid: [[GridContentType; N_COLS as usize]; N_ROWS as usize] = [[' '; N_COLS as usize]; N_ROWS as usize];
/// let input_index = 5;
/// let piece_char = &'X';
///
/// set_piece(&mut grid, &input_index, &piece_char);
/// assert_eq!(grid[1][1], 'X');
/// ```
fn set_piece(grid: &mut[[GridContentType; N_COLS as usize]; N_ROWS as usize],
             input_index: &i32,
             piece_char: &char)
{
    let coordinates = __INDEX_MAP__[(*input_index-1) as usize];
    grid[coordinates[0] as usize][coordinates[1] as usize] = *piece_char;
}


/// Checks whether a cell on the game grid is occupied.
///
/// This function takes in a mutable reference to the game grid and an input index as a reference.
/// The input index is used to map to the corresponding cell on the game grid.
/// If the cell is not empty, then this function returns true, indicating that the cell is occupied.
/// Otherwise, it returns false, indicating that the cell is empty.
///
/// # Arguments
///
/// * `grid` - A mutable reference to the game grid.
/// * `input_index` - The input index that maps to a cell on the game grid.
///
/// # Returns
///
/// * `true` if the cell on the game grid is occupied.
/// * `false` if the cell on the game grid is empty.
///
/// # Examples
///
/// ```
/// let mut grid = [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']];
/// assert_eq!(is_occupied(&mut grid, &5), false);
/// assert_eq!(is_occupied(&mut grid, &4), false);
///
/// grid[1][1] = 'X';
/// assert_eq!(is_occupied(&mut grid, &5), true);
/// assert_eq!(is_occupied(&mut grid, &4), false);
/// ```
fn is_occupied(grid: &mut[[GridContentType; N_COLS as usize]; N_ROWS as usize],
               input_index: &i32) -> bool
{
    let coordinates = __INDEX_MAP__[(*input_index-1) as usize];
    if grid[coordinates[0] as usize][coordinates[1] as usize] != NAN_TOKEN {
        return true;
    }
    return false
}


fn main() {

    print_description();

    // Fixed-size array.
    let mut grid: [[GridContentType; N_COLS as usize]; N_ROWS as usize] = [[NAN_TOKEN; N_COLS as usize]; N_ROWS as usize];

    // Starting game loop
    'game: for round in 0..N_ROUNDS {
        println!("\n ================= Round {} ==================\n", round+1);

        for player_nr in 0..N_PLAYERS {
            // Player input
            let player_char = if player_nr == 0 { 'X'} else { 'O'};
            println!("Player {} ('{}') - type a number and press Enter: ", player_nr+1, player_char);
            // Checking that index is within range and the cell is unoccupied
            #[allow(unused_assignments)] let mut player_input_num = 0;
            loop {
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Failed to read line");
                player_input_num = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                if (player_input_num >= 1) && (player_input_num <= 9) && (!is_occupied(& mut grid, &player_input_num)) {
                    break;
                }
                if player_input_num < 1 || player_input_num > 9 {
                    println!("Invalid input - please enter a number in the range [1, 9]!");
                } else if is_occupied(& mut grid, &player_input_num) {
                    println!("Invalid input - please choose a free cell!");
                }
            }
            set_piece(&mut grid, &player_input_num, &player_char);
            print_grid(&grid);

            // Checking for game finished
            if has_winner(&grid) == (true, false) {
                println!("\n # PLAYER 1 HAS WON - GAME FINISHED! ");
                break 'game;
            }
            else if has_winner(&grid) == (false, true) {
                println!("\n # PLAYER 2 HAS WON - GAME FINISHED! ");
                break 'game;
            }
            else if is_over(&grid) {
                println!("\n # TIEBREAK - NO WINNER! ");
                break 'game;
            }
        }
    }


}
