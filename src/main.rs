fn main() {
    let mut grid = create_grid();

    for (_i, row) in grid.iter().enumerate() {
        for (_j, col) in row.iter().enumerate() {
            print!("{}", col);
        }
        println!()
    }
}

// note the grid is set as [[u8; coloumn]; row];
fn create_grid() -> [[u8; 4]; 2]{
    const M: usize = 2;
    const N: usize = 4;

    let grid: [[u8; 4]; 2] = [[0 as u8; N] ; M];

    grid
}