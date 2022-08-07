use sdl2::{pixels::Color, rect::Rect, render::{Canvas, TextureCreator}, video::{Window, WindowContext}, EventPump, ttf::Font};

pub struct WindowManager {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub texture_creator: TextureCreator<WindowContext>
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

        WindowManager { canvas, event_pump, texture_creator }
    }

    pub fn refresh(&mut self, display: &[[u8; 64]; 32], font: &Font, instruction: &str) {
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
        self.render_instruction(&font, &instruction);
        self.canvas.present();
    }

    pub fn render_instruction(&mut self, font: &Font, text: &str) {
        let surface = font.render(&text).blended(Color::RGB(255, 255, 255));

        let surface = match surface {
            Ok(surface) => surface,
            Err(error) => panic!("{:?}", error),
        };

        let texture = 
            self.texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();

        let rect = Rect::new(700, 0, (text.len() * 10) as u32, 50);

        self.canvas.copy(&texture, None, Some(rect)).unwrap();
    }
}
