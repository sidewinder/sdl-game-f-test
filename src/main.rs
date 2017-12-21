extern crate sdl2;

mod game;

fn main() {
    let mut game = game::Game::new("Hello SDL2", 640, 480, true);
    
    while game.is_running_instance() == true {
        game.handle_events();
        game.update();
        game.render();
    }    
}
