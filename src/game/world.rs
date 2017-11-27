//! An abstract representation of the game 'world' and its extents.

/// A game world.
pub struct World {
    /// The width of the game world.
    pub w: u32,
    /// The height of the game world.
    pub h: u32,
}

impl World {
    /// Creates new World with the give width and height.
    pub fn new(w: u32, h: u32) -> World {
        World { w, h }
    }
}
