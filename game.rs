const CELL_WIDTH: usize = 10;
const CELL_HEIGHT: usize = 10;

const WIDTH: usize = CELL_WIDTH * 100;
const HEIGHT: usize = CELL_HEIGHT * 50;
const BYTES_PER_PIXEL: usize = 4;

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

const PLAYER1: Color = Color { r: 255, g: 0, b: 0, a: 255 };
const PLAYER2: Color = Color { r: 0, g: 0, b: 255, a: 255 };

struct GameState {
    pixels: Vec<u8>,
    counter: f32,
}

impl GameState {
    fn new() -> Self {
        GameState {
            pixels: vec![0; WIDTH * HEIGHT * BYTES_PER_PIXEL],
            counter: 0.0,
        }
    }

    fn write_color(&mut self, color: Color, idx: usize) {
        self.pixels[idx] = color.r;
        self.pixels[idx + 1] = color.g;
        self.pixels[idx + 2] = color.b;
        self.pixels[idx + 3] = color.a;
    }

    fn update(&mut self, dt: f32) {
        self.counter += dt;
        
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let idx = (y * WIDTH + x) * BYTES_PER_PIXEL;
                
                if x < WIDTH / 2 {
                    self.write_color(PLAYER1, idx);
                } else {
                    self.write_color(PLAYER2, idx);
                }
            }
        }
    }

    fn get_pixels_ptr(&self) -> *const u8 {
        self.pixels.as_ptr()
    }
}

use std::sync::{Mutex, OnceLock};

static GAME_STATE: OnceLock<Mutex<GameState>> = OnceLock::new();

fn get_game_state() -> &'static Mutex<GameState> {
    GAME_STATE.get_or_init(|| Mutex::new(GameState::new()))
}

#[no_mangle]
pub fn update(dt: f32) {
    get_game_state().lock().unwrap().update(dt);
}

#[no_mangle]
pub fn get_pixels() -> *const u8 {
    get_game_state().lock().unwrap().get_pixels_ptr()
}

#[no_mangle]
pub fn get_width() -> usize {
    WIDTH
}

#[no_mangle]
pub fn get_height() -> usize {
    HEIGHT
}

fn main() {}

