use rand::Rng;

#[derive(Clone, Debug, Copy)]
pub struct Cell {
    pub is_mine: bool,
    pub is_revealed: bool,
    pub is_flagged: bool,
    pub neighbor_mines: u8, //0-8
}

impl Cell {
    fn empty() -> Self {
        Self {
            is_mine: false,
            is_revealed: false,
            is_flagged: false,
            neighbor_mines: 0,
        }
    }
}

pub struct Game {
    pub size_x: usize,
    pub size_y: usize,
    pub bombs: u32,
    pub is_initialized: bool,
    pub board: Vec<Vec<Cell>>,
}

impl Game {
    pub fn new(size_x: usize, size_y: usize, bombs: u32) -> Self {
        let board = vec![vec![Cell::empty(); size_y]; size_x];

        Game {
            size_x,
            size_y,
            bombs,
            is_initialized: false,
            board,
        }
    }

    //function that will generate bombs after first click
    pub fn generateBombs(&mut self, first_x: usize, first_y: usize) {
        let mut rng = rand::rng();
        let mut placed = 0;

        while placed < self.bombs {
            let x = rng.random_range(0..self.size_x);
            let y = rng.random_range(0..self.size_y);

            if x == first_x && y == first_y {
                continue;
            }

            if !self.board[x][y].is_mine {
                self.board[x][y].is_mine = true;
                placed += 1;
            }

            //self.calculate_numbers();
        }
    }

    pub fn debug_print_board(game: &Game) {
        let width = game.size_x;
        let height = game.size_y;

        println!("Wait for first click to generate map...");

        for y in 0..height {
            print!("{:2} | ", y);
            for x in 0..width {
                let cell = &game.board[x][y];

                let symbol = if cell.is_mine {
                    "X".to_string()
                } else if cell.neighbor_mines == 0 {
                    ".".to_string()
                } else {
                    cell.neighbor_mines.to_string()
                };

                print!("{} ", symbol);
            }
            println!();
        }
    }
}
