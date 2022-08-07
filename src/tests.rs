#[cfg(test)]
mod tests {
    use crate::cpu::CPU;
    use crate::keypad::{check_for_key_press, KeyStroke};
    use crate::window_manager::WindowManager;
    use crate::{cpu, rom_loader};

    #[test]
    fn init_cpu() {
        let cpu = CPU::init_cpu();

        assert_eq!(cpu.regs, [0u8; 16]);
        assert_eq!(cpu.ram, [0u8; 4096]);
        assert_eq!(cpu.display, [[0u8; 64]; 32]);
        assert_eq!(cpu.pc, 0x200);
        assert_eq!(cpu.sp, 0);
        assert_eq!(cpu.stack, [0; 16]);
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
    fn ret() {
        let mut cpu = cpu::init_test_cpu();

        cpu.sp = 10;
        cpu.stack[cpu.sp] = 0xAAAA;

        cpu.pc = cpu.stack[cpu.sp as usize] as u16;
        if cpu.sp > 0 {
            cpu.sp -= 1;
        }
        assert_eq!(cpu.pc, 0xAAAA);
        assert_eq!(cpu.sp, 9);
    }

    #[test]
    fn call_addr() {
        let mut cpu = cpu::init_test_cpu();

        let addr = 0xFABB;

        cpu.sp += 1;
        cpu.stack[cpu.sp] = cpu.pc;
        cpu.pc = addr;

        assert_eq!(cpu.stack[1], 0x200);
        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0xFABB);
    }

    #[test]
    fn se_byte() {
        let mut cpu = cpu::init_test_cpu();

        let x = 5;
        let nn = 20;

        if cpu.regs[x] == nn {
            cpu.pc += 2;
        }

        assert_eq!(cpu.pc, 0x200);

        cpu.regs[5] = 20;

        if cpu.regs[x] == nn {
            cpu.pc += 2;
        }

        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn sne_byte() {
        let mut cpu = cpu::init_test_cpu();

        let x = 5;
        let nn = 20;

        if cpu.regs[x] != nn {
            cpu.pc += 2;
        }

        assert_eq!(cpu.pc, 0x202);

        cpu.regs[5] = 20;

        if cpu.regs[x] != nn {
            cpu.pc += 2;
        }

        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn se_reg_reg() {
        let mut cpu = cpu::init_test_cpu();

        let x = 5;
        let y = 8;

        if cpu.regs[x] == cpu.regs[y] {
            cpu.pc += 2;
        }

        assert_eq!(cpu.pc, 0x202);

        cpu.regs[5] = 20;

        if cpu.regs[x] == cpu.regs[y] {
            cpu.pc += 2;
        }

        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn ld_reg_reg() {
        let mut cpu = cpu::init_test_cpu();

        // Reg x = reg y

        let x = 5;
        let y = 8;

        cpu.regs[y] = 20;

        cpu.regs[x] = cpu.regs[y];

        assert_eq!(cpu.regs[x], 20);
    }

    #[test]
    fn bit_or() {
        let mut cpu = cpu::init_test_cpu();

        let x = 5;
        let y = 8;

        cpu.regs[x] = 0b11011001;
        cpu.regs[y] = 0b11111111;

        cpu.regs[x] |= cpu.regs[y];

        assert_eq!(cpu.regs[x], 0b11111111);
    }

    #[test]
    fn bit_and() {
        let mut cpu = cpu::init_test_cpu();

        let x = 5;
        let y = 8;

        cpu.regs[x] = 0b11011001;
        cpu.regs[y] = 0b11111111;

        cpu.regs[x] &= cpu.regs[y];

        assert_eq!(cpu.regs[x], 0b11011001);
    }

    #[test]
    fn bit_xor() {
        let mut cpu = cpu::init_test_cpu();

        let x = 5;
        let y = 8;

        cpu.regs[x] = 0b11011001;
        cpu.regs[y] = 0b11111111;

        cpu.regs[x] ^= cpu.regs[y];

        assert_eq!(cpu.regs[x], 0b00100110);
    }

    #[test]
    fn add_reg_reg() {
        let mut cpu = cpu::init_test_cpu();

        let x = 5;
        let y = 8;

        cpu.regs[x] = 20;
        cpu.regs[y] = 20;

        let overflow = cpu.regs[x].overflowing_add(cpu.regs[y]);

        if overflow.1 {
            cpu.regs[0xF] = 1;
        };

        cpu.regs[x] = overflow.0;

        assert_eq!(cpu.regs[x], 40);
        assert_eq!(cpu.regs[0xF], 0);

        cpu.regs[x] = 2;
        cpu.regs[y] = 255;

        let overflow = cpu.regs[x].overflowing_add(cpu.regs[y]);

        if overflow.1 {
            cpu.regs[0xF] = 1;
        };

        cpu.regs[x] = overflow.0;

        assert_eq!(cpu.regs[x], 1);
        assert_eq!(cpu.regs[0xF], 1);
    }

    #[test]
    fn sub_reg_reg() {
        let mut cpu = cpu::init_test_cpu();

        let x = 5;
        let y = 8;

        cpu.regs[x] = 0;
        cpu.regs[y] = 1;

        if cpu.regs[x] > cpu.regs[y] {
            cpu.regs[0xF] = 1;
        }

        cpu.regs[x] = cpu.regs[x].overflowing_sub(cpu.regs[y]).0;

        assert_eq!(cpu.regs[x], 255);
        assert_eq!(cpu.regs[0xF], 0);

        cpu.regs[x] = 2;
        cpu.regs[y] = 1;

        if cpu.regs[x] > cpu.regs[y] {
            cpu.regs[0xF] = 1;
        }

        cpu.regs[x] = cpu.regs[x].overflowing_sub(cpu.regs[y]).0;

        assert_eq!(cpu.regs[x], 1);
        assert_eq!(cpu.regs[0xF], 1);
    }

    #[test]
    fn shr() {
        let mut cpu = cpu::init_test_cpu();

        let x = 5;
        cpu.regs[x] = 0b11100101;

        if cpu.regs[x] & 1 == 1 {
            cpu.regs[0xF] = 1;
        };

        cpu.regs[x] >>= 1;

        assert_eq!(cpu.regs[x], 0b01110010);
        assert_eq!(cpu.regs[0xF], 1);

        cpu.regs[x] = 0b11100100;

        if cpu.regs[x] & 1 == 1 {
            cpu.regs[0xF] = 1;
        };

        cpu.regs[x] >>= 1;

        assert_eq!(cpu.regs[x], 0b01110010);
        assert_eq!(cpu.regs[0xF], 1);
    }

    #[test]
    fn sub_not_borrow() {
        let mut cpu = cpu::init_test_cpu();

        let x = 5;
        let y = 8;

        cpu.regs[x] = 0;
        cpu.regs[y] = 1;

        if cpu.regs[y] > cpu.regs[x] {
            cpu.regs[0xF] = 1;
        }

        cpu.regs[x] = cpu.regs[x].overflowing_sub(cpu.regs[y]).0;

        assert_eq!(cpu.regs[x], 255);
        assert_eq!(cpu.regs[0xF], 1);

        cpu.regs[0xF] = 0;
        cpu.regs[x] = 2;
        cpu.regs[y] = 1;

        if cpu.regs[y] > cpu.regs[x] {
            cpu.regs[0xF] = 1;
        }

        cpu.regs[x] = cpu.regs[x].overflowing_sub(cpu.regs[y]).0;

        assert_eq!(cpu.regs[x], 1);
        assert_eq!(cpu.regs[0xF], 0);
    }

    #[test]
    fn shl() {
        let mut cpu = cpu::init_test_cpu();

        let x = 5;

        cpu.regs[x] = 0b11011001;

        if cpu.regs[x] >> 7 & 1 == 1 {
            cpu.regs[0xF] = 1;
        }

        cpu.regs[x] <<= 1;

        assert_eq!(cpu.regs[x], 0b10110010);
        assert_eq!(cpu.regs[0xF], 1);

        cpu.regs[0xF] = 0;

        cpu.regs[x] = 0b01001101;

        if cpu.regs[x] >> 7 & 1 == 1 {
            cpu.regs[0xF] = 1;
        }

        assert_eq!(cpu.regs[0xF], 0);
    }

    #[test]
    fn sne_reg_reg() {
        let mut cpu = cpu::init_test_cpu();

        let x = 5;
        let y = 8;

        if cpu.regs[x] != cpu.regs[y] {
            cpu.pc += 2;
        };

        assert_eq!(cpu.pc, 0x200);

        cpu.regs[x] = 10;

        if cpu.regs[x] != cpu.regs[y] {
            cpu.pc += 2;
        };

        assert_eq!(cpu.pc, 0x202);
    }

    #[test]
    fn jmp_to_addr_reg_0() {
        let mut cpu = cpu::init_test_cpu();

        let addr = 500;
        cpu.regs[0] = 10;

        cpu.pc = cpu.regs[0] as u16 + addr;

        assert_eq!(cpu.pc, 510);
    }

    #[test]
    fn ld_dt_to_reg() {
        let mut cpu = cpu::init_test_cpu();

        let x = 5;
        cpu.delay_timer = 60;

        cpu.regs[x] = cpu.delay_timer;

        assert_eq!(cpu.regs[x], 60);
    }

    #[test]
    fn ld_key() {
        let mut cpu = cpu::init_test_cpu();
        let mut window = WindowManager::init_sdl();

        let x = 5;

        'running: loop {
            let key_pressed = check_for_key_press(&mut window.event_pump);
            cpu.set_key(&key_pressed);

            if cpu.key_pressed == [0u8; 16] {
                cpu.pc -= 2;
                println!("pc - 2")
            } else {
                println!("reg {}", cpu.regs[x]);
                for i in 0..cpu.key_pressed.len() {
                    if cpu.key_pressed[i] == 1 {
                        cpu.regs[x] = i as u8;
                        println!("reg {}", cpu.regs[x]);

                        break 'running;
                    }
                }
            }
        }
    }

    #[test]
    fn ld_reg_to_dt() {
        let mut cpu = cpu::init_test_cpu();

        let x = 5;
        cpu.delay_timer = 60;
        cpu.regs[x] = 10;

        cpu.delay_timer = cpu.regs[x];

        assert_eq!(cpu.delay_timer, 10);
    }

    #[test]
    fn ld_st_to_reg() {
        let mut cpu = cpu::init_test_cpu();

        let x = 5;
        cpu.sound_timer = 60;

        cpu.regs[x] = cpu.sound_timer;

        assert_eq!(cpu.regs[x], 60);
    }

    #[test]
    fn add_i_to_reg() {
        let mut cpu = cpu::init_test_cpu();

        let x = 5;
        cpu.index_reg = 100;
        cpu.regs[x] = 10;

        cpu.index_reg += cpu.regs[x] as u16;

        assert_eq!(cpu.index_reg, 110);
    }

    #[test]
    fn ld_font() {
        let mut cpu = cpu::init_test_cpu();

        let x = 5;
        cpu.regs[x] = 5;
        cpu.index_reg = cpu.regs[x] as u16 * 5;

        assert_eq!(cpu.index_reg, 25);
    }

    #[test]
    fn bcd() {
        let mut cpu = cpu::init_test_cpu();

        let x = 5;
        cpu.regs[x] = 012;

        cpu.ram[cpu.index_reg as usize] = cpu.regs[x] / 100;
        cpu.ram[cpu.index_reg as usize + 1] = (cpu.regs[x] % 100) / 10;
        cpu.ram[cpu.index_reg as usize + 2] = cpu.regs[x] % 10;

        assert_eq!(cpu.ram[cpu.index_reg as usize], 0);
        assert_eq!(cpu.ram[cpu.index_reg as usize + 1], 1);
        assert_eq!(cpu.ram[cpu.index_reg as usize + 2], 2);
    }   

    #[test]
    fn ld_reg_to_ram() {
        let mut cpu = cpu::init_test_cpu();

        let x = 5;
        cpu.regs[0] = 1;
        cpu.regs[1] = 2;
        cpu.regs[2] = 3;
        cpu.regs[3] = 4;
        cpu.regs[4] = 5;
        cpu.regs[5] = 6;


        if x == 0 {
            cpu.ram[cpu.index_reg as usize] = cpu.regs[0];
        }

        for i in 0..x + 1{
            cpu.ram[cpu.index_reg as usize + i] = cpu.regs[i];
        }

        assert_eq!(cpu.ram[0], 1);
        assert_eq!(cpu.ram[1], 2);
        assert_eq!(cpu.ram[2], 3);
        assert_eq!(cpu.ram[3], 4);
        assert_eq!(cpu.ram[4], 5);
        assert_eq!(cpu.ram[5], 6);

    }
    
    #[test]
    fn ld_ram_to_reg() {
        let mut cpu = cpu::init_test_cpu();

        let x = 5;

        cpu.ram[0] = 1;
        cpu.ram[1] = 2;
        cpu.ram[2] = 3;
        cpu.ram[3] = 4;
        cpu.ram[4] = 5;
        cpu.ram[5] = 6;


        if x == 0 {
            cpu.regs[0] = cpu.ram[cpu.index_reg as usize];
        }

        for i in 0..x + 1 {
            cpu.regs[i] = cpu.ram[cpu.index_reg as usize + i];
        }

        assert_eq!(cpu.regs[0], 1);
        assert_eq!(cpu.regs[1], 2);
        assert_eq!(cpu.regs[2], 3);
        assert_eq!(cpu.regs[3], 4);
        assert_eq!(cpu.regs[4], 5);
        assert_eq!(cpu.regs[5], 6);

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
