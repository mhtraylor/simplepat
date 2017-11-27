#![deny(missing_docs)]

//! A simple game engine.

pub extern crate sdl2;
pub extern crate vecmath;

pub mod core;
pub mod game;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
