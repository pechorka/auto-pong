#[no_mangle]
extern "C" {
    fn set_canvas_size(width: u32, height: u32);
    fn fill_rect(x: f32, y: f32, w: f32, h: f32, color: u32);
    fn clear_background(color: u32);
}

// Color comes in as 0xRRGGBBAA format
const BLACK: u32 = 0x000000FF;
const RED: u32 = 0xFF0000FF;

#[no_mangle]
pub fn update_frame(dt: f32) {
    unsafe {
        clear_background(BLACK);
        fill_rect(0.0, 0.0, 100.0, 100.0, RED);
    }
}

pub fn main() {
    unsafe {
        set_canvas_size(1920, 1080);
    }
}

