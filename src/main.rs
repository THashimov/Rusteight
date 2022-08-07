use std::time::Instant;

use cpu::CPU;
use keypad::{check_for_key_press, KeyStroke};
use window_manager::WindowManager;

mod cpu;
mod keypad;
mod rom_loader;
mod tests;
mod window_manager;

fn main() {
    let path = String::from("./src/ROMS/breakout.ch8");
    let rom = rom_loader::load_rom(path);

    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font(
        "./src/fonts/Raleway-Black.ttf",
        128,
    );

    let font = match font {
        Ok(font) => font,
        Err(err) => panic!("{}", err),
    };

    let mut cpu = CPU::init_cpu();
    cpu.ram = rom;
    let mut window = WindowManager::init_sdl();
    let mut key_pressed = check_for_key_press(&mut window.event_pump);

    'running: loop {
        key_pressed = check_for_key_press(&mut window.event_pump);
        cpu.set_key(&key_pressed);

        if key_pressed == KeyStroke::Quit {
            break 'running;
        }
            let instruction = cpu.tick();

            if cpu.delay_timer > 0 && cpu.delay_timer < 60 {
                cpu.delay_timer -= 1
            } else {
                cpu.delay_timer = 60;
            };
            if cpu.sound_timer > 0 && cpu.sound_timer < 60 {
                cpu.sound_timer -= 1
            } else {
                cpu.sound_timer = 60;
            };

            window.refresh(&cpu.display, &font, &instruction);
            std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
