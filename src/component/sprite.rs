//! Sprites: renderable components.

use sdl2::render::{Texture, WindowCanvas};
use sdl2::rect::Rect;

/// A component for rendering a sprite or 2D image.
pub struct Sprite<'a> {
    texture: &'a Texture<'a>,
    bounds: Option<Rect>,
}

impl<'a> Sprite<'a> {
    /// Creates a new Sprite from a Texture.
    pub fn new(texture: &'a Texture<'a>, bounds: Option<Rect>) -> Sprite<'a> {
        Sprite { texture, bounds }
    }

    /// Render a Sprite on to a canvas.
    pub fn render(&self, dest: Option<Rect>, canvas: &mut WindowCanvas) -> () {
        canvas
            .copy(&self.texture, self.bounds, dest)
            .expect("Render failed");
    }
}
