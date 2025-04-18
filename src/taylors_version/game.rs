use std::process::exit;
use std::time::Instant;

use crate::taylors_version::input::{GameInput, MenuInput};
use crate::taylors_version::input;
use crate::taylors_version::level::tile_map::Tile;

use super::level::generation::LevelGenerator;
use super::level::tile_map::Dimensions;
use super::level::Level;

pub struct Game{
    level_generator: LevelGenerator,
    levels_completed: usize,
    score: f64,
}

impl Game {
    pub(crate) fn new() -> Self {
        Self {
            level_generator: LevelGenerator::new_seeded(0, Dimensions::new(12, 10), Dimensions::new(15, 24)),
            score: 0.0,
            levels_completed: 0
        }
    }

    pub fn start(&mut self) {
        loop {
            self.start_level(&self.level_generator.generate());

            println!("Press Enter for a new maze or q to quit: ");
            loop {
                match input::wait_for_input() {
                    Some(MenuInput::NewGame) => {
                        println!("Generating next puzzle!");
                        self.level_generator.reseed();
                        break
                    }
                    Some(MenuInput::Quit) => {
                        clear_terminal();
                        println!("You completed {} levels with a score of {:.0}!", self.levels_completed, self.score);
                        return
                    }
                    // Just keep waiting
                    None => continue
                }
            }
        }
    }

    fn start_level(&mut self, level: &Level) {
        let start_time = Instant::now();
        let mut player_position = level.spawn_position();

        loop {
            clear_terminal();
            print_score(self.score, self.levels_completed);
            print_controls();
            println!("{}", level.draw_to_string(player_position));

            match input::wait_for_input() {
                Some(GameInput::Direction(direction)) => {
                    if let Some(new_position) = player_position.offset_bounded(direction, level.dimensions()) {
                        if level.tile_map().get(new_position).is_some_and(|tile| tile == Tile::Empty) {
                            player_position = new_position
                        }
                    }
                },
                Some(GameInput::Quit) => {
                    clear_terminal();
                    print_score(self.score, self.levels_completed);
                    print!("Level quit. ");
                    break;
                }
                Some(GameInput::Break) => {
                    println!("Aborting!");
                    exit(0);
                }
                None => continue
            }

            if player_position == level.goal_position() {
                let score_time = start_time.elapsed();
                self.score+=10.0/((1.0+score_time.as_secs_f64()).log(10.0).abs());
                self.levels_completed += 1;
                clear_terminal();
                print_score(self.score, self.levels_completed);
                break;
            }
        }
    }
}

fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H")
}

fn print_score(score: f64, levels_completed: usize) {
    println!("Current Level: {}\tCurrent Score: {score:.0}", levels_completed + 1);
}

fn print_controls() {
    println!("Use arrow keys (press 'q' to quit):");
}