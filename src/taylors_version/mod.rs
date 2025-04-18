mod level;
mod game;
mod input;

use game::*;
use level::*;

pub fn main() {
    Game::new().start();
}