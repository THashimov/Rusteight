use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, TextureCreator},
    ttf::Font,
    video::{Window, WindowContext},
    EventPump,
};

use crate::cpu::CPU;

const TEXT_INDENT: i32 = 700;

struct CpuInfo {
    text_height: i32,
    coords: Rect
}

impl CpuInfo { 
    fn init_cpu_info(y_coord: i32, text_height: i32) -> CpuInfo {
        let coords = Rect::new(
            TEXT_INDENT,
            y_coord,
            0 as u32,
            text_height as u32);

    CpuInfo { text_height, coords}

}
}

pub struct WindowManager {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub texture_creator: TextureCreator<WindowContext>,
}

impl WindowManager {
    pub fn init_sdl() -> WindowManager {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        let window = video_subsystem
            .window("RustEight", 1500, 640)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let texture_creator = canvas.texture_creator();

        WindowManager {
            canvas,
            event_pump,
            texture_creator,
        }
    }

    pub fn refresh(&mut self, display: &[[u8; 64]; 32], font: &Font, cpu: &CPU) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(0, 255, 0));

        for x_coord in 0..64 {
            for y_coord in 0..32 {
                let byte = display[y_coord][x_coord];
                for bit in 0..8 {
                    // Draw a rect for every pixel that is on
                    if (byte >> bit) & 1 == 1 {
                        self.canvas
                            .fill_rect(Rect::new(
                                (x_coord as i32 + bit) * 10,
                                y_coord as i32 * 10,
                                10,
                                10,
                            ))
                            .unwrap();
                    }
                }
            }
        }
        self.render_cpu_info(&font, cpu);
        self.canvas.present();
    }

    pub fn render_text(&mut self, rect: Rect, font: &Font, text: &str) {        
        let surface = font.render(&text).blended(Color::RGB(200, 0, 0));

        let surface = match surface {
            Ok(surface) => surface,
            Err(error) => panic!("{:?}", error),
        };

        let texture = self
            .texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        self.canvas.copy(&texture, None, Some(rect)).unwrap();
    }

    fn render_cpu_info(&mut self, font:&Font, cpu: &CPU) {
        let mut cpu_info = CpuInfo::init_cpu_info(0, 30);

        let reg_info = cpu.regs;
        let mut reg_str = String::with_capacity(10);

        for i in 0..reg_info.len() {
            reg_str.push_str(&i.to_string());
            reg_str.push_str(&String::from(" "));
            reg_str.push_str(&reg_info[i].to_string());
            cpu_info.coords.set_width(reg_str.len() as u32 * 20);
            self.render_text(cpu_info.coords, font, &reg_str);
            cpu_info.coords.y += cpu_info.text_height;
            reg_str = String::from("Reg");
        }
    }
}
