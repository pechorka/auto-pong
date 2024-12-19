use std::f32::consts::PI;
use std::sync::Mutex;
use std::sync::OnceLock;



static GAME_STATE: OnceLock<Mutex<GameState>> = OnceLock::new();

#[no_mangle]
pub fn update_frame(dt: f32) {
    GAME_STATE.get_or_init(|| Mutex::new(GameState::new())).lock().unwrap().update(dt);
}

#[no_mangle]
pub fn set_player_speed(speed: f32) {
    GAME_STATE.get_or_init(|| Mutex::new(GameState::new())).lock().unwrap().set_player_speed(speed);
}

fn main() {}

// Color comes in as 0xRRGGBBAA format
const BLACK: u32 = 0x000000FF;
const RED: u32 = 0xFF0000FF;
const BLUE: u32 = 0x0000FFFF;
const WHITE: u32 = 0xFFFFFFFF;

const DEFAULT_BOARD_HEIGHT: usize = 20;
const DEFAULT_BOARD_WIDTH: usize = DEFAULT_BOARD_HEIGHT * 2;
const DEFAULT_CELL_SIZE: f32 = 20.0;
const DEFAULT_SCREEN_WIDTH: f32 = DEFAULT_BOARD_WIDTH as f32 * DEFAULT_CELL_SIZE;
const DEFAULT_SCREEN_HEIGHT: f32 = DEFAULT_BOARD_HEIGHT as f32 * DEFAULT_CELL_SIZE;

const DEFAULT_PLAYER_RADIUS: f32 = DEFAULT_CELL_SIZE as f32;
const DEFAULT_PLAYER_SPEED: f32 = 500.0;

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
    direction: Vector2,
    speed: f32,
    color: u32,
    cell_color: u32,
}

impl Player {
    fn velocity(&self) -> Vector2 {
        Vector2 {
            x: self.direction.x * self.speed,
            y: self.direction.y * self.speed,
        }
    }
}

// Game state structure to manage mutable state
struct GameState {
    players: [Player; 2],
    board: Vec<usize>,
    board_width: usize,
    board_height: usize,
    cell_size: f32,
    screen_width: f32,
    screen_height: f32,
    player_radius: f32,
}

impl GameState {
    fn new() -> Self {
        let board_width = DEFAULT_BOARD_WIDTH;
        let board_height = DEFAULT_BOARD_HEIGHT;
        let cell_size = DEFAULT_CELL_SIZE;
        let screen_width = DEFAULT_SCREEN_WIDTH;
        let screen_height = DEFAULT_SCREEN_HEIGHT;
        let player_radius = DEFAULT_PLAYER_RADIUS;
        Self {
            players: Self::new_players(DEFAULT_PLAYER_SPEED, screen_width, screen_height),
            board: Self::new_board(0, 1, board_width, board_height),
            board_width,
            board_height,
            cell_size,
            screen_width,
            screen_height,
            player_radius,
        }
    }

    fn new_players(player_speed: f32, screen_width: f32, screen_height: f32) -> [Player; 2] {
        [
            Player {
                position: Vector2 { 
                    x: screen_width / 4.0, 
                    y: screen_height / 2.0 
                },
                direction: Vector2 { 
                    x: (PI * 0.25).cos(), 
                    y: (PI * 0.25).sin() 
                },
                speed: player_speed,
                color: PLAYER_1_COLOR,
                cell_color: PLAYER_1_CELL_COLOR,
            },
            Player {
                position: Vector2 { 
                    x: screen_width / 4.0 * 3.0, 
                    y: screen_height / 2.0 
                },
                direction: Vector2 { 
                    x: (PI * 1.25).cos(), 
                    y: (PI * 1.25).sin() 
                },
                speed: player_speed,
                color: PLAYER_2_COLOR,
                cell_color: PLAYER_2_CELL_COLOR,
            },
        ]
    }

    fn new_board(left_player_index: usize, right_player_index: usize, width: usize, height: usize) -> Vec<usize> {
        let mut board = vec![0; width * height];
        for y in 0..height {
            for x in 0..width {
                board[y * width + x] = if x < width/2 { left_player_index } else { right_player_index };
            }
        }
        board
    }

    fn player_eats_enemy_cell(&mut self, px: f32, py: f32, player_index: usize) -> bool {
        // player bounds in border coordinates
        let bx = ((px - self.player_radius)/self.cell_size).floor() as usize;
        let by = ((py - self.player_radius)/self.cell_size).floor() as usize;
        let tx = ((px + self.player_radius)/self.cell_size).floor() as usize;
        let ty = ((py + self.player_radius)/self.cell_size).floor() as usize;
        
        for x in bx..tx {
            for y in by..ty {
                if y >= self.board_height || x >= self.board_width {
                    continue;
                }
                let idx = y * self.board_width + x;
                if self.board[idx] != player_index {
                    self.board[idx] = player_index;
                    return true;
                }
            }
        }
        false
    }

    fn update(&mut self, dt: f32) {
        js::set_canvas_size(self.screen_width as usize, self.screen_height as usize);
        js::clear_background(BACKGROUND_COLOR);

        // Draw board first
        for by in 0..self.board_height {
            for bx in 0..self.board_width {
                let x = bx as f32 * self.cell_size;
                let y = by as f32 * self.cell_size;
                let w = self.cell_size;
                let h = self.cell_size;
                let player_index = self.board[by * self.board_width + bx];
                let color = self.players[player_index].cell_color;
                js::fill_rect(x, y, w, h, color);
                js::fill_rect_border(x, y, w, h, BLACK);
            }
        }

        // Update players
        for i in 0..self.players.len() {
            // Calculate new positions first
            let current_pos = self.players[i].position;
            let current_dir = self.players[i].direction;
            let current_vel = self.players[i].velocity();
            let mut new_pos = current_pos;
            let mut new_dir = current_dir;
            
            // Update X position
            let nx = current_pos.x + current_vel.x * dt;
            if nx - self.player_radius < 0.0 || 
               nx + self.player_radius > self.screen_width || 
               self.player_eats_enemy_cell(nx, current_pos.y, i) {
                new_dir.x *= -1.0;
            } else {
                new_pos.x = nx;
            }
            
            // Update Y position
            let ny = current_pos.y + current_vel.y * dt;
            if ny - self.player_radius < 0.0 || 
               ny + self.player_radius > self.screen_height || 
               self.player_eats_enemy_cell(new_pos.x, ny, i) {
                new_dir.y *= -1.0;
            } else {
                new_pos.y = ny;
            }

            // Update player state
            self.players[i].position = new_pos;
            self.players[i].direction = new_dir;

            // Draw player
            js::fill_circle(new_pos.x, new_pos.y, self.player_radius, self.players[i].color);
            js::fill_circle_border(new_pos.x, new_pos.y, self.player_radius, WHITE);
        }

        let fps = 1.0 / dt;
        js::draw_text(&format!("FPS: {}", fps.round()), 20.0, 20.0, WHITE);
    }

    fn set_player_speed(&mut self, speed: f32) {
        for player in &mut self.players {
            player.speed = speed;
        }
    }
}


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