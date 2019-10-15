mod grid;

use std::io;
use grid::Grid;

fn main() {
    let mut input = String::from("");
    println!("Hello, world! Enter size:");
    io::stdin().read_line(&mut input).expect("Failed");
    input.pop();
    let size: usize = input.parse().unwrap();
    println!("You entered {}", size);
    let full_grid = Grid::create_grid(size, size);
    full_grid.print_grid();
    let (length, width) = full_grid.return_dimensions();
    println!("This was a {} by {} grid", length, width);

}