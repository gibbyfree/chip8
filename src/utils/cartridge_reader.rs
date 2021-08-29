use std::fs::File;
use std::io::{Read, Write};

/* 
* memory: "Most Chip-8 programs start at location 0x200 (512)" and "0xFFF (4095) End of Chip-8 RAM"
*/
pub struct CartridgeReader {
    pub memory: [u8; 3583]
}

impl CartridgeReader {
    pub fn new(file_name: &str) -> Self {
        let mut f = File::open(&file_name).expect("File not found");
        let mut buffer = [0u8; 3583];
        f.read(&mut buffer).expect("Buffer overflow reading file");

        println!("{:x?}", buffer);

        CartridgeReader {
            memory: buffer
        }
    }
}