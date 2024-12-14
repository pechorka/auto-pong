#[no_mangle]
extern "C" {
    fn set_canvas_size(width: u32, height: u32);
    fn fill_rect(x: f32, y: f32, w: f32, h: f32, color: u32);
    fn set_update_frame(f: fn(f32));
    fn clear_background(color: u32);
}

fn update_frame(dt: f32) {
    unsafe {
        clear_background(0x0000FFFF);
        fill_rect(0.0, 0.0, 100.0, 100.0, 0xFF0000FF);
    }
}

pub fn main() {
    unsafe {
        set_canvas_size(800, 600);
        set_update_frame(update_frame);
    }
}

