use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

const SIZE: usize = 3;

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
enum Mark {
    EMPTY,
    CROSS,
    NOUGHT,
}
#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<Mark>>,
}

impl Grid {
    fn reset_grid(&mut self) {
        for y in 0..SIZE {
            for x in 0..SIZE {
                self.grid[y][x] = Mark::EMPTY;
            }
        }
    }

    fn is_full(&self) -> bool {
        for y in 0..SIZE {
            for x in 0..SIZE {
                if self.grid[y][x] == Mark::EMPTY {
                    return false;
                }
            }
        }
        true
    }

    fn is_win(&self) -> Mark {
        if self.grid[1][1] != Mark::EMPTY
            && ((self.grid[1][1] == self.grid[0][1] && self.grid[1][1] == self.grid[2][1])
                || (self.grid[1][1] == self.grid[1][0] && self.grid[1][1] == self.grid[1][2])
                || (self.grid[1][1] == self.grid[0][0] && self.grid[1][1] == self.grid[2][2])
                || (self.grid[2][0] == self.grid[1][1] && self.grid[1][1] == self.grid[0][2]))
        {
            return self.grid[1][1].clone();
        }
        if self.grid[0][0] != Mark::EMPTY
            && ((self.grid[0][0] == self.grid[1][0] && self.grid[0][0] == self.grid[2][0])
                || (self.grid[0][0] == self.grid[0][1] && self.grid[0][0] == self.grid[0][2]))
        {
            return self.grid[0][0].clone();
        }
        if self.grid[2][2] != Mark::EMPTY
            && ((self.grid[2][2] == self.grid[1][2] && self.grid[2][2] == self.grid[0][2])
                || (self.grid[2][2] == self.grid[2][1] && self.grid[2][2] == self.grid[2][0]))
        {
            return self.grid[2][2].clone();
        }
        Mark::EMPTY
    }

    fn flatten_grid(&self) -> String {
        let mut s: String = "".to_string();

        for y in 0..SIZE {
            for x in 0..SIZE {
                match self.grid[y][x] {
                    Mark::EMPTY => s.push('E'),
                    Mark::CROSS => s.push('C'),
                    Mark::NOUGHT => s.push('N'),
                }
            }
        }
        s
    }

    fn possible_moves(&self) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = Vec::new();
        for y in 0..SIZE {
            for x in 0..SIZE {
                if self.grid[y][x] == Mark::EMPTY {
                    moves.push((y, x));
                }
            }
        }
        moves
    }

    fn change(&mut self, y: usize, x: usize, change_to: Mark) {
        self.grid[y][x] = change_to;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct CSVGrid {
    now: String,
    next_moves: String,
}

impl CSVGrid {
    fn unflatten(&self) -> Vec<Vec<Mark>> {
        let mut board: Vec<Vec<Mark>> = vec![vec![Mark::EMPTY; SIZE]; SIZE];
        let mut s = self.next_moves.chars();
        for y in 0..SIZE {
            for x in 0..SIZE {
                match s.next() {
                    Some('E') => board[y][x] = Mark::EMPTY,
                    Some('C') => board[y][x] = Mark::CROSS,
                    Some('N') => board[y][x] = Mark::NOUGHT,
                    _ => {
                        println!("Error");
                        return vec![vec![]];
                    }
                }
            }
        }
        board
    }

    fn write_csv(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut wtr = csv::Writer::from_path("Tablebase.csv")?;
        wtr.serialize(self)?;
        wtr.flush()?;
        Ok(())
    }

    fn best_moves(&self) -> Vec<(usize, usize)> {
        vec![]
    }
}
fn draw_background() {
    clear_background(BLACK);
    draw_rectangle_lines(60.0, 60.0, 600.0, 600.0, 20.0, LIGHTGRAY);
    draw_rectangle_lines(255.0, 60.0, 210.0, 600.0, 20.0, LIGHTGRAY);
    draw_rectangle_lines(60.0, 255.0, 600.0, 210.0, 20.0, LIGHTGRAY);
}

fn draw_cross(x: usize, y: usize) {
    let s1: f32 = 100.0 + (x * 200) as f32;
    let mut s2: f32 = 100.0 + (y * 200) as f32;
    let e1: f32 = 220.0 + (x * 200) as f32;
    let mut e2: f32 = 220.0 + (y * 200) as f32;
    draw_line(s1, s2, e1, e2, 20.0, RED);
    s2 += 120.0;
    e2 -= 120.0;
    draw_line(s1, s2, e1, e2, 20.0, RED);
}

fn draw_nought(x: usize, y: usize) {
    let xf: f32 = 160.0 + (x * 200) as f32;
    let yf: f32 = 160.0 + (y * 200) as f32;
    draw_circle_lines(xf, yf, 50.0, 20.0, DARKGREEN);
}

fn read_csv() -> Result<Vec<(usize, usize)>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open("Tablebase.csv")?;
    let mut rdr = csv::Reader::from_reader(std::io::BufReader::new(file));
    let mut records = Vec::new();

    Ok(records)
}

fn minmax(board: &mut Grid, depth: i32, maxing: bool) -> i32 {
    match board.is_win() {
        Mark::CROSS => return depth - SIZE as i32,
        Mark::NOUGHT => return SIZE as i32 - depth,
        Mark::EMPTY => {
            if board.is_full() {
                return 0;
            }
        }
    };

    if maxing {
        let mut max_score: i32 = i32::MIN;
        for (y, x) in board.possible_moves() {
            board.change(y, x, Mark::NOUGHT);
            let score = minmax(board, depth + 1, false);
            board.change(y, x, Mark::EMPTY);
            max_score = max_score.max(score);
        }
        max_score
    } else {
        let mut min_score: i32 = i32::MAX;
        for (y, x) in board.possible_moves() {
            board.change(y, x, Mark::CROSS);
            let score: i32 = minmax(board, depth + 1, true);
            board.change(y, x, Mark::EMPTY);
            min_score = min_score.min(score);
        }
        min_score
    }
}

fn generate_tablebase() {
    let mut next_grids: Vec<Grid> = Vec::new();
    next_grids.push(Grid {
        grid: vec![vec![Mark::EMPTY; SIZE]; SIZE],
    });

    while let Some(ng) = next_grids.pop() {
        if ng.is_win() != Mark::EMPTY {
            continue;
        }
        let mut tmp_grids: Vec<Grid> = Vec::new();
        for (y, x) in ng.possible_moves() {
            let mut board_player = ng.clone();
            board_player.change(y, x, Mark::CROSS);

            // find best move for AI after player played cross.
            let mut max_score = i32::MIN;
            tmp_grids.clear();
            for (ty, tx) in board_player.possible_moves() {
                let mut board_ai = board_player.clone();
                board_ai.change(ty, tx, Mark::NOUGHT);
                let score = minmax(&mut board_ai, 1, true);
                if score >= max_score {
                    if score > max_score {
                        max_score = score;
                        tmp_grids.clear();
                    }
                    println!("{:?}", board_ai.grid);
                    tmp_grids.push(board_ai);
                }
            }
        }
        next_grids.extend(tmp_grids.clone());
    }
}

#[macroquad::main("TicTacToe")]
async fn main() {
    request_new_screen_size(720.0, 720.0);

    let mut board: Grid = Grid {
        grid: vec![vec![Mark::EMPTY; SIZE]; SIZE],
    };

    if !std::path::Path::new("Tablebase.csv").exists() {
        generate_tablebase();
    } else {
        //read_csv();
    }

    loop {
        if is_key_pressed(KeyCode::Escape) {
            println!("Exiting program!");
            return;
        } else if is_mouse_button_pressed(MouseButton::Left) {
            let pos: (f32, f32) = mouse_position();
            let x: usize = match pos.0 {
                70.0..255.0 => 0,
                265.0..455.0 => 1,
                465.0..650.0 => 2,
                _ => SIZE,
            };
            let y: usize = match pos.1 {
                70.0..255.0 => 0,
                265.0..455.0 => 1,
                465.0..650.0 => 2,
                _ => SIZE,
            };
            if x != SIZE && y != SIZE {
                board.change(y, x, Mark::CROSS);
            }
        }

        draw_background();
        for y in 0..SIZE {
            for x in 0..SIZE {
                match board.grid[y][x] {
                    Mark::CROSS => draw_cross(x, y),
                    Mark::NOUGHT => draw_nought(x, y),
                    Mark::EMPTY => {}
                }
            }
        }
        next_frame().await
    }
}
