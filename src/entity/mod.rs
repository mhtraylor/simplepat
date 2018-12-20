//! Game entities.

use component::transform::Transform;
use component::sprite::Sprite;

/// An object in the game world.
pub struct Entity<'a> {
    /// The entity's Tranform.
    pub transform: Transform,
    /// The optional Sprite component.
    pub sprite: Option<Sprite<'a>>,
}

impl<'a> Entity<'a> {
    /// Creates new Entity.
    pub fn new() -> Entity<'a> {
        Entity {
            transform: Transform::zero(),
            sprite: None,
        }
    }

    /// Add a Sprite to this Entity.
    pub fn add_sprite(&mut self, sprite: Option<Sprite<'a>>) {
        self.sprite = sprite;
    }
}
