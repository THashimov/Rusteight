use std::time::Duration;

use cpu::CPU;
use keypad::{check_for_key_press, KeyStroke};
use window_manager::WindowManager;

mod cpu;
mod keypad;
mod rom_loader;
mod tests;
mod window_manager;

fn main() {
    let path = String::from("./src/ROMS/IBM.ch8");
    let rom = rom_loader::load_rom(path);

    let mut cpu = CPU::init_cpu();
    cpu.ram = rom;
    let mut window = WindowManager::init_sdl();

    'running: loop {
        let key_pressed = check_for_key_press(&mut window.event_pump);
        cpu.set_key(&key_pressed);

        if key_pressed == KeyStroke::Quit {
            break 'running;
        // }
        } else if key_pressed == KeyStroke::Next {
            cpu.tick();
            window.refresh(&cpu.display);
        } else {
        }
    }
}
