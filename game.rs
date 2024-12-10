// Global static array for pixels
const WIDTH: usize = 1600;
const HEIGHT: usize = 900;
static mut PIXELS: [u8; WIDTH * HEIGHT * 4] = [0; WIDTH * HEIGHT * 4];
static mut COUNTER: f32 = 0.0;

#[no_mangle]
pub fn update(dt: f32) {
    unsafe {
        COUNTER += dt;
        
        // Update the checkerboard pattern
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let idx = ((y * WIDTH + x) * 4) as usize;
                // Create an alternating pattern that changeis with time
                let is_white = (x / 8 + y / 8 + (COUNTER as usize)) % 2 == 0;
                
                // Set RGBA values
                PIXELS[idx] = if is_white { 255 } else { 0 };     // R
                PIXELS[idx + 1] = if is_white { 255 } else { 0 }; // G
                PIXELS[idx + 2] = if is_white { 255 } else { 0 }; // B
                PIXELS[idx + 3] = 255;                            // A (always fully opaque)
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
