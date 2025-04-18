mod matrix;
mod game;
pub mod direction;
mod taylors_version;

use std::io::{self,Write};
use game::*;
use matrix::*;
use level_gen::*;

const COL:usize = 10;
const ROW:usize = 20;

const TAYLORS_VERSION: bool = true;

fn main() {
    if TAYLORS_VERSION {
        taylors_version::main();
        return;
    }

    let mut current_game = Game{level_map:  Box::new(radom_gen_lvl()),score: 0.0,level_number:1};
    current_game.game_loop();
    loop{
        print!("\r");
        print!("Press Enter for a new maze or q to quit: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input.eq_ignore_ascii_case("q"){
            println!("Exiting program");
            break;
        }
        println!("Proceeding to next puzzle!");
        current_game.level_map = Box::new(radom_gen_lvl());
        current_game.game_loop();
    }
}