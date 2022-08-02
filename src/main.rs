use std::time::Duration;

use cpu::CPU;
use keypad::{check_for_key_press, KeyStroke};
use window_manager::WindowManager;

mod cpu;
mod rom_loader;
mod keypad;
mod window_manager;
mod tests;


fn main() {
    let path = String::from("./src/ROMS/IBM.ch8");
    let rom = rom_loader::load_rom(path);

    let mut cpu = CPU::init_cpu();
    cpu.ram = rom;  
    let mut window = WindowManager::init_sdl();

    'running: loop {
        if check_for_key_press(&mut window.event_pump) == KeyStroke::Quit {
            break 'running
        } else if check_for_key_press(&mut window.event_pump) == KeyStroke::Next {
            cpu.tick();
            window.refresh(&cpu.display);
        } else {
            
        }
    }
    
}


