/// Initially from: https://github.com/StarArawn/bevy_ecs_tilemap
/// 
/// A tile position in the tilemap grid.
#[derive(Component, Reflect, Default, Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub struct TilePos {
    pub x: u32,
    pub y: u32,
}

impl TilePos {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    /// Converts a tile position (2D) into an index in a flattened vector (1D), assuming the
    /// tile position lies in a tilemap of the specified size.
    pub fn to_index(&self, tilemap_size: &TilemapSize) -> usize {
        ((self.y * tilemap_size.x) + self.x) as usize
    }

    /// Checks to see if `self` lies within a tilemap of the specified size.
    pub fn within_map_bounds(&self, map_size: &TilemapSize) -> bool {
        self.x < map_size.x && self.y < map_size.y
    }
}