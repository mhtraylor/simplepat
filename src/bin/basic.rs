extern crate sdl2;
extern crate simplepat;
extern crate vecmath;



pub fn main() {
    use sdl2::*;
    use simplepat::game::Game;
    use simplepat::game::stage::Stage;

    let sdl = sdl2::init().unwrap();
    let mut game = Game::new("SimplePat Engine Demo", 512, 512, sdl);

    game.world.add_stage(0, Stage {});
    game.world.set_active_stage(0);

    game.play();
}
