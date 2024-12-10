
const CELL_WIDTH: usize = 10;
const CELL_HEIGHT: usize = 10;

const WIDTH: usize = CELL_WIDTH * 100;
const HEIGHT: usize = CELL_HEIGHT * 50;
const BYTES_PER_PIXEL: usize = 4;
static mut PIXELS: [u8; WIDTH * HEIGHT * BYTES_PER_PIXEL] = [0; WIDTH * HEIGHT * BYTES_PER_PIXEL];
static mut COUNTER: f32 = 0.0;


struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

const PLAYER1: Color = Color { r: 255, g: 0, b: 0, a: 255 };
const PLAYER2: Color = Color { r: 0, g: 0, b: 255, a: 255 };

fn write_color(color: Color, idx: usize) {
    unsafe {
        PIXELS[idx] = color.r;
        PIXELS[idx + 1] = color.g;
        PIXELS[idx + 2] = color.b;
        PIXELS[idx + 3] = color.a;
    }
}

#[no_mangle]
pub fn update(dt: f32) {
    unsafe {
        COUNTER += dt;
        
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let idx = ((y * WIDTH + x) * BYTES_PER_PIXEL) as usize;
                
                if x < WIDTH / 2 {
                    write_color(PLAYER1, idx);
                } else {
                    write_color(PLAYER2, idx);
                }
            }
        }
    }
}

#[no_mangle]
pub fn get_pixels() -> *const u8 {
    unsafe {
        PIXELS.as_ptr()
    }
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
