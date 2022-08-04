use rand::Rng;

use crate::{keypad::KeyStroke, rom_loader};

#[derive(Debug)]
pub struct CPU {
    pub regs: [u8; 16],
    pub ram: [u8; 4096],
    pub display: [[u8; 64]; 32],
    pub pc: u16,
    pub sp: usize,
    pub index_reg: u16,
    pub stack: [u16; 16],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub key_pressed: [u8; 16],
}

impl CPU {
    pub fn init_cpu() -> CPU {
        let regs = [0u8; 16];
        let ram = [0u8; 4096];
        let pc = 0x200;
        let sp = 0;
        let index_reg = 0;
        let stack = [0u16; 16];
        let delay_timer = 60;
        let sound_timer = 60;
        let display = [[0u8; 64]; 32];
        let key_pressed = [0u8; 16];

        CPU {
            regs,
            ram,
            display,
            pc,
            sp,
            index_reg,
            stack,
            delay_timer,
            sound_timer,
            key_pressed,
        }
    }

    pub fn tick(&mut self) {
        let inst = self.fetch();
        self.pc += 2;
        self.execute(inst);
    }

    pub fn set_key(&mut self, key: &KeyStroke) {
        match key {
            KeyStroke::Quit => {}
            KeyStroke::Key(k) => self.key_pressed = *k,
            KeyStroke::Next => {}
        }
    }

    fn fetch(&mut self) -> u16 {
        let hi = self.ram[self.pc as usize] as u16;
        let lo = self.ram[self.pc as usize + 1] as u16;

        hi << 8 | lo
    }

    fn execute(&mut self, inst: u16) {
        let opcode = ((inst & 0xF000) >> 12) as u8;
        let x = ((inst & 0x0F00) >> 8) as usize;
        let y = ((inst & 0x00F0) >> 4) as usize;
        let n = (inst & 0x000F) as u8;
        let nn = (inst & 0x00FF) as u8;
        let addr = inst & 0x0FFF;

        // set reg 0 to 320

        match opcode {
            0x0 => match n {
                0x0 => {
                    self.cls();
                    println!("Clear Screen");
                }
                0xE => {
                    self.ret();
                    println!("Return from subroutine");
                }
                _ => {}
            },
            0x1 => {
                self.jmp_to_addr(addr);
                println!("Jump to {:x}", addr);
            }
            0x2 => {
                self.call_addr(addr);
                println!("Call subroutine at {:x}", addr);
            }
            0x3 => {
                self.se_byte(x, nn);
                println!("Skip if reg {} is equal to {}", x, nn);
            }
            0x4 => {
                self.sne_byte(x, nn);
                println!("Skip if reg {} is not equal to {}", x, nn);
            }
            0x5 => {
                self.se_reg_reg(x, y);
                println!("Skip if reg {} is not equal to reg {}", x, y);
            }
            0x6 => {
                self.set_reg_to_nn(x, nn);
                println!("Set reg {} to nn {} ", x, nn);
            }
            0x7 => {
                self.add_val_to_reg(x, nn);
                println!("Add val {} to reg {}", nn, x);
            }
            0x8 => match n {
                0x0 => {
                    self.ld_reg_reg(x, y);
                    println!("Set reg {} to reg {}", x, y);
                }
                0x1 => {
                    self.bit_or(x, y);
                    println!("Bitwise OR {} to {}", x, y);
                }
                0x2 => {
                    self.bit_and(x, y);
                    println!("Bitwise AND {} to {}", x, y,);
                }
                0x3 => {
                    self.bit_xor(x, y);
                    println!("Bitwise XOR {} to {}", x, y);
                }
                0x4 => {
                    self.add_reg_reg(x, y);
                    println!("Add reg {} to reg {}", y, x);
                }
                0x5 => {
                    self.sub_reg_reg(x, y);
                    println!("Sub reg {} from {}", y, x);
                }
                0x6 => {
                    self.shr(x);
                    println!("Shift reg {} right by one", x);
                }
                0x7 => {
                    self.sub_not_borrow(x, y);
                    println!("Sub not borrow {} from {}", y, x);
                }
                0xE => {
                    self.shl(x);
                    println!("Shift reg {} left by one", x);
                }
                _ => {}
            },
            0x9 => {
                self.sne_reg_reg(x, y);
                println!("Skip if reg {} != {}", x, y);
            }
            0xA => {
                self.set_index_reg_to_addr(addr);
                println!("Set index reg to addr {}", addr)
            }
            0xB => {
                self.jmp_to_addr_reg_0(addr);
                println!("Jump to addr {} + reg 0", addr)
            }
            0xC => {
                self.rnd_num(x, nn);
                println!("Set reg {} to random number & {}", x, nn);
            }
            0xD => {
                self.draw(x, y, n);
                println!("Draw x {} y {} n {}", x, y, n)
            }
            0xE => match n {
                0xE => {
                    self.skp(x);
                    println!("Skip if key with value at reg {} is pressed", x)
                }
                0x1 => {
                    self.sknp(x);
                    println!("Skip if key with value at reg {} is not pressed", x)
                }
                _ => {}
            },
            0xF => match nn {
                0x07 => {
                    self.ld_dt_to_reg(x);
                    println!("Load value of delay timer into reg {}", x);
                }
                0x0A => {
                    self.ld_key(x);
                    println!("Wait for key press and store value in reg {}", x);
                }
                0x15 => {
                    self.ld_reg_to_dt(x);
                    println!("Load reg {} to delay timer", x);
                }
                0x18 => {
                    self.ld_st_to_reg(x);
                    println!("Load sound timer to reg {}", x);
                }
                0x1E => { 
                    self.add_i_to_reg(x);
                    println!("Index reg = index reg + reg {}", x);
                }
                0x29 => {
                    self.ld_font(x);
                    println!("load font at location reg {} to index reg", x);
                }
                0x33 => { 
                    self.bcd(x);
                    println!("Store value of reg {} as bcd in index reg", x);
                }
                0x55 => { 
                    self.ld_reg_to_ram(x);
                    println!("Store values of reg 0 to reg {} in ram", x);
                }
                0x65 => { 
                    self.ld_ram_to_reg(x);
                    println!("Load ram into reg 0 to reg {}", x);
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn cls(&mut self) {
        self.display = [[0u8; 64]; 32];
    }

    fn ret(&mut self) {
        self.pc = self.stack[self.sp];
        if self.sp > 0 {
            self.sp -= 1;
        };
    }

    fn jmp_to_addr(&mut self, addr: u16) {
        self.pc = addr;
    }

    fn call_addr(&mut self, addr: u16) {
        self.sp += 1;
        self.stack[self.sp] = self.pc;
        self.pc = addr;
    }

    fn se_byte(&mut self, x: usize, nn: u8) {
        if self.regs[x] == nn {
            self.pc += 2;
        }
    }

    fn sne_byte(&mut self, x: usize, nn: u8) {
        if self.regs[x] != nn {
            self.pc += 2;
        }
    }

    fn se_reg_reg(&mut self, x: usize, y: usize) {
        if self.regs[x] == self.regs[y] {
            self.pc += 2;
        }
    }

    fn set_reg_to_nn(&mut self, x: usize, nn: u8) {
        self.regs[x] = nn;
    }

    fn add_val_to_reg(&mut self, x: usize, val: u8) {
        self.regs[x] = self.regs[x].overflowing_add(val).0;
    }

    fn ld_reg_reg(&mut self, x: usize, y: usize) {
        self.regs[x] = self.regs[y];
    }

    fn bit_or(&mut self, x: usize, y: usize) {
        self.regs[x] |= self.regs[y];
    }

    fn bit_and(&mut self, x: usize, y: usize) {
        self.regs[x] &= self.regs[y];
    }

    fn bit_xor(&mut self, x: usize, y: usize) {
        self.regs[x] ^= self.regs[y];
    }

    fn add_reg_reg(&mut self, x: usize, y: usize) {
        let overflow = self.regs[x].overflowing_add(self.regs[y]);

        if overflow.1 {
            self.regs[0xF] = 1;
        };

        self.regs[x] = overflow.0;
    }

    fn sub_reg_reg(&mut self, x: usize, y: usize) {
        if self.regs[x] > self.regs[y] {
            self.regs[0xF] = 1;
        }

        self.regs[x] = self.regs[x].overflowing_sub(self.regs[y]).0;
    }

    fn shr(&mut self, x: usize) {
        if self.regs[x] & 1 == 1 {
            self.regs[0xF] = 1;
        };

        self.regs[x] >>= 1;
    }

    fn sub_not_borrow(&mut self, x: usize, y: usize) {
        if self.regs[y] > self.regs[x] {
            self.regs[0xF] = 1;
        }

        self.regs[x] = self.regs[x].overflowing_sub(self.regs[y]).0;
    }

    fn shl(&mut self, x: usize) {
        if self.regs[x] >> 7 & 1 == 1 {
            self.regs[0xF] = 1;
        }

        self.regs[x] <<= 1;
    }

    fn sne_reg_reg(&mut self, x: usize, y: usize) {
        if self.regs[x] != self.regs[y] {
            self.pc += 2;
        }
    }

    fn set_index_reg_to_addr(&mut self, addr: u16) {
        self.index_reg = addr;
    }

    fn jmp_to_addr_reg_0(&mut self, addr: u16) {
        self.pc = self.regs[0] as u16 + addr;
    }

    fn rnd_num(&mut self, x: usize, nn: u8) {
        let mut rng = rand::thread_rng();
        let rnd_num = rng.gen_range(0..255);

        self.regs[x] = rnd_num & nn;
    }

    fn draw(&mut self, x: usize, y: usize, n: u8) {
        self.regs[0xF] = 0;

        // N is the height of the sprite. Display is 32 high
        // Draw from mem location Index reg
        // X coord = reg[x]
        // Y coord = reg[y]

        for i in 0..n {
            let y_coord = (self.regs[y] + i) % 32;
            let byte = self.ram[(self.index_reg as usize) + i as usize];
            for bit in 0..8 {
                let x_coord = (self.regs[x] + bit) %64;
                let byte_at_disp = self.display[y_coord as usize][x_coord as usize];
                let pixel_to_turn_on = (byte >> (7 - bit)) & 1;
                let current_pixel_status = (byte_at_disp >> bit) & 1;
                if pixel_to_turn_on == 1 && current_pixel_status == 1 {
                    self.regs[0xF] = 1;
                }
                self.display[y_coord as usize][x_coord as usize] ^= pixel_to_turn_on;
            }
        }
    }

    fn skp(&mut self, x: usize) {
        if self.key_pressed[self.regs[x] as usize] == 1 {
            self.pc += 2;
        }
    }

    fn sknp(&mut self, x: usize) {
        if self.key_pressed[self.regs[x] as usize] != 1 {
            self.pc += 2;
        }
    }

    fn ld_dt_to_reg(&mut self, x: usize) {
        self.regs[x] = self.delay_timer;
    }

    fn ld_key(&mut self, x: usize) {
        if self.key_pressed == [0u8; 16] {
            self.pc -= 2;
        } else {
            for i in 0..self.key_pressed.len() {
                if self.key_pressed[i] == 1 {
                    self.regs[x] = i as u8;
                }
            }
        }
    }

    fn ld_reg_to_dt(&mut self, x: usize) {
        self.delay_timer = self.regs[x];
    }

    fn ld_st_to_reg(&mut self, x: usize) {
        self.sound_timer = self.regs[x];
    }

    fn add_i_to_reg(&mut self, x: usize) {
        self.index_reg += self.regs[x] as u16;
    }

    fn ld_font(&mut self, x: usize,) {
        self.index_reg = self.regs[x] as u16 * 5;
    }

    fn bcd(&mut self, x: usize) {
        self.ram[self.index_reg as usize] = self.regs[x] / 100;
        self.ram[self.index_reg as usize + 1] = (self.regs[x] % 100) / 10;
        self.ram[self.index_reg as usize + 2] = self.regs[x] % 10;
    }

    fn ld_reg_to_ram(&mut self, x: usize) {
        for i in 0..x {
            self.ram[self.index_reg as usize + i] = self.regs[i];
        }
    }

    fn ld_ram_to_reg(&mut self, x: usize) {
        for i in 0..x {
            self.regs[i] = self.ram[self.index_reg as usize + i];
        }
    }

}

pub fn init_test_cpu() -> CPU {
    let path = String::from("./src/ROMS/IBM.ch8");
    let mut cpu = CPU::init_cpu();
    cpu.ram = rom_loader::load_rom(path);

    cpu
}
