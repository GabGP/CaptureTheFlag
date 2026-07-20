use macroquad::prelude::*;

#[macroquad::main("Capture The Flag")]
async fn main() {
    loop {
        clear_background(BLACK);
        next_frame().await
    }
}
