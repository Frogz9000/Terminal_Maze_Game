use crate::*;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{enable_raw_mode, disable_raw_mode},
};
use std::time::Instant;

pub struct Game{
    pub level_map: Box<dyn LevelMatrixTrait>,
    pub score: f64,
    pub level_number: usize,
}
impl Game {
pub fn game_loop(&mut self){
    let current = &mut self.level_map;
    let level_time = Instant::now();
    print!("\x1B[2J\x1B[1;1H");
    println!("Current Level: {lvl}       CurrentScore: {scr:.0}",lvl=self.level_number, scr= self.score);
    print!("\r");
    println!("Use arrow keys (press 'q' to quit):");
    print!("\r");
    current.print_matrix();
    println!();
    print!("\r");
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
                        self.score = 0.0;
                        self.level_number=0;
                        break;
                    }
                    _ => {}
                }
                println!();
                print!("\x1B[2J\x1B[1;1H");
                println!("Current Level: {lvl}       CurrentScore: {scr:.0}",lvl=self.level_number, scr= self.score);
                print!("\r");
                println!("Use arrow keys (press 'q' to quit):");
                print!("\r");
                current.print_matrix();
                println!();
                print!("\r");
                if  current.win_game()==true{
                    let score_time = level_time.elapsed();
                    self.score+=10.0/((1.0+score_time.as_secs_f64()).log(10.0).abs());
                    self.level_number+=1;
                    print!("\x1B[2J\x1B[1;1H");
                    println!("\r");
                    println!("Current Level: {lvl}       CurrentScore: {scr:.0}",lvl=self.level_number, scr= self.score);
                    print!("\r");
                    println!("Use arrow keys (press 'q' to quit):");
                    print!("\r");
                    current.print_matrix();
                    println!();
                    print!("\r");
                    println!("You win :)");
                    print!("\r");
                    break;
                }
            }
        }
    }
    disable_raw_mode().expect("Failed to exit raw mode");


}
}

