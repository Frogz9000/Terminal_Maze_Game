use crate::*;

const AVATAR: char =  '😎';
const OBSTACLE: char = '🌵';
const GOAL: char = '🏆';
const EMPTY: char = '🟫';

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
impl<const COLS: usize, const ROWS: usize> LevelMatrixTrait for Matrix<COLS, ROWS> {

    fn print_matrix(&self) {
        for col in &self.data {
            for cell in col.iter() {
                let emoji: char = match cell {
                    DrawType::Obstacle => OBSTACLE,
                    DrawType::Player => AVATAR,
                    DrawType::Goal => GOAL,
                    DrawType::Empty => EMPTY
                };
                print!("{} ", emoji);
            }
            println!();
            print!("\r");
        }
    }
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
}

pub trait LevelMatrixTrait {
    fn goal_position(&self) -> (usize,usize);
    fn player_position(&self) -> (usize,usize);
    fn lose_game(&mut self);
    fn print_matrix(&self);
    fn set_player_start(&mut self, x:usize, y:usize)->Result<(),&str>;
    fn set_obstacle(&mut self, x:usize, y:usize, l: usize, w:usize)->Result<(),&str>;
    fn set_goal(&mut self, x:usize, y:usize)->Result<(),&str>;
    fn update_player_position(&mut self, dir : Direction);
    fn win_game(&self)->bool;

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

