extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod game;

use game::Game;

const WINDOW_WIDTH: u32 = 960;
const WINDOW_HEIGHT: u32= 640;

fn main() {
    let mut game = Game::new("Conspiracy", WINDOW_WIDTH, WINDOW_HEIGHT);
    
    game.canvas.set_draw_color(Color::RGB(0, 5, 5));
    game.canvas.clear();
    game.canvas.present();

    let mut event_pump = game.sdl_ctx.event_pump().unwrap();
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
        
        // updates
        

        game.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

}
