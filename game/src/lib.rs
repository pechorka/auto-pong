use wasm_bindgen::prelude::*;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

// We'll store our pixel buffer here. RGBA = 4 bytes per pixel.
static mut PIXELS: [u8; WIDTH * HEIGHT * 4] = [0; WIDTH * HEIGHT * 4];

// DVD logo properties
static mut LOGO_X: i32 = 50;
static mut LOGO_Y: i32 = 50;
static mut LOGO_VX: i32 = 2;
static mut LOGO_VY: i32 = 2;
const LOGO_WIDTH: i32 = (FONT_WIDTH + FONT_SPACING) * 3 - FONT_SPACING;
const LOGO_HEIGHT: i32 = FONT_HEIGHT;

// Define a simple 5x7 pixel font for characters
const FONT_WIDTH: i32 = 5;
const FONT_HEIGHT: i32 = 7;
const FONT_SPACING: i32 = 1;

// Basic 5x7 pixel font data - each character is represented by an array of bytes
// where each bit represents a pixel (1 = pixel on, 0 = pixel off)
const FONT_DATA: &[(&str, [u8; FONT_HEIGHT as usize])] = &[
    ("D", [
        0b11110000,
        0b10001000,
        0b10001000,
        0b10001000,
        0b10001000,
        0b10001000,
        0b11110000,
    ]),
    ("V", [
        0b10001000,
        0b10001000,
        0b10001000,
        0b10001000,
        0b10001000,
        0b01010000,
        0b00100000,
    ]),
    ("W", [
        0b10001000,
        0b10001000,
        0b10001000,
        0b10101000,
        0b10101000,
        0b11011000,
        0b10001000,
    ]),
    ("A", [
        0b00100000,
        0b01010000,
        0b01010000,
        0b01110000,
        0b01110000,
        0b01010000,
        0b01010000,
    ]),
    ("S", [
        0b00100000,
        0b01010000,
        0b01000000,
        0b00110000,
        0b00010000,
        0b01010000,
        0b00110000,
    ]),
    ("E", [
        0b01110000,
        0b01000000,
        0b01000000,
        0b01110000,
        0b01000000,
        0b01000000,
        0b01110000,
    ]),
    ("N", [
        0b10001000,
        0b11001000,
        0b10101000,
        0b10101000,
        0b10101000,
        0b10111000,
        0b10001000,
    ]),
];

// Function to draw a single character
fn draw_char(x: i32, y: i32, ch: &str, r: u8, g: u8, b: u8, a: u8) {
    if let Some((_char, bitmap)) = FONT_DATA.iter().find(|(c, _)| *c == ch) {
        for row in 0..FONT_HEIGHT {
            for col in 0..FONT_WIDTH {
                let pixel_on = (bitmap[row as usize] & (0b10000000 >> col)) != 0;
                if pixel_on {
                    let pixel_x = x + col;
                    let pixel_y = y + row;
                    
                    if pixel_x >= 0 && pixel_x < WIDTH as i32 && 
                       pixel_y >= 0 && pixel_y < HEIGHT as i32 {
                        unsafe {
                            let idx = ((pixel_y as usize) * WIDTH + (pixel_x as usize)) * 4;
                            PIXELS[idx] = r;
                            PIXELS[idx + 1] = g;
                            PIXELS[idx + 2] = b;
                            PIXELS[idx + 3] = a;
                        }
                    }
                }
            }
        }
    }
}

// A utility function to clear the buffer to a solid color (e.g. black).
fn clear_buffer() {
    unsafe {
        for i in 0..(WIDTH * HEIGHT * 4) {
            PIXELS[i] = 0; // Black
        }
    }
}

// New function to draw text (replaces draw_logo)
fn draw_logo(x: i32, y: i32) {
    // let text = ["D", "V", "D"];
    let text = ["S","A","D","E","V","A","N","A"];
    let (r, g, b, a) = (255, 0, 255, 255); // Keep the same bright color
    
    for (i, ch) in text.iter().enumerate() {
        let char_x = x + (i as i32) * (FONT_WIDTH + FONT_SPACING);
        draw_char(char_x, y, ch, r, g, b, a);
    }
}

#[wasm_bindgen]
pub fn update_frame() {
    // Update the position of the logo
    unsafe {
        LOGO_X += LOGO_VX;
        LOGO_Y += LOGO_VY;

        // Bounce off walls
        if LOGO_X < 0 || LOGO_X + LOGO_WIDTH > WIDTH as i32 {
            LOGO_VX = -LOGO_VX;
        }
        if LOGO_Y < 0 || LOGO_Y + LOGO_HEIGHT > HEIGHT as i32 {
            LOGO_VY = -LOGO_VY;
        }

        // Clear and redraw
        clear_buffer();
        draw_logo(LOGO_X, LOGO_Y);
    }
}

#[wasm_bindgen]
pub fn get_width() -> u32 {
    WIDTH as u32
}

#[wasm_bindgen]
pub fn get_height() -> u32 {
    HEIGHT as u32
}

#[wasm_bindgen]
pub fn get_pixels_ptr() -> *const u8 {
    unsafe { PIXELS.as_ptr() }
}
