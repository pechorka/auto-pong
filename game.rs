use std::f32::consts::PI;
use std::sync::Mutex;
use std::sync::OnceLock;

// External JS functions wrapper
mod js {
    #[link(wasm_import_module = "env")]
    extern "C" {
        fn extern_set_canvas_size(width: usize, height: usize);
        fn extern_fill_rect(x: f32, y: f32, w: f32, h: f32, color: u32);
        fn extern_fill_rect_border(x: f32, y: f32, w: f32, h: f32, color: u32);
        fn extern_fill_circle(x: f32, y: f32, r: f32, color: u32);
        fn extern_fill_circle_border(x: f32, y: f32, r: f32, color: u32);
        fn extern_draw_text(x: f32, y: f32, textPtr: *const u8, textLen: usize, color: u32);
        fn extern_clear_background(color: u32);
        #[allow(dead_code)]
        fn extern_console_log(textPtr: *const u8, textLen: usize);
    }

    // Safe wrapper functions

    pub fn set_canvas_size(width: usize, height: usize) {
        unsafe {
            extern_set_canvas_size(width, height);
        }
    }

    pub fn fill_rect(x: f32, y: f32, w: f32, h: f32, color: u32) {
        unsafe {
            extern_fill_rect(x, y, w, h, color);
        }
    }

    pub fn fill_rect_border(x: f32, y: f32, w: f32, h: f32, color: u32) {
        unsafe {
            extern_fill_rect_border(x, y, w, h, color);
        }
    }

    pub fn fill_circle(x: f32, y: f32, r: f32, color: u32) {
        unsafe {
            extern_fill_circle(x, y, r, color);
        }
    }

    pub fn fill_circle_border(x: f32, y: f32, r: f32, color: u32) {
        unsafe {
            extern_fill_circle_border(x, y, r, color);
        }
    }

    pub fn draw_text(text: &str, x: f32, y: f32, color: u32) {
        unsafe {
            extern_draw_text(x, y, text.as_ptr(), text.len(), color);
        }
    }

    pub fn clear_background(color: u32) {
        unsafe {
            extern_clear_background(color);
        }
    }

    #[allow(dead_code)]
    pub fn console_log(text: &str) {
        unsafe {
            extern_console_log(text.as_ptr(), text.len());
        }
    }
}



static GAME_STATE: OnceLock<Mutex<GameState>> = OnceLock::new();

#[no_mangle]
pub fn update_frame(dt: f32) {
    GAME_STATE.get_or_init(|| Mutex::new(GameState::new())).lock().unwrap().update(dt);
}

fn main() {}

// Color comes in as 0xRRGGBBAA format
const BLACK: u32 = 0x000000FF;
const RED: u32 = 0xFF0000FF;
const BLUE: u32 = 0x0000FFFF;
const WHITE: u32 = 0xFFFFFFFF;

const BOARD_HEIGHT: usize = 20;
const BOARD_WIDTH: usize = BOARD_HEIGHT * 2;
const CELL_SIZE: f32 = 20.0;
const SCREEN_WIDTH: f32 = BOARD_WIDTH as f32 * CELL_SIZE;
const SCREEN_HEIGHT: f32 = BOARD_HEIGHT as f32 * CELL_SIZE;

const PLAYER_RADIUS: f32 = CELL_SIZE as f32;
const PLAYER_SPEED: f32 = 500.0;

const BACKGROUND_COLOR: u32 = BLACK;
const PLAYER_1_COLOR: u32 = RED;
const PLAYER_2_COLOR: u32 = BLUE;
const PLAYER_1_CELL_COLOR: u32 = PLAYER_2_COLOR;
const PLAYER_2_CELL_COLOR: u32 = PLAYER_1_COLOR;

#[derive(Clone, Copy)]
struct Vector2 {
    x: f32,
    y: f32,
}

#[derive(Clone)]
struct Player {
    position: Vector2,
    velocity: Vector2,
    color: u32,
    cell_color: u32,
}

// Game state structure to manage mutable state
struct GameState {
    players: [Player; 2],
    board: [[usize; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl GameState {
    fn new() -> Self {
        Self {
            players: Self::new_players(),
            board: Self::new_board(),
        }
    }

    fn new_players() -> [Player; 2] {
        [
            Player {
                position: Vector2 { 
                    x: SCREEN_WIDTH / 4.0, 
                    y: SCREEN_HEIGHT / 2.0 
                },
                velocity: Vector2 { 
                    x: (PI * 0.25).cos() * PLAYER_SPEED, 
                    y: (PI * 0.25).sin() * PLAYER_SPEED 
                },
                color: PLAYER_1_COLOR,
                cell_color: PLAYER_1_CELL_COLOR,
            },
            Player {
                position: Vector2 { 
                    x: SCREEN_WIDTH / 4.0 * 3.0, 
                    y: SCREEN_HEIGHT / 2.0 
                },
                velocity: Vector2 { 
                    x: (PI * 1.25).cos() * PLAYER_SPEED, 
                    y: (PI * 1.25).sin() * PLAYER_SPEED 
                },
                color: PLAYER_2_COLOR,
                cell_color: PLAYER_2_CELL_COLOR,
            },
        ]
    }

    fn new_board() -> [[usize; BOARD_WIDTH]; BOARD_HEIGHT] {
        let mut board = [[0; BOARD_WIDTH]; BOARD_HEIGHT];
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                board[y][x] = if x < BOARD_WIDTH/2 { 0 } else { 1 };
            }
        }
        board
    }

    fn player_eats_enemy_cell(&mut self, px: f32, py: f32, player_index: usize) -> bool {
        let bx = ((px - PLAYER_RADIUS)/CELL_SIZE).floor() as usize;
        let by = ((py - PLAYER_RADIUS)/CELL_SIZE).floor() as usize;
        let tx = ((px + PLAYER_RADIUS)/CELL_SIZE).floor() as usize;
        let ty = ((py + PLAYER_RADIUS)/CELL_SIZE).floor() as usize;
        
        for x in bx..tx {
            for y in by..ty {
                if y >= BOARD_HEIGHT || x >= BOARD_WIDTH {
                    continue;
                }
                if self.board[y][x] != player_index {
                    self.board[y][x] = player_index;
                    return true;
                }
            }
        }
        false
    }

    fn update(&mut self, dt: f32) {
        js::set_canvas_size(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize);
        js::clear_background(BACKGROUND_COLOR);

        // Draw board first
        for by in 0..BOARD_HEIGHT {
            for bx in 0..BOARD_WIDTH {
                let x = bx as f32 * CELL_SIZE;
                let y = by as f32 * CELL_SIZE;
                let w = CELL_SIZE;
                let h = CELL_SIZE;
                let player_index = self.board[by][bx];
                let color = self.players[player_index].cell_color;
                js::fill_rect(x, y, w, h, color);
                js::fill_rect_border(x, y, w, h, BLACK);
            }
        }

        // Update players
        for i in 0..self.players.len() {
            // Calculate new positions first
            let current_pos = self.players[i].position;
            let current_vel = self.players[i].velocity;
            let mut new_pos = current_pos;
            let mut new_vel = current_vel;
            
            // Update X position
            let nx = current_pos.x + current_vel.x * dt;
            if nx - PLAYER_RADIUS < 0.0 || 
               nx + PLAYER_RADIUS > SCREEN_WIDTH as f32 || 
               self.player_eats_enemy_cell(nx, current_pos.y, i) {
                new_vel.x *= -1.0;
            } else {
                new_pos.x = nx;
            }
            
            // Update Y position
            let ny = current_pos.y + current_vel.y * dt;
            if ny - PLAYER_RADIUS < 0.0 || 
               ny + PLAYER_RADIUS > SCREEN_HEIGHT as f32 || 
               self.player_eats_enemy_cell(new_pos.x, ny, i) {
                new_vel.y *= -1.0;
            } else {
                new_pos.y = ny;
            }

            // Update player state
            self.players[i].position = new_pos;
            self.players[i].velocity = new_vel;

            // Draw player
            js::fill_circle(new_pos.x, new_pos.y, PLAYER_RADIUS, self.players[i].color);
            js::fill_circle_border(new_pos.x, new_pos.y, PLAYER_RADIUS, WHITE);
        }

        let fps = 1.0 / dt;
        js::draw_text(&format!("FPS: {}", fps.round()), 20.0, 20.0, WHITE);
    }
}



