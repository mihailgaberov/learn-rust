use macroquad::prelude::*;

pub struct Game {
    pub quit: bool,
  }
  
  impl Game {
    pub fn new() -> Self {
        Self { quit: false }
    }
    pub fn update(&mut self) {
        if is_key_down(KeyCode::Escape) {
            self.quit = true;
        }
    }
    pub fn draw(&self) {
        clear_background(color_u8!(255, 255, 255, 255));
    }
  }
 
#[macroquad::main("game")]
async fn main() {
    let mut game = Game::new();
    loop {
        game.update();
        game.draw();
        if game.quit {
            return;
        }
        next_frame().await
    }
}