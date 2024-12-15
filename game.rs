use std::f32::consts::PI;
extern "C" {
    fn set_canvas_size(width: usize, height: usize);
    fn fill_rect(x: f32, y: f32, w: f32, h: f32, color: u32);
    fn fill_circle(x: f32, y: f32, r: f32, color: u32);
    fn clear_background(color: u32);
}

// Color comes in as 0xRRGGBBAA format
const BLACK: u32 = 0x000000FF;
const RED: u32 = 0xFF0000FF;
const BLUE: u32 = 0x0000FFFF;

const BOARD_HEIGHT: usize = 20;
const BOARD_WIDTH: usize = BOARD_HEIGHT * 2;
const CELL_SIZE: f32 = 20.0;
const SCREEN_WIDTH: f32 = BOARD_WIDTH as f32 * CELL_SIZE;
const SCREEN_HEIGHT: f32 = BOARD_HEIGHT as f32 * CELL_SIZE;

const PLAYER_RADIUS: f32 = CELL_SIZE as f32;
const PLAYER_SPEED: f32 = 100.0;

const BACKGROUND_COLOR: u32 = BLACK;
const PLAYER_1_COLOR: u32 = RED;
const PLAYER_2_COLOR: u32 = BLUE;

struct Vector2 {
    x: f32,
    y: f32,
}

struct Player {
    position: Vector2,
    velocity: Vector2,
    color: u32,
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
        color: PLAYER_1_COLOR
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
        color: PLAYER_2_COLOR
    },
];

// board is matrix of BOARD_WIDTH * BOARD_HEIGHT filled with indices of the players
static mut BOARD: [[usize; BOARD_WIDTH]; BOARD_HEIGHT] = [[0; BOARD_WIDTH]; BOARD_HEIGHT];


fn rect_circle_collision(l: f32, r: f32, t: f32, b: f32, px: f32, py: f32, rad: f32) -> bool {
    let x = l.min(r.max(px)).max(l);
    let y = t.min(b.max(py)).max(t);
    let dx = px - x;
    let dy = py - y;
    return dx*dx + dy*dy <= rad*rad;
}

unsafe fn player_eats_enemy_cell(px: f32, py: f32, player_index: usize) -> bool {
    let bx = (px - PLAYER_RADIUS/CELL_SIZE) as usize;
    let by = (py - PLAYER_RADIUS/CELL_SIZE) as usize;
    let tx = (px + PLAYER_RADIUS/CELL_SIZE) as usize;
    let ty = (py + PLAYER_RADIUS/CELL_SIZE) as usize;
    for x in bx..tx {
        for y in by..ty {
            let l = x as f32 * CELL_SIZE;
            let r = (x + 1) as f32 * CELL_SIZE;
            let t = y as f32 * CELL_SIZE;
            let b = (y + 1) as f32 * CELL_SIZE;
            if rect_circle_collision(l, r, t, b, px, py, PLAYER_RADIUS) && BOARD[y][x] != player_index {
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
                let color = PLAYERS[player_index].color;
                fill_rect(x, y, w, h, color);
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
        }
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
                    BOARD[y][x] = 1;
                } else {
                    BOARD[y][x] = 0;
                }
            }
        }
    }
}

