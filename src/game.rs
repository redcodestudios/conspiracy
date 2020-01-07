extern crate sdl2;

use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
    
pub struct Game<'a> {
    pub name: &'a str,
    pub window_width: u32,
    pub window_height: u32,
    pub sdl_ctx: Sdl,
    pub canvas: Canvas<Window>,
}

impl<'a> Game<'a> {
    
    pub fn new(name: &str, width: u32, height: u32) -> Game {
        let sdl_ctx = sdl2::init().unwrap();
        
        let video_subsystem = sdl_ctx.video().unwrap();

        let window = video_subsystem.window(name, width, height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        Game { 
            name: name, 
            window_width: width, 
            window_height: height,
            sdl_ctx: sdl_ctx,
            canvas: canvas
        }
    }
    
    pub fn run(&mut self) {
        self.canvas.present();
        
        let mut event_pump = self.sdl_ctx.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
        
            self.update();

            self.canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    fn update(&self) {
        
    }
}
