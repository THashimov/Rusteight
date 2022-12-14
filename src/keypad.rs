use sdl2::{event::Event, keyboard::Keycode, EventPump};

use crate::cpu::CPU;

#[derive(Debug, PartialEq)]
pub enum KeyStroke {
    Quit,
    Key([u8; 16]),
    Next,
}

pub fn check_for_key_press(event_pump: &mut EventPump, cpu: &mut CPU) -> KeyStroke {
    let mut key_pressed = [0u8; 16];

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return KeyStroke::Quit,
            Event::KeyDown {
                keycode: Some(Keycode::Num1),
                ..
            } => {
                key_pressed[0] = 1;
                cpu.key_pressed = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Num2),
                ..
            } => {
                key_pressed[1] = 1;
                cpu.key_pressed = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Num3),
                ..
            } => {
                key_pressed[2] = 1;
                cpu.key_pressed = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Num4),
                ..
            } => {
                key_pressed[3] = 1;
                cpu.key_pressed = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => {
                key_pressed[4] = 1;
                cpu.key_pressed = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => {
                key_pressed[5] = 1;
                cpu.key_pressed = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::E),
                ..
            } => {
                key_pressed[6] = 1;
                cpu.key_pressed = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => {
                key_pressed[7] = 1;
                cpu.key_pressed = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => {
                key_pressed[8] = 1;
                cpu.key_pressed = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => {
                key_pressed[9] = 1;
                cpu.key_pressed = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => {
                key_pressed[10] = 1;
                cpu.key_pressed = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::F),
                ..
            } => {
                key_pressed[11] = 1;
                cpu.key_pressed = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Z),
                ..
            } => {
                key_pressed[12] = 1;
                cpu.key_pressed = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::X),
                ..
            } => {
                key_pressed[13] = 1;
                cpu.key_pressed = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::C),
                ..
            } => {
                key_pressed[14] = 1;
                cpu.key_pressed = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::V),
                ..
            } => {
                key_pressed[15] = 1;
                cpu.key_pressed = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => return KeyStroke::Next,
            _ => {}
        }
    }
    if cpu.key_pressed {
        cpu.set_key(&KeyStroke::Key(key_pressed));
        cpu.key_pressed = false;
    }

    KeyStroke::Key(key_pressed)
}
