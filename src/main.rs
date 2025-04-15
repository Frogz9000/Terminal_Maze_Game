mod matrix;
mod game;
use std::usize;
use std::io::{self,Write};
use game::*;
use matrix::*;

#[derive(Copy,Clone,PartialEq,Default)]
enum DrawType{
    #[default]
    Empty,
    Obstacle,
    Player,
    Goal
}
#[derive(PartialEq)]
enum Direction{
    Up,
    Down,
    Left,
    Right
}

const COL:usize = 10;
const ROW:usize = 20;
fn main() {

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



fn radom_gen_lvl()->Matrix<COL,ROW>{
    loop{
        if let Some(level) = setup_level(){
            break level;
        }
    }   
}

fn setup_level()->Option<Matrix<COL,ROW>>{

    let mut lvl: Matrix<COL,ROW> = Matrix::default();
    let num_obstacles = rand::random_range(1..(ROW*COL/4));
    for _ in 0..num_obstacles{
        let (c, r) = random_position(COL, ROW);
        let (len, wid) = random_size(4,8);
        let Ok(()) = lvl.set_obstacle(r, c, len, wid) else { return None};
    }
    let (c, r) = random_position(COL, ROW);
    let Ok(()) = lvl.set_goal(c, r) else { return None};
    let (c, r) = random_position(COL, ROW);
    let Ok(()) = lvl.set_player_start(c, r) else { return None};
    Some(lvl)
}

fn random_position(c: usize, r: usize)->(usize,usize){
    (rand::random_range(0..c-1),rand::random_range(0..r-1))
}

fn random_size(c: usize, r: usize)->(usize,usize){
    (rand::random_range(1..c),rand::random_range(1..r))
}