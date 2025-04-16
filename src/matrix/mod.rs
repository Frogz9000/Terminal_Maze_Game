pub mod level;
pub mod level_gen;

use std::fmt::Display;
use level::LevelMatrixTrait;
use crate::{direction::Direction, *};

#[derive(Clone)]
pub struct Matrix<const C: usize, const R: usize> {
    player_flag: bool,
    win_game: bool,
    player_position: (usize,usize),
    data: [[DrawType; R]; C],
    goal_position: (usize, usize)
}
impl<const C: usize, const R: usize> Default for Matrix<C, R> {
    fn default() -> Self{
        Self{
            player_flag: false,
            win_game :false,
            player_position: (0,0),
            goal_position: (0,0),
            data: [[DrawType::Empty; R]; C]
        }
    }
}
impl<const COLS: usize, const ROWS: usize> Display for Matrix<COLS, ROWS> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "({}, {})", self.x, self.y)
        for col in &self.data {
            for cell in col.iter() {
                write!(f, "{} ", cell.to_char())?;
            }
            writeln!(f, "")?;
            write!(f, "\r")?;
        }
        Ok(())
    }
}


impl<const COLS: usize, const ROWS: usize> LevelMatrixTrait for Matrix<COLS, ROWS> {
    fn set_player_start(&mut self, x:usize, y:usize)->Result<(),&str>{
        //since player is size 1x1 the x and y are it's true position
        if COLS-1<y || ROWS-1<x {
            Err("Invald Player Poition, out of bounds")
        }
        else if self.data[x][y] != DrawType::Empty {
            Err("Invald Player Poition: object already there")
        }
        else if self.player_flag == true {
            Err("Invald Player Poition: player already declared")
        }else{
            self.data[x][y] = DrawType::Player;
            self.player_flag = true;
            self.player_position = (x,y);
            Ok(())
        }
    }

    fn set_obstacle(&mut self, x:usize, y:usize, height: usize, width:usize)->Result<(),&'static str>{
        //obstacle can be variable size so x,y define top left corner 
        if y+height>COLS || x+width>ROWS { return Err("Invalid Obstacle Position")}
        for j in x..(x+width){
            for i in y..(y+height){
                if self.data[i][j] != DrawType::Empty && self.data[i][j] != DrawType::Obstacle {return Err("Invalid Obsacle Position")}
                self.data[i][j] = DrawType::Obstacle;
            }
        }
        Ok(())
    }
    fn set_goal(&mut self, x:usize, y:usize)->Result<(),&'static str>{
        if ROWS-1<y || COLS-1<x {
            Err("Invald Goal Poition, out of bounds")
        }
        else if self.data[x][y] != DrawType::Empty {
            Err("Invald GOAL Poition: object already there")
        }else{
            self.data[x][y] = DrawType::Goal;
            self.goal_position = (x,y);
            Ok(())
        }
    }

    fn update_player_position(&mut self, dir : Direction){
        let (posx, posy) = self.player_position;
        let (new_posx, new_posy) = match dir {
            Direction::Right => (posx, sat_add(posy, 1, ROWS-1)),
            Direction::Left => (posx, sat_sub(posy, 1, 0)),
            Direction::Down => (sat_add(posx, 1, COLS-1), posy),
            Direction::Up => (sat_sub(posx, 1, 0), posy),
        };
        if self.data[new_posx][new_posy]==DrawType::Obstacle {
           return; 
        }else{   
        if  self.data[new_posx][new_posy]==DrawType::Goal{
            self.win_game = true;
        }
        self.data[posx][posy] = DrawType::Empty; //Delete Old Player
        self.data[new_posx][new_posy] = DrawType::Player;
        self.player_position = (new_posx, new_posy);
        }
    }
    fn win_game(&self)->bool{
        self.win_game
    }
    fn lose_game(&mut self){
        self.win_game=false;
    }
    fn player_position(&self) -> (usize,usize){
        self.player_position
    }
    fn goal_position(&self) -> (usize,usize){
        self.goal_position
    }
    
    fn get_tile(&self, coordinate: (usize, usize)) -> Option<DrawType> {
        let (x, y) = coordinate;

        if 0 <= x && x < COL && 0 <= y && y < ROW {
            Some(self.data[x][y])
        } else {
            None
        }
    }
}

impl Matrix<COL, ROW> {
    fn get_neightbors_with_no_checks(&self, coordinate: (usize, usize)) -> impl IntoIterator<Item=(usize, usize)> {
        let mut out = Vec::new();

        let (x, y) = coordinate;

        if x > 0 {
            out.push((x - 1, y))
        }

        if x < COL - 1 {
            out.push((x + 1, y));
        }

        if y > 0 {
            out.push((x, y - 1))
        }

        if y < ROW - 1 {
            out.push((x, y + 1))
        }

        return out;
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

#[derive(Copy,Clone,PartialEq,Default)]
pub enum DrawType{
    #[default]
    Empty,
    Obstacle,
    Player,
    Goal
}
impl DrawType{
    fn to_char(&self)->char{
        match self {
            DrawType::Player => '😎',
            DrawType::Obstacle => '🌵',
            DrawType::Goal => '🏆',
            DrawType::Empty => '🟫',
        }
    }
}
