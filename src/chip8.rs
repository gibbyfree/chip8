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
* redraw: I added this to mark whether or not it's necessary to redraw the display.
*/
pub struct Chip8 {
    memory: [u8; MEMORY_SIZE],
    gfx: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    stack: [u16; STACK_SIZE],
    sp: u16,
    vx: [u8; REGISTER_SIZE],
    i: u16,
    pc: usize,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [u8; KEYPAD_SIZE],
    redraw: bool
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
        keypad: [0; KEYPAD_SIZE],
        redraw: false
    }
}

    /*
    * Load the program into memory.
    * "Most Chip-8 programs start at location 0x200 (512)"
    */
    pub fn load_cartridge(&mut self, data: &Vec<u8>) {
        for (i,&item) in data.iter().enumerate() {
            let address = 0x200 + i;
            self.memory[address] = item;
        }
    }

    pub fn cycle(&mut self) {
        let opcode = self.fetch();
        self.decode(opcode);
        println!("{:#04X?}", opcode);
    }

    pub fn needsRedraw(&mut self) -> bool {
        return self.redraw;
    }

    pub fn checkVram(&mut self) -> [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT] {
        return self.gfx;
    }

    fn fetch(&self) -> u16 {
        return (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
    }

    fn decode(&mut self, opcode: u16) {
        // split opcode into nibbles
        let nibbles = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            (opcode & 0x000F)
        );

        let nnn = opcode & 0x0FFF;
        let nn = opcode & 0x00FF;
        let n = opcode & 0x000F;
        let x = (opcode & 0xF000) >> 8;
        let y = (opcode & 0x00F0) >> 4;

        match nibbles {
            (0x0a, _, _, _) => self.op_annn(nnn),
            (0x0b, _, _, _) => self.op_bnnn(nnn),
            (0x01, _, _, _) => self.op_1nnn(nnn),
            (0x02, _, _, _) => self.op_2nnn(nnn),
            (0x03, _, _, _) => self.op_3xnn(x, nn),
            (0x04, _, _, _) => self.op_4xnn(x, nn),
            (0x05, _, _, 0x00) => self.op_5xy0(x, y),
            (0x06, _, _, _) => self.op_6xnn(x, nn),
            (0x07, _, _, _) => self.op_7xnn(x, nn),
            (0x0c, _, _, _) => self.op_cxnn(x, nn),
            (0x08, _, _, 0x00) => self.op_8xy0(x, y),
            (0x08, _, _, 0x01) => self.op_8xy1(x, y),
            (0x08, _, _, 0x02) => self.op_8xy2(x, y),
            (0x08, _, _, 0x03) => self.op_8xy3(x, y),
            (0x08, _, _, 0x04) => self.op_8xy4(x, y),
            (0x08, _, _, 0x05) => self.op_8xy5(x, y),
            (0x08, _, _, 0x06) => self.op_8xy6(x, y),
            (0x08, _, _, 0x07) => self.op_8xy7(x, y),
            (0x08, _, _, 0x0e) => self.op_8xye(x, y),
            (0x09, _, _, 0x00) => self.op_9xy0(x, y),
            (0x0d, _, _, _) => self.op_dxyn(x, y, n),
            (0x0e, _, 0x09, 0x0e) => self.op_ex9e(x),
            (0x0e, _, 0x0a, 0x01) => self.op_exa1(x),
            (0x0f, _, 0x00, 0x07) => self.op_fx07(x),
            (0x0f, _, 0x00, 0x0a) => self.op_fx0a(x),
            (0x0f, _, 0x01, 0x05) => self.op_fx15(x),
            (0x0f, _, 0x01, 0x08) => self.op_fx18(x),
            (0x0f, _, 0x01, 0x0e) => self.op_fx1e(x),
            (0x0f, _, 0x02, 0x09) => self.op_fx29(x),
            (0x0f, _, 0x03, 0x03) => self.op_fx33(x),
            (0x0f, _, 0x05, 0x05) => self.op_fx55(x),
            (0x0f, _, 0x06, 0x05) => self.op_fx65(x),
            (0x00, 0x00, 0x0e, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0e, 0x0e) => self.op_00ee(),
            _ => println!("oops")
        }
    }

    /*
    * OPCODES
    */

    // "Sets I to the address NNN."
    fn op_annn(&mut self, nnn: u16) {
        self.i = nnn;
    }

    // "Jumps to the address NNN plus V0."
    fn op_bnnn(&mut self, nnn: u16) {
        // not implemented
    }

    // "Jumps to address NNN."
    fn op_1nnn(&mut self, nnn: u16) {
        self.pc = nnn as usize;
    }

    // "Calls subroutine at NNN."
    fn op_2nnn(&mut self, nnn: u16) {
        self.stack[(self.sp as usize)] = self.pc as u16; // save current pc
        self.sp += 1;
        self.pc = nnn as usize;
    }

    // "Skips the next instruction if VX equals NN. (Usually the next instruction is a jump to skip a code block)"
    fn op_3xnn(&mut self, x: u16, nn: u16) {
        // not implemented
    }

    // "Skips the next instruction if VX does not equal NN. (Usually the next instruction is a jump to skip a code block)"
    fn op_4xnn(&mut self, x: u16, nn: u16) {
        // not implemented
    }

    // "Skips the next instruction if VX equals VY. (Usually the next instruction is a jump to skip a code block)"
    fn op_5xy0(&mut self, x: u16, y: u16) {
        // not implemented
    }

    // "Sets VX to NN."
    fn op_6xnn(&mut self, x: u16, nn: u16) {
       self.vx[x as usize] = nn as u8;
    }

    // "Adds NN to VX. (Carry flag is not changed)"
    fn op_7xnn(&mut self, x: u16, nn: u16) {
        self.vx[x as usize] += nn as u8;
    }

    // "Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN."
    fn op_cxnn(&mut self, x: u16, nn: u16) {
        // not implemented
    }

    // "Sets VX to the value of VY."
    fn op_8xy0(&mut self, x: u16, y: u16) {
        // not implemented
    }

    // "Sets VX to VX or VY. (Bitwise OR operation)"
    fn op_8xy1(&mut self, x: u16, y: u16) {
        // not implemented
    }

    // "Sets VX to VX and VY. (Bitwise AND operation)"
    fn op_8xy2(&mut self, x: u16, y: u16) {
        // not implemented
    }

    // "Sets VX to VX xor VY."
    fn op_8xy3(&mut self, x: u16, y: u16) {
        // not implemented
    }

    // "Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not."
    fn op_8xy4(&mut self, x: u16, y: u16) {
        // not implemented
    }

    // "VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not."
    fn op_8xy5(&mut self, x: u16, y: u16) {
        // not implemented
    }

    // "Stores the least significant bit of VX in VF and then shifts VX to the right by 1."
    fn op_8xy6(&mut self, x: u16, y: u16) {
        // not implemented
    }

    // "Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not."
    fn op_8xy7(&mut self, x: u16, y: u16) {
        // not implemented
    }

    // "Stores the most significant bit of VX in VF and then shifts VX to the left by 1."
    fn op_8xye(&mut self, x: u16, y: u16) {
        // not implemented
    }
    
    // "Skips the next instruction if VX does not equal VY. (Usually the next instruction is a jump to skip a code block)"
    fn op_9xy0(&mut self, x: u16, y: u16) {
        // not implemented
    }

    // "Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. 
    // Each row of 8 pixels is read as bit-coded starting from memory location I; 
    // I value does not change after the execution of this instruction. 
    // As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that does not happen"
    fn op_dxyn(&mut self, x: u16, y: u16, n: u16) {
        self.vx[0x0f] = 0;
        for row in 0..n { 
            let y = (self.vx[y as usize] as usize + (row as usize)) % 32;
            for pixel in 0..8 {
                let x = (self.vx[x as usize] as usize + pixel) % 64;
                let value = (self.memory[(self.i + row) as usize] >> (7 - pixel)) & 1;
                self.vx[0x0f] |= value & self.gfx[y][x];
                self.gfx[y][x]^= value;
            }
        }
        self.redraw = true;
        self.pc += 2;
    }

    // "Skips the next instruction if the key stored in VX is pressed. (Usually the next instruction is a jump to skip a code block)"
    fn op_ex9e(&mut self, x: u16) {
        // not implemented
    }

    // "Skips the next instruction if the key stored in VX is not pressed. (Usually the next instruction is a jump to skip a code block)"
    fn op_exa1(&mut self, x: u16) {
        // not implemented
    }

    // "Sets VX to the value of the delay timer."
    fn op_fx07(&mut self, x: u16) {
        // not implemented
    }

    // "A key press is awaited, and then stored in VX. (Blocking Operation. All instruction halted until next key event)"
    fn op_fx0a(&mut self, x: u16) {
        // not implemented
    }

    // "Sets the delay timer to VX."
    fn op_fx15(&mut self, x: u16) {
        // not implemented
    }

    // "Sets the sound timer to VX."
    fn op_fx18(&mut self, x: u16) {
        // not implemented
    }

    // "Adds VX to I. VF is not affected."
    fn op_fx1e(&mut self, x: u16) {
        // not implemented()
    }

    // "Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font."
    fn op_fx29(&mut self, x: u16) {
        // not implemented
    }

    // "Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, 
    // and the least significant digit at I plus 2. (In other words, take the decimal representation of VX, place the hundreds digit in memory at location in I, 
    // the tens digit at location I+1, and the ones digit at location I+2.)"
    fn op_fx33(&mut self, x: u16) {
        // not implemented
    }

    // "Stores V0 to VX (including VX) in memory starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified."
    fn op_fx55(&mut self, x: u16) {
        // not implemented
    }

    // "Fills V0 to VX (including VX) with values from memory starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified."
    fn op_fx65(&mut self, x: u16) {
        // not implemented
    }

    // "Clears the screen."
    fn op_00e0(&mut self) {
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                self.gfx[y][x] = 0;
            }
        }
        self.redraw = true;
        self.pc += 1;
    }
    
    // "Returns from a subroutine."
    // "The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer."
    fn op_00ee(&mut self) {
        self.pc = self.stack[(self.sp as usize)] as usize;
        self.sp -= 1;
    }
}