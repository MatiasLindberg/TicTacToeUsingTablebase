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
    fn change(&mut self, x: usize, y: usize, change_to: Mark) {
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

fn write_csv(next: CSVGrid) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = csv::Writer::from_path("Tablebase.csv")?;
    wtr.serialize(next)?;
    wtr.flush()?;
    Ok(())
}

fn read_csv() -> Result<Vec<(usize, usize)>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open("Tablebase.csv")?;
    let mut rdr = csv::Reader::from_reader(std::io::BufReader::new(file));
    let mut records = Vec::new();

    Ok(records)
}

fn generate_tablebase() {
    let mut board: Grid = Grid {
        grid: vec![vec![Mark::EMPTY; SIZE]; SIZE],
    };

    for (x, y) in board.possible_moves() {
        let mut board_cpy = board.clone();
        board_cpy.grid[y][x] = Mark::CROSS;
    }
}

fn is_win(board: &Grid) -> Mark {
    if board.grid[1][1] != Mark::EMPTY
        && ((board.grid[1][1] == board.grid[0][1] && board.grid[1][1] == board.grid[2][1])
            || (board.grid[1][1] == board.grid[1][0] && board.grid[1][1] == board.grid[1][2])
            || (board.grid[0][0] == board.grid[1][1] && board.grid[0][0] == board.grid[2][2])
            || (board.grid[0][0] == board.grid[1][1] && board.grid[1][1] == board.grid[2][2])
            || (board.grid[2][0] == board.grid[1][1] && board.grid[1][1] == board.grid[0][2]))
    {
        return board.grid[1][1].clone();
    }
    if board.grid[0][0] != Mark::EMPTY
        && ((board.grid[0][0] == board.grid[1][0] && board.grid[0][0] == board.grid[2][0])
            || (board.grid[0][0] == board.grid[0][1] && board.grid[0][0] == board.grid[0][2]))
    {
        return board.grid[0][0].clone();
    }
    if board.grid[2][2] != Mark::EMPTY
        && ((board.grid[2][2] == board.grid[1][2] && board.grid[2][2] == board.grid[0][2])
            || (board.grid[2][2] == board.grid[2][1] && board.grid[2][2] == board.grid[2][0]))
    {
        return board.grid[2][2].clone();
    }
    Mark::EMPTY
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
                board.change(x, y, Mark::CROSS);
            }
        }

        clear_background(BLACK);
        draw_rectangle_lines(60.0, 60.0, 600.0, 600.0, 20.0, LIGHTGRAY);
        draw_rectangle_lines(255.0, 60.0, 210.0, 600.0, 20.0, LIGHTGRAY);
        draw_rectangle_lines(60.0, 255.0, 600.0, 210.0, 20.0, LIGHTGRAY);
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
