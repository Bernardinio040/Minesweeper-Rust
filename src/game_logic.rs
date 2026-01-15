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
    pub board: Vec<Vec<Cell>>,
    pub is_initialized: bool,
    pub is_game_over: bool,
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
            is_game_over: false,
        }
    }

    //function that will generate bombs after first click
    pub fn generate_bombs(&mut self, first_x: usize, first_y: usize) {
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

            self.calculate_numbers();
        }

        self.reveal_cell(first_x, first_y);
    }

    pub fn calculate_numbers(&mut self) {
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                if self.board[x][y].is_mine {
                    continue;
                }

                let mut count = 0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;

                        if nx < 0 || nx >= self.size_x as i32 || ny < 0 || ny >= self.size_y as i32
                        {
                            continue;
                        }

                        if self.board[nx as usize][ny as usize].is_mine {
                            count += 1;
                        }
                    }
                }

                self.board[x][y].neighbor_mines = count;
            }
        }
    }

    pub fn reveal_cell(&mut self, x: usize, y: usize) {
        if self.board[x][y].is_revealed {
            return;
        }

        //game over
        if self.board[x][y].is_mine {
            self.is_game_over = true;
            return;
        }

        self.board[x][y].is_revealed = true;

        if self.board[x][y].neighbor_mines == 0 {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if nx < 0 || nx >= self.size_x as i32 || ny < 0 || ny >= self.size_y as i32 {
                        continue;
                    }

                    self.reveal_cell(nx as usize, ny as usize);
                }
            }
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

                print!("{}{} ", symbol, if cell.is_revealed { "R" } else { " " });
            }
            println!();
        }
    }
}
