use pathfinding::prelude::astar;

use crate::*;

use super::level::LevelMatrixTrait;

pub fn radom_gen_lvl()->Matrix<COL,ROW>{
    loop{
        if let Some(level) = setup_level(){
            if let Ok(()) = valid_path(&level) {
                break level;
            }
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
    let Ok(()) = valid_path(&lvl) else {return None};
    Some(lvl)
}

fn random_position(c: usize, r: usize)->(usize,usize){
    (rand::random_range(0..c-1),rand::random_range(0..r-1))
}

fn random_size(c: usize, r: usize)->(usize,usize){
    (rand::random_range(1..c),rand::random_range(1..r))
}

fn valid_path(matrix: &Matrix<COL, ROW>)->Result<(),&'static str>{
    if let Some(_) = astar(
        &matrix.player_position,
        |coordinate| neighbors_and_heuristics_of_neighbors_of_coordinate(*coordinate, matrix),
        |coordinate| distance(*coordinate, matrix),
        |coordinate| coordinate.0 == matrix.goal_position.0 && coordinate.1 == matrix.goal_position.1
    ) {
        Ok(())
    } else {
        Err("No path found")
    }
}

fn neighbors_and_heuristics_of_neighbors_of_coordinate(coordinate: (usize, usize), matrix: &Matrix<COL, ROW>) -> impl IntoIterator<Item=((usize, usize), usize)> {
    matrix.get_neightbors_with_no_checks(coordinate)
        .into_iter()
        .filter(|neighbor| matches!(matrix.get_tile(*neighbor), Some(DrawType::Empty | DrawType::Goal)))
        .map(|neighbor| (neighbor, 1))
}

fn distance(coordinate: (usize, usize), matrix: &Matrix<COL, ROW>) -> usize {
    let (x, y) = coordinate;
    let (goal_x, goal_y) = matrix.goal_position;

    let x_dist = x.abs_diff(goal_x);
    let y_dist = y.abs_diff(goal_y);

    return x_dist * x_dist + y_dist * y_dist;
}