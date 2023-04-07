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

fn set_piece(grid: &mut[[GridContentType; N_COLS as usize]; N_ROWS as usize],
             input_index: &i32,
             piece_char: &char)
{
    let coordinates = __INDEX_MAP__[(*input_index-1) as usize];
    grid[coordinates[0] as usize][coordinates[1] as usize] = *piece_char;
}

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