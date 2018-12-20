extern crate sdl2;
extern crate simplepat;
extern crate vecmath;

use sdl2::*;
use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use sdl2::render::{TextureCreator, WindowCanvas};
use simplepat::game::Game;
use simplepat::game::stage::Stage;
use simplepat::component::sprite::Sprite;
use simplepat::entity::Entity;
use std::path::Path;

pub fn main() {
    let sdl = sdl2::init().unwrap();

    let video = sdl.video().unwrap();
    let window = video
        .window("SimplePat Engine Demo", 512, 480)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let tc = canvas.texture_creator();
    let tx = tc.load_texture(Path::new("../../src/bin/assets/gfx/tile_bkg.png"))
        .unwrap();

    let mut game = Game::new(512, 480, &mut canvas, sdl);
    let mut stage_0 = Stage::new();

    let mut bkg_entity = Entity::new();
    bkg_entity.add_sprite(Some(Sprite::new(&tx, Some(Rect::new(0, 0, 512, 480)))));

    game.world.add_entity(bkg_entity);

    game.world.add_stage(0, stage_0);
    game.world.set_active_stage(0);

    game.play();
}
