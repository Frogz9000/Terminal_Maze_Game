mod matrix;
use std::usize;
use std::io::{self,Write};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{enable_raw_mode, disable_raw_mode},
};
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
const AVATAR: char =  '😎';
const OBSTACLE: char = '🌵';
const GOAL: char = '🏆';
const EMPTY: char = '🟫';

struct Game{
    levels: Box<dyn LevelMatrixTrait>,
}
impl Game {
fn game_loop(&mut self){
    let current = &mut self.levels;
    print!("\x1B[2J\x1B[1;1H");
    println!("Use arrow keys (press 'q' to quit):");
    current.print_matrix();
    current.lose_game();
    enable_raw_mode().expect("Failed to enter raw mode");
    loop {
        // Check if an event is available
        if event::poll(std::time::Duration::from_millis(500)).unwrap() {
            // Read the event
            if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                match code {
                    KeyCode::Up => current.update_player_position(Direction::Up),
                    KeyCode::Down => current.update_player_position(Direction::Down),
                    KeyCode::Left => current.update_player_position(Direction::Left),
                    KeyCode::Right => current.update_player_position(Direction::Right),
                    KeyCode::Char('q') => {
                        println!("Quitting...");
                        break;
                    }
                    _ => {}
                }
                println!();
                print!("\x1B[2J\x1B[1;1H");
                println!("Use arrow keys (press 'q' to quit):");
                print!("\r");
                current.print_matrix();
                println!();
                print!("\r");
                if  current.win_game()==true{
                    print!("\r");
                    print!("You win :)");
                    println!("\r");
                    break;
                }
            }
        }
    }
    disable_raw_mode().expect("Failed to exit raw mode");


}
}
fn sat_add(num1:usize, num2:usize, limit:usize) -> usize{
        let mut res = num1+num2;
        if res>limit{
            res = limit;
        }
        res
    }


fn sat_sub(num1:usize, num2:usize, limit:usize)-> usize{
    let mut res = num1.saturating_sub(num2);
    if res<limit{
        res = limit;
    }
    res
}

const COL:usize = 10;
const ROW:usize = 20;
fn main() {
    //start with a blank frame
    let mut frame: Matrix<COL, ROW> = Matrix::default();
    let _ = frame.set_player_start(0,5);
    let _ = frame.set_goal(COL-1, ROW-1);
    let _ = frame.set_obstacle(2, 2, 2, 2);

    let mut current_game = Game{levels:  Box::new(frame)};
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
        current_game.levels = Box::new(radom_gen_lvl());
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
        let (len, wid) = random_size(4,4);
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