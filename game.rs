const CELL_WIDTH: usize = 10;
const CELL_HEIGHT: usize = 10;

const WIDTH: usize = CELL_WIDTH * 100;
const HEIGHT: usize = CELL_HEIGHT * 50;
const BYTES_PER_PIXEL: usize = 4;

#[derive(Clone, Copy, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

const PLAYER1: Color = Color { r: 255, g: 0, b: 0, a: 255 };
const PLAYER2: Color = Color { r: 0, g: 0, b: 255, a: 255 };


struct Player {
    x: usize,
    y: usize,
    color: Color,
    cell_color: Color,
}

struct Cell {
    belongs_to_player: usize,
    idx: usize,
    color: Color,
}

struct GameState {
    pixels: Vec<u8>,
    counter: f32,
    cells: Vec<Cell>,
    players: Vec<Player>,
}

fn get_cell_idx(x: usize, y: usize) -> usize {
    (y * WIDTH + x) * BYTES_PER_PIXEL
}

impl GameState {
    fn new() -> Self {
        let player1 = Player { x: WIDTH/4, y: HEIGHT/2, color: PLAYER1, cell_color: PLAYER2};
        let player2 = Player { x: WIDTH/4*3, y: HEIGHT/2, color: PLAYER2, cell_color: PLAYER1};
        let players = vec![player1, player2];

        let mut cells = vec![];
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let (belongs_to_player, color) = if x < WIDTH / 2 { (0, players[0].cell_color) } else { (1, players[1].cell_color) };
                let idx = get_cell_idx(x, y);
                cells.push(Cell { belongs_to_player, idx, color });
            }
        }
        GameState {
            pixels: vec![0; WIDTH * HEIGHT * BYTES_PER_PIXEL],
            counter: 0.0,
            cells,
            players,
        }
    }


    fn update(&mut self, dt: f32) {
        self.counter += dt;
        
        for cell in self.cells.iter_mut() {
            self.pixels[cell.idx] = cell.color.r;
            self.pixels[cell.idx + 1] = cell.color.g;
            self.pixels[cell.idx + 2] = cell.color.b;
            self.pixels[cell.idx + 3] = cell.color.a;
        }

        // for player in self.players.iter_mut() {
        //     let cell = &self.cells[get_cell_idx(player.x, player.y)];
        //     self.pixels[cell.idx] = player.color.r;
        //     self.pixels[cell.idx + 1] = player.color.g;
        //     self.pixels[cell.idx + 2] = player.color.b;
        //     self.pixels[cell.idx + 3] = player.color.a;
        // }
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

