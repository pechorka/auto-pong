const std = @import("std");

const WIDTH = 800;
const HEIGHT = 600;
const BYTES_PER_PIXEL = 4;
const SCREEN_BUFFER_SIZE = WIDTH * HEIGHT * BYTES_PER_PIXEL; // RGBA

// Global state for animation
var x: i32 = 100;
var y: i32 = 100;
var dx: i32 = 3;
var dy: i32 = 3;

// Exported memory for the pixel buffer
export var screen_buffer: [SCREEN_BUFFER_SIZE]u8 = undefined;

export fn update() void {
    // Clear screen (black background)
    @memset(&screen_buffer, 0);

    // Simple bouncing rectangle simulation
    x += dx;
    y += dy;

    // Bounce off walls
    if (x <= 0 or x + 200 >= WIDTH) dx = -dx;
    if (y <= 0 or y + 50 >= HEIGHT) dy = -dy;

    // Draw text
    const text = "Hello World!";
    for (text, 0..) |c, i| {
        draw_char(x + @as(i32, @intCast(i * 10)), y, c, 0xFF_FF_FF_FF);
    }
}

fn draw_char(px: i32, py: i32, c: u8, color: u32) void {
    // Simple placeholder "font":
    // Each character is an 8x8 white block if c is not a space.
    if (c == ' ') return;

    for (0..8) |row| {
        for (0..8) |col| {
            const nx = px + @as(i32, @intCast(col));
            const ny = py + @as(i32, @intCast(row));
            if (nx < 0 or nx >= WIDTH or ny < 0 or ny >= HEIGHT) continue;

            const idx = @as(usize, @intCast(ny)) * WIDTH + @as(usize, @intCast(nx)) * BYTES_PER_PIXEL;
            screen_buffer[idx] = @as(u8, @intCast((color >> 24) & 0xFF));
            screen_buffer[idx + 1] = @as(u8, @intCast((color >> 16) & 0xFF));
            screen_buffer[idx + 2] = @as(u8, @intCast((color >> 8) & 0xFF));
            screen_buffer[idx + 3] = @as(u8, @intCast(color & 0xFF));
        }
    }
}

pub fn main() void {}
