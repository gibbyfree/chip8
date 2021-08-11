use crate::consts::{FONTSET, MEMORY_SIZE, DISPLAY_WIDTH, DISPLAY_HEIGHT, STACK_SIZE, REGISTER_SIZE, KEYPAD_SIZE};

/* 
* memory: "The Chip-8 language is capable of accessing up to 4KB (4,096 bytes) of RAM, from location 0x000 (0) to 0xFFF (4095)"
* gfx: "The original implementation of the Chip-8 language used a 64x32-pixel monochrome display"
* stack: "The stack is an array of 16 16-bit values, used to store the address that the interpreter shoud return to when finished with a subroutine."
* sp: "The stack pointer (SP) can be 8-bit, it is used to point to the topmost level of the stack."
* vx: "Chip-8 has 16 general purpose 8-bit registers, usually referred to as Vx, where x is a hexadecimal digit (0 through F)"
* i: "There is also a 16-bit register called I. This register is generally used to store memory addresses,
*    so only the lowest (rightmost) 12 bits are usually used."
* pc: "The program counter (PC) should be 16-bit, and is used to store the currently executing address."
* delay_timer: "Chip-8 also has two special purpose 8-bit registers, for the delay and sound timers."
* sound_timer: ^
* keypad: "The computers which originally used the Chip-8 Language had a 16-key hexadecimal keypad"
*/
pub struct Chip8 {
    memory: [u8; MEMORY_SIZE],
    gfx: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    stack: [u16; STACK_SIZE],
    sp: u16,
    vx: [u8; REGISTER_SIZE],
    i: u16,
    pc: u16,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [u8; KEYPAD_SIZE]
}

impl Chip8 {
    pub fn new() -> Self {
        // Load fontset into memory
        // "The data should be stored in the interpreter area of Chip-8 memory (0x000 to 0x1FF)"
        let mut memory = [0u8; MEMORY_SIZE];
        for n in 0..80 {
            memory[n] = FONTSET[n];
        }

    Chip8 {
        memory,
        gfx: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        stack: [0; STACK_SIZE],
        sp: 0,
        vx: [0; REGISTER_SIZE],
        i: 0,
        pc: 0x200,
        delay_timer: 0,
        sound_timer: 0,
        keypad: [0; KEYPAD_SIZE]
    }
}

    pub fn load_cartridge(&mut self, data: &[u8]) {

    }
}