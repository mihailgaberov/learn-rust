use macroquad::prelude::*;

pub struct PlayerState {
    pub position: Vec2,
    pub rotation: f32,
}

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

        const ROT_SPEED: f32 = 0.015;
 
        if is_key_down(KeyCode::Right) {
            self.player_state.rotation += ROT_SPEED;
        }
        if is_key_down(KeyCode::Left) {
            self.player_state.rotation -= ROT_SPEED;
        }

        const SPEED: f32 = 0.6;
 
        self.player_state.position += vec2_from_angle(self.player_state.rotation) * SPEED;
    }
    pub fn draw(&self) {
        clear_background(color_u8!(255, 255, 255, 255));
 
        draw_poly_lines(
            self.player_state.position.x,
            self.player_state.position.y,
            3,
            10.,
self.player_state.rotation * 180. / std::f32::consts::PI - 90.,
            2.,
            BLACK
        );
 
        draw_box(Vec2::new(400f32, 200f32), Vec2::new(50f32, 20f32));
    }

    fn draw_box(pos: Vec2, size: Vec2) {
        let dimension = size * 2.;
        let upper_left = pos - size;
     
        draw_rectangle(upper_left.x, upper_left.y, dimension.x, dimension.y, BLACK);
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