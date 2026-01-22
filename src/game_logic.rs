use rand::Rng;

#[derive(Clone, Debug, Copy)]
pub struct Cell {
    pub is_mine: bool,
    pub is_revealed: bool,
    pub is_flagged: bool,
    pub neighbour_mines: u8, //0-8
}

impl Cell {
    fn empty() -> Self {
        Self {
            is_mine: false,
            is_revealed: false,
            is_flagged: false,
            neighbour_mines: 0,
        }
    }
}

pub struct Game {
    pub size_x: usize,
    pub size_y: usize,
    pub mines: u32,
    pub board: Vec<Vec<Cell>>,
    pub is_initialized: bool,
    pub is_game_over: bool,
    pub is_win: bool,
}

impl Game {
    pub fn new(size_x: usize, size_y: usize, mines: u32) -> Self {
        let board = vec![vec![Cell::empty(); size_y]; size_x];

        Game {
            size_x,
            size_y,
            mines,
            is_initialized: false,
            board,
            is_game_over: false,
            is_win: false,
        }
    }

    //function that will generate mines after first click
    pub fn generate_mines(&mut self, first_x: usize, first_y: usize) {
        let mut rng = rand::rng();
        let mut placed = 0;

        while placed < self.mines {
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

                self.board[x][y].neighbour_mines = count;
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
            self.is_win = false;
            return;
        }

        self.board[x][y].is_revealed = true;

        if self.board[x][y].neighbour_mines == 0 {
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

        self.check_win_condition();
    }

    fn check_win_condition(&mut self) {
        let mut revealed_count = 0;
        let total_cells = self.size_x * self.size_y;

        for x in 0..self.size_x {
            for y in 0..self.size_y {
                if self.board[x][y].is_revealed {
                    revealed_count += 1;
                }
            }
        }

        //win condition is if all cells are revealed except for the mines
        if revealed_count == (total_cells - self.mines as usize) {
            self.is_game_over = true;
            self.is_win = true;
        }
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) {
        if self.board[x][y].is_revealed || self.is_game_over {
            return;
        }
        self.board[x][y].is_flagged = !self.board[x][y].is_flagged;
    }

    //chording in minesweeper is when you click on a cell with a number and have flagged cells adjacent to that number.
    //adjacent hidden cells are revealed
    pub fn chord_cell(&mut self, x: usize, y: usize) {
        if !self.board[x][y].is_revealed || self.board[x][y].neighbour_mines == 0 {
            return;
        }

        let mut flags_count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && nx < self.size_x as i32 && ny >= 0 && ny < self.size_y as i32 {
                    if self.board[nx as usize][ny as usize].is_flagged {
                        flags_count += 1;
                    }
                }
            }
        }

        if flags_count == self.board[x][y].neighbour_mines {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if nx >= 0 && nx < self.size_x as i32 && ny >= 0 && ny < self.size_y as i32 {
                        let ux = nx as usize;
                        let uy = ny as usize;

                        if !self.board[ux][uy].is_flagged && !self.board[ux][uy].is_revealed {
                            self.reveal_cell(ux, uy);
                        }
                    }
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
                } else if cell.neighbour_mines == 0 {
                    ".".to_string()
                } else {
                    cell.neighbour_mines.to_string()
                };

                print!("{}{} ", symbol, if cell.is_revealed { "R" } else { " " });
            }
            println!();
        }
    }
}
