use macroquad::prelude::*;

#[macroquad::main("TicTacToe")]
async fn main() {
    loop {
        clear_background(WHITE);

        draw_text("TicTacToe!", 30.0, 50.0, 50.0, DARKGRAY);

        next_frame().await
    }
}
