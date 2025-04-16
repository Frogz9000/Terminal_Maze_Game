use std::fmt::Display;

use crate::direction::Direction;

use super::DrawType;


pub trait LevelMatrixTrait: Display{
    fn goal_position(&self) -> (usize,usize);
    fn player_position(&self) -> (usize,usize);
    fn lose_game(&mut self);
    fn set_player_start(&mut self, x:usize, y:usize)->Result<(),&str>;
    fn set_obstacle(&mut self, x:usize, y:usize, l: usize, w:usize)->Result<(),&str>;
    fn set_goal(&mut self, x:usize, y:usize)->Result<(),&str>;
    fn update_player_position(&mut self, dir : Direction);
    fn win_game(&self)->bool;
    fn get_tile(&self, coordinate: (usize, usize)) -> Option<DrawType>;
}