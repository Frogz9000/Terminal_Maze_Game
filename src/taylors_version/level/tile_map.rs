use super::position::Position;


pub struct TileMap {
    dimensions: Dimensions,
    tiles: Vec<Tile>
}
impl TileMap {
    pub fn new_empty(dimensions: Dimensions) -> Self {
        Self {
            dimensions,
            tiles: std::iter::repeat(Tile::Empty)
                .take(dimensions.width * dimensions.height)
                .collect()
        }
    }

    pub fn get(&self, position: Position) -> Option<Tile> {
        if !position.is_within(self.dimensions()) {
            return None
        }

        Some(unsafe { self.get_unchecked(position) })
    }

    pub unsafe fn get_unchecked(&self, position: Position) -> Tile {
        self.tiles[position.x() + self.dimensions().width * position.y()]
    }

    pub fn is_traversable(&self, position: Position) -> bool {
        self.get(position)
            .is_some_and(|tile| tile == Tile::Empty)
    }

    pub fn set_tiles(&mut self, mapping: impl Fn(Position) -> Tile) {
        let width = self.dimensions.width();

        for (idx, tile) in self.tiles.iter_mut().enumerate() {
            *tile = mapping(Position::new(idx % width, idx / width))
        }
    }

    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }
}

#[derive(Clone, Copy)]
pub struct Dimensions {
    width: usize,
    height: usize
}

impl Dimensions {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Obstacle
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Empty => "🟫",
            Self::Obstacle => "🌵"
        })
    }
}