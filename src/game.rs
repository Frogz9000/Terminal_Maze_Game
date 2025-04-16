use crate::{direction::Direction, matrix::level::LevelMatrixTrait, *};
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
        let level_time = Instant::now();
        self.print_header_info();
        let current = &mut self.level_map;
        current.lose_game();

        enable_raw_mode().expect("Failed to enter raw mode");


        loop {
            let current = &mut self.level_map;
            // Check if an event is available
            if event::poll(std::time::Duration::from_millis(500)).unwrap() {
                // Read the event
                if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {

                    match Direction::try_from(code) {
                        Ok(direction) => {
                            current.update_player_position(direction);
                        },
                        Err(key) if key == KeyCode::Char('q') => {
                            println!("Quitting...");
                            self.score = 0.0;
                            self.level_number=0;
                            break;
                        },
                        _ => {}
                    };

                    println!();
                    self.print_header_info();
                    let current = &mut self.level_map;
                    // current.print_matrix();
                    println!();
                    print!("\r");
                    if  current.win_game()==true{
                        let score_time = level_time.elapsed();
                        self.score+=10.0/((1.0+score_time.as_secs_f64()).log(10.0).abs());
                        self.level_number+=1;
                        self.print_header_info();
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
    fn print_header_info(&mut self){
        let current = &mut self.level_map;
        print!("\x1B[2J\x1B[1;1H");
        println!("Current Level: {lvl}       CurrentScore: {scr:.0}",lvl=self.level_number, scr= self.score);
        print!("\r");
        println!("Use arrow keys (press 'q' to quit):");
        print!("\r");
        println!("{}", current);
    }
}

impl TryFrom<KeyCode> for Direction{
    type Error = KeyCode;
    fn try_from(value: KeyCode) -> Result<Self, Self::Error> {
        match value {
            KeyCode::Up => Ok(Direction::Up),
            KeyCode::Down => Ok(Direction::Down),
            KeyCode::Left => Ok(Direction::Left),
            KeyCode::Right => Ok(Direction::Right),
            _ => {Err(value)}
        }
    }
}