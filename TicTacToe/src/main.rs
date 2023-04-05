// Type aliases
type GridContentType = char;
// Constant variables
const N_ROWS: u32 = 3;
const N_COLS: u32 = 3;


fn print_grid(grid: &[[GridContentType; N_COLS as usize]; N_ROWS as usize]) {
    println!("===== PRINTING GRID =====");
    for row in grid.iter() {
            println!("     {:?} ", row);
    }
    println!("========================");
}

fn main() {
    // Fixed-size array.
    let mut grid: [[GridContentType; N_COLS as usize]; N_ROWS as usize] = [['-'; N_COLS as usize]; N_ROWS as usize];

    // Fill the grid with values.
    grid[0][1] = 'X';
    grid[1][1] = 'X';
    grid[1][2] = 'O';

    print_grid(&grid);

}
