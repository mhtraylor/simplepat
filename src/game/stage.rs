//! A game stage that holds game entities.

use entity::Entity;

/// A 'stage' or level that contains game entities.
pub struct Stage<'a> {
    /// The list of entities in a Stage.
    pub entities: Vec<Entity<'a>>,
}

impl<'a> Stage<'a> {
    /// Creates a new Stage.
    pub fn new() -> Stage<'a> {
        Stage {
            entities: Vec::new(),
        }
    }

    /// Add an Entity to this Stage.
    pub fn add_entity(&mut self, entity: Entity<'a>) -> () {
        self.entities.push(entity);
    }
}
