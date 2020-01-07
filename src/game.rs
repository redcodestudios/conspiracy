extern crate sdl2;

use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::render::Canvas;
    
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

        let canvas = window.into_canvas().build().unwrap();

        Game { 
            name: name, 
            window_width: width, 
            window_height: height,
            sdl_ctx: sdl_ctx,
            canvas: canvas
        }
    }
}
