use crate::*;

pub fn radom_gen_lvl()->Matrix<COL,ROW>{
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
    let start = matrix.player_position();
    let end = matrix.goal_position();
    //TODO ask jack, implement dome pathing algo to ensure goal is reach able
    //If this works, my valid checking is air tight enought to turn up random gen to a higher level and get even more hog wild
    //Try making random gen smarter by making longer straight line and stuff like that to be more maze like
    
    
    Ok(())
}