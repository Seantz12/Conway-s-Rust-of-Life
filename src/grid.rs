use std::fmt;

struct Cell {
    alive: bool
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printed_char;
        if (&self).alive {
            printed_char = '*';
        } else {
            printed_char = '-';
        }
        write!(f, "{}", printed_char)
    }
}

impl Cell {
    fn create_cell() -> Cell{
        Cell {alive: false}
    }
}

pub struct Grid {
    length: usize,
    width: usize,
    grid: Vec<Vec<Cell>>
}

impl Grid {
    pub fn return_dimensions(&self) -> (usize, usize) {
        (self.length, self.width)
    }

    pub fn print_grid(&self) {
        for row in &(self.grid) {
            for character in row {
                print!("{}", character);
            }
            println!();
        }
    }

    pub fn create_grid(length: usize, width: usize) -> Grid {
        let mut grid = vec![];
        for x in 0..length {
            let mut vec = vec![];
            for y in 0..width {
                let new_cell = Cell::create_cell();
                vec.push(new_cell);
            }
            grid.push(vec);
        }
        Grid {length, width, grid}
    }
}