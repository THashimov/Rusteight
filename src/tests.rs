#[cfg(test)]
mod tests {
    use crate::cpu::CPU;
    use crate::{cpu, rom_loader};

    #[test]
    fn init_cpu() {
        let cpu = CPU::init_cpu();

        assert_eq!(cpu.regs, [0u8; 16]);
        assert_eq!(cpu.ram, [0u8; 4096]);
        assert_eq!(cpu.display, [[0u8; 64]; 32]);
        assert_eq!(cpu.pc, 0);
        assert_eq!(cpu.sp, 0);
        assert_eq!(cpu.stack, []);
        assert_eq!(cpu.delay_timer, 60);
        assert_eq!(cpu.sound_timer, 60);
    }

    #[test]
    fn load_rom() {
        let path = String::from("./src/ROMS/IBM.ch8");
        let rom = rom_loader::load_rom(path);

        let font = rom_loader::FONT;

        for i in 0..font.len() {
            assert_eq!(rom[i + 50], font[i])
        }

        assert_eq!(rom[0x201], 0xE0);
    }

    #[test]
    fn fetch() {
        let mut cpu = cpu::init_test_cpu();
        let hi = cpu.ram[cpu.pc as usize] as u16;
        let lo = cpu.ram[cpu.pc as usize + 1] as u16;
        cpu.pc += 2;
        let inst = hi << 8 | lo;

        assert_eq!(cpu.pc, 0x202);
        assert_eq!(inst, 0x00E0);
    }

    #[test]
    fn execute() {
        let inst = 0x1228 as u16;

        let opcode = ((inst & 0xF000) >> 12) as u8;
        let x = ((inst & 0x0F00) >> 8) as usize;
        let y = ((inst & 0x00F0) >> 4) as usize;
        let n = (inst & 0x000F) as u8;
        let nn = (inst & 0x00FF) as u8;
        let nnn = inst & 0x0FFF;

        assert_eq!(opcode, 0x1);
        assert_eq!(x, 0x2);
        assert_eq!(y, 0x2);
        assert_eq!(n, 0x8);
        assert_eq!(nn, 0x28);
        assert_eq!(nnn, 0x228);
    }

    #[test]
    fn cls() {
        let mut cpu = cpu::init_test_cpu();

        cpu.display[0][0] = 0xFF;

        cpu.display = [[0u8; 64]; 32];

        assert_eq!(cpu.display[0][0], 0);
    }

    #[test]
    fn jmp_to_addr() {
        let mut cpu = cpu::init_test_cpu();
        let addr = 0xFFAE as u16;
        cpu.pc = addr;

        assert_eq!(cpu.pc, 0xFFAE);
    }

    #[test]
    fn set_reg_to_nn() {
        let mut cpu = cpu::init_test_cpu();

        let x = 7;
        let nn = 0xFA;

        cpu.regs[x] = nn;
    }

    #[test]
    fn add_val_to_reg() {
        let mut cpu = cpu::init_test_cpu();
        let x = 7;
        let val = 10;

        cpu.regs[x] = 50;
        cpu.regs[x] = cpu.regs[x].overflowing_add(val).0;

        assert_eq!(cpu.regs[x], 60);

        cpu.regs[x] = cpu.regs[x].overflowing_add(196).0;

        assert_eq!(cpu.regs[x], 0);
    }

    #[test]
    fn set_index_reg_to_addr() {
        let mut cpu = cpu::init_test_cpu();
        let addr = 0xFFE;
        cpu.index_reg = addr;

        assert_eq!(cpu.index_reg, addr);
    }

    #[test]
    fn bitwise() {
        let byte: u8 = 0b11011001;

        for i in 0..8 {
            if (byte >> (7 - i)) & 1 == 1 {
                println!("Bit at {}, is on", i)
            }
        }
    }
}
