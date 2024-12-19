use std::f32::consts::PI;
extern "C" {
    fn set_canvas_size(width: usize, height: usize);
    fn fill_rect(x: f32, y: f32, w: f32, h: f32, color: u32);
    fn fill_rect_border(x: f32, y: f32, w: f32, h: f32, color: u32);
    fn fill_circle(x: f32, y: f32, r: f32, color: u32);
    fn fill_circle_border(x: f32, y: f32, r: f32, color: u32);
    fn draw_text(x: f32, y: f32, textPtr: *const u8, textLen: usize, color: u32);
    fn clear_background(color: u32);

    fn console_log(textPtr: *const u8, textLen: usize);
}

unsafe fn display_text(text: &str, x: f32, y: f32, color: u32) {
    draw_text(x, y, text.as_ptr(), text.len(), color);
}

unsafe fn log_text(text: &str) {
    console_log(text.as_ptr(), text.len());
}

// Color comes in as 0xRRGGBBAA format
const BLACK: u32 = 0x000000FF;
const RED: u32 = 0xFF0000FF;
const BLUE: u32 = 0x0000FFFF;
const GREEN: u32 = 0x00FF00FF;
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

struct Vector2 {
    x: f32,
    y: f32,
}

struct Player {
    position: Vector2,
    velocity: Vector2,
    color: u32,
    cell_color: u32,
}

// players array
static mut PLAYERS: [Player; 2] = [
    Player { 
        position: Vector2 { 
            x: 0.0, 
            y: 0.0 
        }, 
        velocity: Vector2 { 
            x: 0.0, 
            y: 0.0 
        },
        color: PLAYER_1_COLOR,
        cell_color: PLAYER_1_CELL_COLOR,
    },
    Player { 
        position: Vector2 { 
            x: 0.0, 
            y: 0.0 
        }, 
        velocity: Vector2 { 
            x: 0.0, 
            y: 0.0 
        },
        color: PLAYER_2_COLOR,
        cell_color: PLAYER_2_CELL_COLOR,
    },
];

// board is matrix of BOARD_WIDTH * BOARD_HEIGHT filled with indices of the players
static mut BOARD: [[usize; BOARD_WIDTH]; BOARD_HEIGHT] = [[0; BOARD_WIDTH]; BOARD_HEIGHT];

unsafe fn player_eats_enemy_cell(px: f32, py: f32, player_index: usize) -> bool {
    let bx = ((px - PLAYER_RADIUS)/CELL_SIZE).floor() as usize;
    let by = ((py - PLAYER_RADIUS)/CELL_SIZE).floor() as usize;
    let tx = ((px + PLAYER_RADIUS)/CELL_SIZE).floor() as usize;
    let ty = ((py + PLAYER_RADIUS)/CELL_SIZE).floor() as usize;
    for x in bx..tx {
        for y in by..ty {
            if y>=BOARD_HEIGHT || x>=BOARD_WIDTH {
                continue;
            }
            if BOARD[y][x] != player_index {
                BOARD[y][x] = player_index;
                return true;
            }
        }
    }
    return false;
}

#[no_mangle]
pub fn update_frame(dt: f32) {
    unsafe {
        clear_background(BACKGROUND_COLOR);
        for by in 0..BOARD_HEIGHT {
            for bx in 0..BOARD_WIDTH {
                let x = bx as f32 * CELL_SIZE ;
                let y = by as f32 * CELL_SIZE ;
                let w = CELL_SIZE ;
                let h = CELL_SIZE ;
                let player_index = BOARD[by][bx];
                let color = PLAYERS[player_index].cell_color;
                fill_rect(x, y, w, h, color);
                fill_rect_border(x, y, w, h, BLACK);
            }
        }

        for (i, player) in PLAYERS.iter_mut().enumerate() {
            let nx = player.position.x + player.velocity.x * dt;
            
            if nx-PLAYER_RADIUS < 0.0 || nx+PLAYER_RADIUS > SCREEN_WIDTH as f32 || player_eats_enemy_cell(nx, player.position.y, i) {
                player.velocity.x *= -1.0;
            } else {
                player.position.x = nx;
            }
            
            let ny = player.position.y + player.velocity.y * dt;
            if ny-PLAYER_RADIUS < 0.0 || ny+PLAYER_RADIUS > SCREEN_HEIGHT as f32 || player_eats_enemy_cell(player.position.x, ny, i) {
                player.velocity.y *= -1.0;
            } else {
                player.position.y = ny;
            }

            fill_circle(player.position.x, player.position.y, PLAYER_RADIUS, player.color);
            fill_circle_border(player.position.x, player.position.y, PLAYER_RADIUS, WHITE);
        }

        let fps = 1.0 / dt;
        display_text(&format!("FPS: {}", fps.round()), 20.0, 20.0, WHITE);
    }
}

pub fn main() {
    unsafe {
        set_canvas_size(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize);
        
        // Initialize players
        PLAYERS[0].position.x = SCREEN_WIDTH /4.0;
        PLAYERS[0].position.y = SCREEN_HEIGHT /2.0;
        PLAYERS[0].velocity.x = (PI*0.25).cos()*PLAYER_SPEED;
        PLAYERS[0].velocity.y = (PI*0.25).sin()*PLAYER_SPEED;
        
        PLAYERS[1].position.x = SCREEN_WIDTH /4.0*3.0;
        PLAYERS[1].position.y = SCREEN_HEIGHT /2.0;
        PLAYERS[1].velocity.x = (PI*1.25).cos()*PLAYER_SPEED;
        PLAYERS[1].velocity.y = (PI*1.25).sin()*PLAYER_SPEED;

        // Initialize board
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                if x < BOARD_WIDTH/2 {
                    BOARD[y][x] = 0;
                } else {
                    BOARD[y][x] = 1;
                }
            }
        }
    }
}

