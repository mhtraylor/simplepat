//! An abstract representation of the game 'world' and its extents.

use std::collections::HashMap;
use game::stage::Stage;

/// A game world.
pub struct World {
    /// The width of the game world.
    pub w: u32,
    /// The height of the game world.
    pub h: u32,
    /// The active game stage.
    pub active_stage: Option<i32>,
    stages: HashMap<i32, Stage>,
}

impl World {
    /// Creates new World with the give width and height.
    pub fn new(w: u32, h: u32) -> World {
        World {
            w,
            h,
            active_stage: None,
            stages: HashMap::new(),
        }
    }

    /// Add a stage to the game world
    pub fn add_stage(&mut self, k: i32, s: Stage) -> () {
        self.stages.insert(k, s);
    }

    /// Sets the active stage.
    pub fn set_active_stage(&mut self, k: i32) -> () {
        self.active_stage = Some(k);
    }

    /// Updates the game world, passing in the delta time.
    pub fn update(&self, delta: f64) -> () {
        self.active_stage.map(|a| {
            self.stages.get(&a).map(|s| {
                println!("{}", a);
            })
        });
    }
}