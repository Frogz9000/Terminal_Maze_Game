use super::tile_map::Dimensions;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn new_random_bounded(rng: &mut impl rand::Rng, bounds: Dimensions) -> Self {
        Self {
            x: rng.random_range(0..bounds.width()),
            y: rng.random_range(0..bounds.height())
        }
    }

    pub fn offset_bounded(&self, direction: Direction, bounds: Dimensions) -> Option<Self> {
        let (other_x, other_y) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0)
        };

        let (new_x, new_y) = (self.x as isize + other_x, self.y as isize + other_y);

        if 
            new_x < 0 || new_x >= bounds.width() as isize || 
            new_y < 0 || new_y >= bounds.height() as isize
        {
            None
        } else {
            Some(Self::new(new_x as usize, new_y as usize))
        }
    }

    pub fn neighbors_bounded(&self, bounds: Dimensions) -> impl Iterator<Item = Self> {
        [
            self.offset_bounded(Direction::Up, bounds),
            self.offset_bounded(Direction::Right, bounds),
            self.offset_bounded(Direction::Down, bounds),
            self.offset_bounded(Direction::Left, bounds)
        ].into_iter().flatten()
    }

    pub fn squared_distance(&self, other: Position) -> usize {
        let delta_x = self.x.abs_diff(other.x);
        let delta_y = self.y.abs_diff(other.y);

        delta_x * delta_x + delta_y * delta_y
    }

    pub fn is_within(&self, bounds: Dimensions) -> bool {
        self.x < bounds.width() && self.y < bounds.height()
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}

#[derive(PartialEq)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right
}