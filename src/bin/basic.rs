extern crate sdl2;
extern crate simplepat;
extern crate vecmath;



pub fn main() {
    use sdl2::*;
    use simplepat::game::Game;

    let sdl = sdl2::init().unwrap();
    let mut game = Game::new("SimplePat Engine Demo", 512, 512, sdl);

    game.play();
}
