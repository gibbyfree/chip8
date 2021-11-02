extern crate sdl2;

mod chip8;
mod utils;
mod consts;

use std::{env};
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use chip8::Chip8;
use consts::{DISPLAY_HEIGHT, DISPLAY_SCALE, DISPLAY_WIDTH};
use utils::CartridgeReader;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("CHIP-8", (DISPLAY_WIDTH as u32) * DISPLAY_SCALE, (DISPLAY_HEIGHT as u32) * DISPLAY_SCALE)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0)); // black
    canvas.clear();
    canvas.present();

    let args: Vec<String> = env::args().collect();

    let cartridge_file_name = &args[1];
    let cartridge = CartridgeReader::new(&cartridge_file_name);

    let mut chip8 = Chip8::new();
    chip8.load_cartridge(&cartridge.memory);

    loop {
        chip8.cycle();

        if chip8.needsRedraw() {
            let current_vram = chip8.checkVram();
            for (x, row) in current_vram.iter().enumerate() {
                for (y, &col) in row.iter().enumerate() {
                    let x = (x as u32) * DISPLAY_SCALE;
                    let y = (y as u32) * DISPLAY_SCALE;

                    if col == 0 { // black
                        canvas.set_draw_color(Color::RGB(0, 0, 0));
                    } else { // white
                        canvas.set_draw_color(Color::RGB(255, 255, 255));
                    }

                    let _ = canvas.fill_rect(Rect::new(x as i32, y as i32, DISPLAY_SCALE, DISPLAY_SCALE));
                    canvas.present();
                }
            }
        }
    }
}
