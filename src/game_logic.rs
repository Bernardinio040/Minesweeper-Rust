pub struct Game {
    sizeX: u32,
    sizeY: u32,
    bombs: u32,
    board: Vec<Vec<i32>>,
}

//function that will generate bombs after first click
fn generateBombs(sizeX: u32, sizeY: u32, bombs: u32, x: u32, y: u32) {
    let mut board = vec![vec![0; sizeY as usize]; sizeX as usize];
    let mut rng = rand::thread_rng();
    let mut placed = 0;

    while placed < bombs {
        let row = rng.gen_range(0..sizeX);
        let col = rng.gen_range(0..sizeY);

        if row == x && col == y {
            continue;
        }

        if board[row as usize][col as usize] == 0 {
            board[row as usize][col as usize] = 1;
            placed += 1;
        }
    }

    board
}

fn debug_print_board(board: &Vec<Vec<i32>>) {
    let width = board.len();
    let height = board[0].len();

    for y in 0..height {
        print!("{:2} | ", y);

        for x in 0..width {
            let cell = board[x][y];

            let symbol = match cell {
                1 => "X".to_string(),
                0 => ".".to_string(),
                n => n.to_string(),
            };

            print!("{} ", symbol);
        }

        println!();
    }
}
