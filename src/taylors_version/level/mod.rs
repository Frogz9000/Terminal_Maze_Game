pub mod generation;
pub mod position;
pub mod tile_map;

use std::fmt::Write;
use position::Position;
use tile_map::{Dimensions, TileMap};

pub struct Level {
    spawn_position: Position,
    goal_position: Position,
    tile_map: TileMap,
}

impl Level {
    pub fn tile_map(&self) -> &TileMap {
        &self.tile_map
    }

    pub fn draw_to_string(&self, player_position: Position) -> String {
        let mut out = String::new();

        for y in 0..self.tile_map.dimensions().height() {
            for x in 0..self.tile_map.dimensions().width() {
                let position = Position::new(x, y);

                if player_position == position {
                    write!(out, "😎 ").unwrap();
                } else if self.goal_position() == position {
                    write!(out, "🏆️ ").unwrap();
                } else {
                    write!(out, "{} ", unsafe { self.tile_map.get_unchecked(position) }).unwrap();
                }
            }
            writeln!(out).unwrap();
        }

        out
    }

    pub fn dimensions(&self) -> Dimensions {
        self.tile_map.dimensions()
    }

    pub fn goal_position(&self) -> Position {
        self.goal_position
    }
    
    pub(crate) fn spawn_position(&self) -> Position {
        self.spawn_position
    }
}
