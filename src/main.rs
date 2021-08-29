mod chip8;
mod utils;
mod consts;

use std::env;

use chip8::Chip8;
use utils::CartridgeReader;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cartridge_file_name = &args[1];
    let cartridge = CartridgeReader::new(&cartridge_file_name);

    let mut chip8 = Chip8::new();
}
