//! An abstract representation of the game 'world' and its extents.

use sdl2::render::WindowCanvas;
use sdl2::rect::Rect;
use std::collections::HashMap;
use game::stage::Stage;
use entity::Entity;

/// A game world.
pub struct World<'a> {
    /// The width of the game world.
    pub w: u32,
    /// The height of the game world.
    pub h: u32,
    /// The world entities.
    pub entities: Vec<Entity<'a>>,
    /// The active game stage.
    pub active_stage: Option<i32>,
    stages: HashMap<i32, Stage<'a>>,
}

impl<'a> World<'a> {
    /// Creates new World with the give width and height.
    pub fn new(w: u32, h: u32) -> World<'a> {
        World {
            w,
            h,
            entities: Vec::new(),
            active_stage: None,
            stages: HashMap::new(),
        }
    }

    /// Add a stage to the game world
    pub fn add_stage(&mut self, k: i32, s: Stage<'a>) -> () {
        self.stages.insert(k, s);
    }

    /// Gets the active stage.
    pub fn get_active_stage(&self) -> Option<&Stage<'a>> {
        self.active_stage.and_then(|a| self.stages.get(&a))
    }

    /// Sets the active stage.
    pub fn set_active_stage(&mut self, k: i32) -> () {
        self.active_stage = Some(k);
    }

    /// Add entity to world.
    pub fn add_entity(&mut self, entity: Entity<'a>) -> () {
        self.entities.push(entity);
    }

    /// Updates the game world, passing in the delta time.
    pub fn update(&self, delta: f64) -> () {
        self.active_stage.map(|a| self.stages.get(&a).map(|s| {}));
    }

    /// Renders the game world, passing in the delta time.
    pub fn render(&self, canvas: &mut WindowCanvas, delta: f64) -> () {
        for entity in self.entities.iter() {
            for sprite in entity.sprite.iter() {
                sprite.render(Some(Rect::new(0, 0, 512, 480)), canvas);
            }
        }
    }
}
