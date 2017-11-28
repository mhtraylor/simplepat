//! Main game, world, etc. controllers.

pub mod world;
pub mod stage;

use sdl2::{EventPump, Sdl};
use sdl2::render::WindowCanvas;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use core::clock::Clock;
use game::world::World;

/// Central game controller, handles all entities'
/// state, rendering, and updates.
pub struct Game {
    clock: Clock,
    /// The game world.
    pub world: World,
    canvas: WindowCanvas,
    events: EventPump,
}

impl Game {
    /// Creates a new Game.
    pub fn new(name: &str, w: u32, h: u32, sdl: Sdl) -> Game {
        let video = sdl.video().unwrap();
        let window = video
            .window(name, w, h)
            .position_centered()
            .opengl()
            .build()
            .unwrap();


        Game {
            clock: Clock::new(sdl.timer().unwrap()),
            world: World::new(w, h),
            canvas: window.into_canvas().present_vsync().build().unwrap(),
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

            // Update world
            self.world.update(self.clock.tick());
            // Render world
            self.canvas.clear();
            self.canvas.present();
        }
    }
}
