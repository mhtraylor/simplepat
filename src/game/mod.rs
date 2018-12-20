//! Main game, world, etc. controllers.

pub mod world;
pub mod stage;

use sdl2::{EventPump, Sdl};
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::image::LoadTexture;
use sdl2::video::WindowContext;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::path::Path;

use core::clock::Clock;
use game::world::World;
use entity::Entity;
use component::sprite::Sprite;

/// Central game controller, handles all entities'
/// state, rendering, and updates.
pub struct Game<'a> {
    clock: Clock,
    /// The game world.
    pub world: World<'a>,
    /// The game canvas.
    pub canvas: &'a mut WindowCanvas,
    events: EventPump,
}

impl<'a> Game<'a> {
    /// Creates a new Game.
    pub fn new(w: u32, h: u32, canvas: &'a mut WindowCanvas, sdl: Sdl) -> Game<'a> {
        Game {
            clock: Clock::new(sdl.timer().unwrap()),
            world: World::new(w, h),
            canvas: canvas,
            events: sdl.event_pump().unwrap(),
        }
    }

    /// Runs the main game loop.
    pub fn play(&mut self) -> () {
        'main: loop {
            // Poll events
            for event in self.events.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'main,
                    _ => {}
                }
            }

            let delta = self.clock.tick();

            // Update world
            self.world.update(delta);
            // Render world
            self.canvas.clear();
            self.world.render(self.canvas, delta);
            self.canvas.present();
        }
    }
}
