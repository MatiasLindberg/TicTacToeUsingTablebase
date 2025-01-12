use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

const SIZE: usize = 3;

#[derive(PartialEq, Clone, Serialize, Deserialize)]
enum Mark {
    EMPTY,
    CROSS,
    NOUGHT,
}
#[derive(Serialize, Deserialize)]
struct Grid {
    grid: Vec<Vec<Mark>>,
}

fn change(board: &mut Grid, x: usize, y: usize, change_to: Mark) {
    board.grid[y][x] = change_to;
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

#[macroquad::main("TicTacToe")]
async fn main() {
    request_new_screen_size(720.0, 720.0);

    let mut board: Grid = Grid {
        grid: vec![vec![Mark::EMPTY; SIZE]; SIZE],
    };

    loop {
        if is_key_pressed(KeyCode::Escape) {
            println!("Exiting program!");
            return;
        } else if is_mouse_button_pressed(MouseButton::Left) {
            let pos: (f32, f32) = mouse_position();
            println!("x: {} , y: {}", pos.0, pos.1);
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
            println!("x: {} , y: {}", x, y);
            if x != SIZE && y != SIZE {
                change(&mut board, x, y, Mark::CROSS);
            }
        }

        clear_background(BLACK);
        draw_rectangle_lines(60.0, 60.0, 600.0, 600.0, 20.0, LIGHTGRAY);
        draw_rectangle_lines(255.0, 60.0, 210.0, 600.0, 20.0, LIGHTGRAY);
        draw_rectangle_lines(60.0, 255.0, 600.0, 210.0, 20.0, LIGHTGRAY);
        draw_circle_lines(60.0, 60.0, 1.0, 1.0, BLUE);
        for r in 0..SIZE {
            for c in 0..SIZE {
                match board.grid[r][c] {
                    Mark::CROSS => draw_cross(c, r),
                    Mark::NOUGHT => draw_nought(c, r),
                    Mark::EMPTY => {}
                }
            }
        }
        next_frame().await
    }
}
