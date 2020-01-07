mod game;

use game::Game;

const WINDOW_WIDTH: u32 = 960;
const WINDOW_HEIGHT: u32= 640;

fn main() {
    let mut game = Game::new("Conspiracy", WINDOW_WIDTH, WINDOW_HEIGHT);
    game.run();
}
