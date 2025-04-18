use core::f64;

use noise::NoiseFn;
use pathfinding::prelude::astar;
use rand::{Rng, SeedableRng};

use crate::taylors_version::*;

use super::{position::Position, tile_map::{Dimensions, TileMap}};

pub struct LevelGenerator {
    min_dimensions: Dimensions,
    max_dimensions: Dimensions,
    seed: u64
}

impl LevelGenerator {
    pub fn new_seeded(seed: u64, min_dimensions: Dimensions, max_dimensions: Dimensions) -> Self {
        Self {
            min_dimensions,
            max_dimensions,
            seed
        }
    }

    pub fn reseed(&mut self) {
        let mut rng = rand::prelude::StdRng::seed_from_u64(self.seed);

        self.seed = rng.random()
    }

    pub fn generate(&self) -> Level {
        const SCALE_FACTOR: f64 = 0.25;
        const THRESHOLD: f64 = 0.05;
        let mut rng = rand::prelude::StdRng::seed_from_u64(self.seed);
        
        loop {
            let dimensions = Dimensions::new(
                rng.random_range(self.min_dimensions.width()..=self.max_dimensions.width()),
                rng.random_range(self.min_dimensions.height()..=self.max_dimensions.height())
            );
            let mut tile_map: TileMap = TileMap::new_empty(dimensions);
            let spawn_position = Position::new_random_bounded(&mut rng, dimensions);
            let goal_position = Position::new_random_bounded(&mut rng, dimensions);

            let simplex = noise::Simplex::new(rng.random());

            tile_map.set_tiles(|pos| {
                match simplex.get([pos.x() as f64 * SCALE_FACTOR, pos.y() as f64 * SCALE_FACTOR]) {
                    ..THRESHOLD => tile_map::Tile::Empty,
                    THRESHOLD.. => tile_map::Tile::Obstacle,
                    _ => tile_map::Tile::Empty,
                }
            });

            if 
                !tile_map.is_traversable(spawn_position) ||
                !tile_map.is_traversable(goal_position)
            {
                continue;
            }

            let level = Level {
                tile_map,
                spawn_position,
                goal_position
            };

            if let Some((_, distance_to_goal)) = Self::find_optimal_completion(&level) {
                if distance_to_goal <= dimensions.width() + dimensions.height() {
                    // We want challenging mazes
                    continue;
                } else {
                    return level;
                }
            }
        }
    }

    fn find_optimal_completion(level: &Level) -> Option<(Vec<Position>, usize)> {
        astar(
            &level.spawn_position,
            |position| {
                position.neighbors_bounded(level.dimensions())
                    .collect::<Vec<_>>()
                    .into_iter()
                    .filter(|neighbor| level.tile_map.is_traversable(*neighbor))
                    .map(move |neighbor| (neighbor, 1))
            },
            |position| position.squared_distance(level.goal_position),
            |position| *position == level.goal_position
        )
    }
}
