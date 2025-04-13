use rand::random;

pub const WINDOW_HEIGHT: usize = 32;
pub const WINDOW_WIDTH: usize = 64;

const RAM_SIZE: usize = 4096;
const V_REGISTERS: usize = 16;
const STACK_SIZE: usize = 16;
const START_ADDR: u16 = 0x200; // 512 bytes offset
const NUM_KEYS: usize = 16;

pub struct CHIP8 {
    pc: u16,  // program counter - stores currently executing address
    ram: [u8; RAM_SIZE], // 4096 (bytes) unsigned 8bit integers as RAM
    screen: [bool; WINDOW_HEIGHT * WINDOW_WIDTH],

    /*
        ref - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.2

        V - registers, general purpose

        VF - register shouldn't be used by any program
    */
    v_reg: [u8; V_REGISTERS], // V0, V1, V3.... V9, VA, VB ..... VF - 16 general purpose registers

    i_reg: u16, // i register to store memory addresses

    // stack
    sp: u16, // stack pointer
    stack: [u16; STACK_SIZE], // the stack is an array of 16 16bit values

    // TIMERS: ref - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.5
    dt: u8, // delay timer
    st: u8, // sound timer

    // KEYBOARD
    keys: [bool; NUM_KEYS]
}

const FONTSET_SIZE: usize = 16 * 5;

/*
    ref - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#keyboard
    for example 1 is represented as: 

    * *  * []  * * * * - 0x20
    * * [] []  * * * * - 0x60
    * *  * []  * * * * - 0x20
    * *  * []  * * * * - 0x20
    * * [] [] [] * * * - 0x70
*/

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

impl CHIP8 {
    // constructor
    pub fn new() -> Self {
        
        let mut new_emu = Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; WINDOW_HEIGHT * WINDOW_WIDTH],
            v_reg: [0; V_REGISTERS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        };

        // LOAD THE SPRITES INTO THE RAM in the address 0x000 to 0x1FF
        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        return  new_emu;
    }

    // push and pop for stack
    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val; // add the value to the stack
        self.sp += 1; // increment the stack pointer
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1; // decrement the stack pointer
        self.stack[self.sp as usize]
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; WINDOW_HEIGHT * WINDOW_WIDTH];
        self.v_reg = [0; V_REGISTERS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick(&mut self) {
        let op = self.fetch();
        self.execute(op);
    }

    // read 2 8 bit values from the ram and return the 16 bit opcode
    fn fetch(&mut self) -> u16 {
        let higher_byte: u16 = self.ram[self.pc as usize] as u16;
        let lower_byte: u16 = self.ram[(self.pc + 1) as usize] as u16;
        let op: u16 = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        return op;
    }

    pub fn tick_timers(&mut self){
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // beep boop
            }
            self.st -= 1;
        }
    }

    // return a pointe to display
    pub fn get_display(&self) -> &[bool] {
        return &self.screen;
    }

    // press ith key
    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        self.keys[idx] = pressed;
    }

    // LOAD DATA FROM THE ROM TO THE EMULATOR RAM
    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDR as usize;
        let end = (START_ADDR as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
    }

    pub fn execute(&mut self, op: u16){

        // ref - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.1

        let digit1 = (op & 0xF000) >> 4 * 3;
        let digit2 = (op & 0x0F00) >> 4 * 2;
        let digit3 = (op & 0x00F0) >> 4 * 1;
        let digit4 = (op & 0x000F) >> 4 * 0;

        match (digit1, digit2, digit3, digit4) {
            (0,0,0,0) => return,

            // 00E0 - CLS
            (0,0,0xE,0) => {
                self.screen = [false; WINDOW_HEIGHT * WINDOW_WIDTH]
            },

            // 00EE - RET
            (0,0,0xE, 0xE) => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
            },

            // 1nnn - set the program counter to `nnn`
            (1, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = nnn;
            },

            // 2nnn - call a subroutine at `nnn` in the stack
            (2,_,_,_) => {
                let nnn = op & 0xFFF;
                self.push(self.pc);
                self.pc = nnn;
            },

            // 3Xnn - skip next opcode if Vx == nn
            (3,_,_,_) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;

                // skip the register if Vk = nn
                if self.v_reg[x] == nn {
                    self.pc += 2;
                }
            },

            // 4Xnn - skip next opcode if Vx != nn
            (4,_,_,_) => {
                let x: usize = digit2 as usize;
                let nn: u8 = (op & 0xFF) as u8;

                // skip the register if Vk = nn
                if self.v_reg[x] != nn {
                    self.pc += 2;
                }
            },

            // 5XY0 - skip if Vx == Vy
            (5,_,_,0) => {
                let x: usize = digit2 as usize;
                let y: usize = digit3 as usize;

                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
            },

            // 6Xnn - set Vx = nn
            (6,_,_,_) => {
                let x: usize = digit2 as usize;
                let nn: u8 = (op & 0xFF) as u8;
                self.v_reg[x] = nn;
            },

            // 7Xnn - Vx += nn
            (7,_,_,_) => {
                let x: usize = digit2 as usize;
                let nn: u8 = (op & 0xFF) as u8;
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
            },

            // 8XY0 - set Vx = Vy
            (8,_,_,0) => {
                let x: usize = digit2 as usize;
                let y: usize = digit3 as usize;
                self.v_reg[x] = self.v_reg[y];
            },

            // 8XY1 - Vx = Vx | Vy
            (8,_,_,1) => {
                let x: usize = digit2 as usize;
                let y: usize = digit3 as usize;

                self.v_reg[x] = self.v_reg[x] | self.v_reg[y];
            },

            // 8XY2 - Vx = Vx & Vy
            (8,_,_,2) => {
                let x: usize = digit2 as usize;
                let y: usize = digit3 as usize;

                self.v_reg[x] = self.v_reg[x] & self.v_reg[y];
            },

            // 8XY3 - Vx = Vx ^ Vy
            (8,_,_,3) => {
                let x: usize = digit2 as usize;
                let y: usize = digit3 as usize;

                self.v_reg[x] = self.v_reg[x] ^ self.v_reg[y];
            },

            // 8XY4 - Vx += Vy and add the carry to VF
            (8,_,_,4) => {
                let x: usize = digit2 as usize;
                let y: usize = digit3 as usize;

                let (vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);

                let new_vf: u8 = if carry { 1 } else { 0 };

                self.v_reg[x] = vx;
                self.v_reg[0xF] = new_vf;
            },

            // 8XY5 - Vx -= Vy and add the carry to VF
            (8,_,_,5) => {
                let x: usize = digit2 as usize;
                let y: usize = digit3 as usize;

                let (vx, carry) = self.v_reg[x].overflowing_sub(self.v_reg[y]);

                let new_vf: u8 = if carry { 0 } else { 1 };

                self.v_reg[x] = vx;
                self.v_reg[0xF] = new_vf;
            },

            // 8XY6 - Vx >>= 1 and capture the shifted bit in VF
            (8,_,_,6) => {
                let x: usize = digit2 as usize;
                
                let lsb = self.v_reg[x] & 1;
                self.v_reg[x] = self.v_reg[x] >> 1;
                self.v_reg[0xF] = lsb;
            },

            // 8XY7 - Vx = Vy - Vx and add the carry to VF
            (8,_,_,7) => {
                let x: usize = digit2 as usize;
                let y: usize = digit3 as usize;

                let (vx, carry) = self.v_reg[y].overflowing_sub(self.v_reg[x]);

                let new_vf: u8 = if carry { 0 } else { 1 };

                self.v_reg[x] = vx;
                self.v_reg[0xF] = new_vf;
            },

            // 8XYE - Vx <<= 1 and save the shifted bit in VF
            (8,_,_,0xE) => {
                let x: usize = digit2 as usize;
                let msb: u8 = (self.v_reg[x] >> 7) & 1;
                self.v_reg[x] = self.v_reg[x] << 1;
                self.v_reg[0xF] = msb;
            },

            // 9XY0 - skip if Vx != Vy
            (9,_,_,0) => {
                let x: usize = digit2 as usize;
                let y: usize = digit3 as usize;
                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2;
                }
            },

            // Annn - set i register to nnn
            (0xA,_,_,_) => {
                let nnn = op & 0xFFF;
                self.i_reg = nnn;
            },

            // Bnnn - jump to addr V0 + nnn
            (0xB,_,_,_) => {
                let nnn: u16 = op & 0xFFF;
                self.pc = (self.v_reg[0] as u16) + nnn;
            },

            // CXnn - set Vx = rand() * nn
            (0xC,_,_,_) => {
                let x: usize = digit2 as usize;
                let nn: u8 = (op & 0xFF) as u8;
                let rng: u8 = random();
                self.v_reg[x] = rng & nn;
            },

            // DXYN - display stride at (x,y) coordinate
            (0xD,_,_,_) => {
                let x_cord: u16 = self.v_reg[digit2 as usize] as u16;
                let y_cord: u16 = self.v_reg[digit3 as usize] as u16;
                let n: usize = digit4 as usize;
                let mut flipped: bool = false;

                for y_line in 0..n {
                    let addr: u16 = self.i_reg + y_line as u16;
                    let pixels: u8 = self.ram[addr as usize];

                    for x_line in 0..8 {
                        if (pixels & (0b1000_0000 >> x_line)) != 0 {
                            let x: usize = (x_cord as usize + x_line) % WINDOW_WIDTH;
                            let y: usize = (y_cord as usize + y_line) % WINDOW_HEIGHT;

                            let idx: usize = x + WINDOW_WIDTH * y;
                            flipped |= self.screen[idx];
                            self.screen[idx] ^= true;
                        }
                    }
                }

                if flipped {
                    self.v_reg[0xF] = 1;
                } else {
                    self.v_reg[0xF] = 0;
                }
            },

            // EX9E - skip if key pressed
            (0xE,_,9,0xE) => {
                let x: usize = digit2 as usize;
                let vx: u8 = self.v_reg[x];
                let key: bool = self.keys[vx as usize];

                if key {
                    self.pc += 2;
                }
            },

            // EXA1 - skip if key NOT pressed
            (0xE, _, 0xA, 1) => {
                let x: usize = digit2 as usize;
                let vx: u8 = self.v_reg[x];
                let key: bool = self.keys[vx as usize];

                if !key {
                    self.pc += 2;
                }
            },

            // FX07 - set Vx = display timer (dt)
            (0xF, _, 0, 7) => {
                let x: usize = digit2 as usize;
                self.v_reg[x] = self.dt;
            },

            // FX0A - wait for keypress
            (0xF, _, 0, 0xA) => {
                let x: usize = digit2 as usize;
                let mut pressed: bool = false;

                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_reg[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    // blocking operating keep doing until keypress
                    self.pc -= 2;
                }
            },

            // FX15 - set display timer (dt) = Vx
            (0xF, _, 1, 5) => {
                let x: usize = digit2 as usize;
                self.dt = self.v_reg[x];
            },

            // FX18 - set sound timmer (st) = Vx
            (0xF, _, 1, 8) => {
                let x: usize = digit2 as usize;
                self.st = self.v_reg[x];
            },

            // FX1E - I register += Vx
            (0xF, _, 1, 0xE) => {
                let x: usize = digit2 as usize;
                let vx: u16 = self.v_reg[x] as u16;
                self.i_reg = self.i_reg.wrapping_add(vx);
            },

            // FX29 - set I register = location of sprite for digit Vx
            (0xF, _, 2, 9) => {
                let x: usize = digit2 as usize;
                let c: u16 = self.v_reg[x] as u16;
                self.i_reg = c * 5;
            },

            // FX33 - binary coded decimal of Vx
            (0xF, _, 3, 3) => {
                let x: usize = digit2 as usize;
                let vx: f32 = self.v_reg[x] as f32;

                let hundreds: u8 = (vx / 100.0).floor() as u8;
                let tens: u8 = ((vx / 10.0) % 10.0).floor() as u8;
                let ones: u8 = (vx % 10.0) as u8;

                self.ram[self.i_reg as usize] = hundreds;
                self.ram[(self.i_reg + 1) as usize] = tens;
                self.ram[(self.i_reg + 2) as usize] = ones;
            },

            // FX55 - write V0 ----- Vx in the ram
            (0xF, _, 5, 5) => {
                let x: usize = digit2 as usize;
                let i: usize = self.i_reg as usize;

                for idx in 0..x {
                    self.ram[i + idx] = self.v_reg[idx];
                }
            },

            // FX65 - read V0 ---- Vx from the memory
            (0xF, _, 6, 5) => {
                let x: usize = digit2 as usize;
                let i: usize = self.i_reg as usize;
                for idx in 0..=x {
                    self.v_reg[idx] = self.ram[i + idx];
                }
            },

            // YAYAYAYAY

            (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", op),
        }
    }

}