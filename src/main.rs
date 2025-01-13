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
    fn write_csv(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("Tablebase.csv")?;

        let mut wtr = csv::Writer::from_writer(file);
        wtr.serialize(self)?;
        wtr.flush()?;
        Ok(())
    }

    fn find_moves(&self) -> Vec<(usize, usize)> {
        let mut pos: Vec<(usize, usize)> = Vec::new();
        for part in self.next_moves.split(':') {
            let p: Vec<char> = part.chars().collect();
            if let (Some(y), Some(x)) = (p[0].to_digit(10), p[1].to_digit(10)) {
                pos.push((y as usize, x as usize));
            }
        }
        pos
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

fn minmax(board: &mut Grid, depth: i32, maxing: bool) -> i32 {
    match board.is_win() {
        Mark::CROSS => return -100 + depth as i32,
        Mark::NOUGHT => return 100 - depth as i32,
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

    let mut grid_set: std::collections::HashSet<String> = std::collections::HashSet::new();

    while let Some(mut ng) = next_grids.pop() {
        if ng.is_win() != Mark::EMPTY {
            continue;
        }
        for (y, x) in ng.possible_moves() {
            let mut tmp_grids: Vec<Grid> = Vec::new();
            let mut tmp_pos: Vec<(usize, usize)> = Vec::new();

            ng.change(y, x, Mark::CROSS);

            let ns: String = ng.flatten_grid();
            if grid_set.contains(&ns) {
                continue;
            }
            grid_set.insert(ns.to_string());

            // find best move for AI after player played cross.
            let mut max_score = i32::MIN;
            for (ty, tx) in ng.possible_moves() {
                ng.change(ty, tx, Mark::NOUGHT);
                let score = minmax(&mut ng, 1, true);
                if score >= max_score {
                    if score > max_score {
                        max_score = score;
                        tmp_grids.clear();
                        tmp_pos.clear();
                    }
                    tmp_pos.push((ty, tx));
                    tmp_grids.push(ng.clone());
                }
                ng.change(ty, tx, Mark::EMPTY);
            }
            let s: String = tmp_pos
                .iter()
                .map(|(y, x)| format!("{}{}", y, x))
                .collect::<Vec<String>>()
                .join(":");

            let csv: CSVGrid = CSVGrid {
                now: ng.flatten_grid(),
                next_moves: s,
            };

            match csv.write_csv() {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("Error when writing csv: {}", e);
                }
            }
            ng.change(y, x, Mark::EMPTY);
            next_grids.extend(tmp_grids);
        }
    }
}

fn find_next(board: &Grid) -> Result<Vec<(usize, usize)>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open("Tablebase.csv")?;
    let mut rdr = csv::Reader::from_reader(std::io::BufReader::new(file));
    let find_s = board.flatten_grid();

    for res in rdr.deserialize() {
        let csv_grid: CSVGrid = res?;
        if csv_grid.now == find_s {
            return Ok(csv_grid.find_moves());
        }
    }
    Ok(vec![])
}

fn ai_turn(board: &mut Grid) {
    match find_next(&board) {
        Ok(pos) => {
            board.change(pos[0].0, pos[0].1, Mark::NOUGHT);
        }
        Err(e) => {
            println!("Error finding next position. {}", e)
        }
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
                _ => continue,
            };
            let y: usize = match pos.1 {
                70.0..255.0 => 0,
                265.0..455.0 => 1,
                465.0..650.0 => 2,
                _ => continue,
            };
            if x != SIZE && y != SIZE && board.grid[y][x] == Mark::EMPTY {
                board.change(y, x, Mark::CROSS);
                if board.is_win() == Mark::CROSS {
                    println!("Player won!");
                    board.reset_grid();
                } else if board.is_full() {
                    println!("Ended in tie!");
                    board.reset_grid();
                } else {
                    ai_turn(&mut board);
                    if board.is_win() == Mark::NOUGHT {
                        println!("AI won!");
                        board.reset_grid();
                    }
                }
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
