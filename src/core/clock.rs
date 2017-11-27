//! Game update timing.

use sdl2::TimerSubsystem;

/// The game update clock.
pub struct Clock {
    cur: f64,
    pre: f64,
    delta: f64,
    timer: TimerSubsystem,
}

impl<'a> Clock {
    /// Creates a new Clock from SDL timer.
    pub fn new(timer: TimerSubsystem) -> Clock {
        let mut clock = Clock {
            cur: 0.,
            pre: 0.,
            delta: 0.,
            timer,
        };

        clock.pre = clock.timer.ticks() as f64;
        clock
    }

    /// Step the clock, returning the delta time.
    pub fn tick(&mut self) -> f64 {
        self.cur = self.timer.ticks() as f64;
        self.delta = (self.cur - self.pre) / 1000.;
        self.pre = self.cur;
        self.delta
    }
}
