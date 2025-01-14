use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use serde::{Deserialize, Serialize};

const SIZE: usize = 3;

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize, Debug)]
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
        let mut s: String = String::new();

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
#[derive(Clone)]
struct Tablebase {
    tablebase: Vec<CSVGrid>,
}
impl Tablebase {
    fn load_tablebase() -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open("Tablebase.csv")?;
        let mut rdr = csv::Reader::from_reader(std::io::BufReader::new(file));
        let tablebase: Vec<CSVGrid> = rdr.deserialize().collect::<Result<_, _>>()?;
        Ok(Self { tablebase })
    }

    fn write_tablebase(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("Tablebase.csv")?;

        let mut wtr = csv::Writer::from_writer(file);

        for csv in &self.tablebase {
            wtr.serialize(csv)?;
        }
        wtr.flush()?;
        Ok(())
    }

    fn find_next(&self, find_s: &String) -> (usize, usize) {
        if let Some(nxt) = self
            .tablebase
            .iter()
            .find(|csv_grid| csv_grid.now == *find_s)
            .map(|csv_grid| csv_grid.find_moves())
        {
            match nxt.choose() {
                Some((y, x)) => return (*y, *x),
                None => println!("Could not choose one from vector."),
            }
        };

        (0, 0)
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

async fn draw_loading(tablebase_data: &std::sync::Arc<std::sync::Mutex<Option<Tablebase>>>) {
    while !tablebase_data.lock().unwrap().is_some() {
        clear_background(BLACK);
        draw_text("Calculating tablebase!", 100.0, 300.0, 60.0, DARKGREEN);
        next_frame().await;
    }
}

fn minmax(board: &mut Grid, depth: i32, maxing: bool) -> i32 {
    match board.is_win() {
        Mark::CROSS => return -100 + depth,
        Mark::NOUGHT => return 100 - depth,
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

fn generate_tablebase() -> Tablebase {
    let mut next_grids: Vec<Grid> = Vec::new();
    next_grids.push(Grid {
        grid: vec![vec![Mark::EMPTY; SIZE]; SIZE],
    });

    let mut table: Vec<CSVGrid> = Vec::new();
    let mut grid_set: std::collections::HashSet<String> = std::collections::HashSet::new();

    while let Some(ng) = next_grids.pop() {
        for (y, x) in ng.possible_moves() {
            let mut ng_cpy = ng.clone();
            ng_cpy.change(y, x, Mark::CROSS);

            let ns: String = ng_cpy.flatten_grid();
            if grid_set.contains(&ns) || ng_cpy.is_win() != Mark::EMPTY {
                continue;
            }
            grid_set.insert(ns.clone());

            // find best move for AI after player played cross.
            let mut max_score = i32::MIN;
            let mut best_grids: Vec<Grid> = Vec::new();
            let mut best_pos: Vec<(usize, usize)> = Vec::new();

            for (ty, tx) in ng_cpy.possible_moves() {
                let mut ai_grid = ng_cpy.clone();
                ai_grid.change(ty, tx, Mark::NOUGHT);
                let score = minmax(&mut ai_grid, 1, false);
                if score > max_score {
                    max_score = score;
                    best_grids = Vec::new();
                    best_pos = Vec::new();
                }
                if score == max_score {
                    best_pos.push((ty, tx));
                    best_grids.push(ai_grid);
                }
            }
            let s: String = best_pos
                .iter()
                .map(|(y, x)| format!("{}{}", y, x))
                .collect::<Vec<String>>()
                .join(":");

            let grid_s: String = ng_cpy.flatten_grid();

            let csv: CSVGrid = CSVGrid {
                now: grid_s,
                next_moves: s,
            };

            table.push(csv);
            next_grids.extend(best_grids);
        }
    }
    let base: Tablebase = Tablebase { tablebase: table };
    if let Err(e) = base.write_tablebase() {
        println!("Error writing tablebase! {}", e)
    }
    base
}

#[macroquad::main("TicTacToe")]
async fn main() {
    request_new_screen_size(720.0, 720.0);

    let mut board: Grid = Grid {
        grid: vec![vec![Mark::EMPTY; SIZE]; SIZE],
    };

    let tablebase: Tablebase;

    if !std::path::Path::new("Tablebase.csv").exists() {
        let tablebase_data = std::sync::Arc::new(std::sync::Mutex::new(None));
        let tb_data_clone = std::sync::Arc::clone(&tablebase_data);

        std::thread::spawn(move || {
            let tb = generate_tablebase();
            let mut data = tb_data_clone.lock().unwrap();
            *data = Some(tb);
        });
        draw_loading(&tablebase_data).await;

        let data = tablebase_data.lock().unwrap();
        if let Some(tb) = &*data {
            tablebase = tb.clone();
        } else {
            eprintln!("Error getting generated tablebase!");
            return;
        }
    } else {
        tablebase = Tablebase::load_tablebase().expect("Failed to load tablebase.");
    }

    let mut player_wins: usize = 0;
    let mut ai_wins: usize = 0;
    let mut ties: usize = 0;
    let mut ended: bool = false;

    loop {
        if is_key_pressed(KeyCode::Escape) {
            println!("Exiting program!");
            return;
        } else if ended && is_key_pressed(KeyCode::Backspace) {
            board.reset_grid();
            ended = false;
        } else if is_key_pressed(KeyCode::Enter) {
            println!(
                "Player has won {} times. Ai has won {} time. {} games have been tied.",
                player_wins, ai_wins, ties
            );
        } else if is_mouse_button_pressed(MouseButton::Left) && !ended {
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
            if x != SIZE && y != SIZE && board.grid[y][x] == Mark::EMPTY {
                board.change(y, x, Mark::CROSS);
                if board.is_win() == Mark::CROSS {
                    println!("Player won!");
                    player_wins += 1;
                    ended = true;
                } else if board.is_full() {
                    println!("Ended in tie!");
                    ties += 1;
                    ended = true;
                } else {
                    let (ny, nx) = tablebase.find_next(&board.flatten_grid());
                    board.change(ny, nx, Mark::NOUGHT);
                    if board.is_win() == Mark::NOUGHT {
                        println!("AI won!");
                        ai_wins += 1;
                        ended = true;
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
