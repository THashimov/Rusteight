use crate::rom_loader;

#[derive(Debug)]
pub struct CPU {
    pub regs: [u8; 16],
    pub ram: [u8; 4096],
    pub display: [[u8; 64]; 32],
    pub pc: u16,
    pub sp: usize,
    pub index_reg: u16,
    pub stack: Vec<u16>,
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl CPU {
    pub fn init_cpu() -> CPU {
        let regs = [0u8; 16];
        let ram = [0u8; 4096];
        let pc = 0x200;
        let sp = 0;
        let index_reg = 0;
        let stack = Vec::new();
        let delay_timer = 60;
        let sound_timer = 60;
        let display = [[0u8; 64]; 32];

        CPU {regs, ram, display, pc, sp, index_reg, stack, delay_timer, sound_timer}
    }

    pub fn tick(&mut self) {
        let inst = self.fetch();
        self.pc += 2;
        self.execute(inst);
    }

    fn fetch(&mut self) -> u16 {
        let hi = self.ram[self.pc as usize] as u16;
        let lo = self.ram[self.pc as usize + 1] as u16;

        hi << 8 | lo
    }

    fn execute(&mut self, inst: u16) {
        let opcode = ((inst & 0xF000) >> 12) as u8;
        let x = ((inst & 0x0F00) >> 8) as usize;
        let y = ((inst  & 0x00F0) >> 4) as usize;
        let n = (inst & 0x000F) as u8;
        let nn = (inst & 0x00FF) as u8;
        let addr = inst & 0x0FFF;

        // set reg 0 to 320


        match opcode {
            0x0 => {self.cls(); println!("Clear Screen")},
            0x1 => {self.jmp_to_addr(addr); println!("Jump to address {}", addr)},
            0x6 => {self.set_reg_to_nn(x, nn); println!("Set reg {} to nn {} ", x, nn)},
            0x7 => {self.add_val_to_reg(x, nn); println!("Add val {} to reg {}", nn, x)},
            0xA => {self.set_index_reg_to_addr(addr); println!("Set index reg to addr {}", addr)},
            0xD => {self.draw(x, y, n); println!("Draw x {} y {} n {}", x, y, n)},
            _ => {}
        }
    }


    fn cls(&mut self) {
        self.display = [[0u8; 64]; 32]
    }

    fn jmp_to_addr(&mut self, addr: u16) {
        self.pc = addr;
    }

    fn set_reg_to_nn(&mut self, x: usize, nn: u8) {
        self.regs[x] = nn;
    }

    fn add_val_to_reg(&mut self, x: usize, val: u8) {
        self.regs[x] = self.regs[x].overflowing_add(val).0;
    }

    fn set_index_reg_to_addr(&mut self, addr: u16) {
        self.index_reg = addr;
    }

    fn draw(&mut self, x: usize, y: usize, n: u8) {
        self.regs[0xF] = 0;

        // N is the height of the sprite. Display is 32 high
        // Draw from mem location Index reg
        // X coord = reg[x]
        // Y coord = reg[y]

        let x_coord = self.regs[x] % 64;

        for i in 0..n {
            let y_coord = (self.regs[y] + i) % 32;
            let byte = self.ram[self.index_reg as usize + i as usize];
            for bit in 0..8 {
                let bit_on = (byte >> bit) & 1;
                self.regs[0xF] |= bit_on & self.display[y_coord as usize][x_coord as usize];
            }
            self.display[y_coord as usize][x_coord as usize] ^= byte;
            println!("{:b}", self.display[y_coord as usize][x_coord as usize]);
        }
    }   

}

pub fn init_test_cpu() -> CPU {
    let path = String::from("./src/ROMS/IBM.ch8");
    let mut cpu = CPU::init_cpu();
    cpu.ram = rom_loader::load_rom(path);
    
    cpu
}