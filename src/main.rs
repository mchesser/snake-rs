#[feature(globs)];

extern mod native;
extern mod sdl2;

use sdl2::{video, render, keycode, timer, event};
use sdl2::rect::Rect;

use game::Game;

mod snake;
mod game;
mod point;

#[cfg(not(test))]
#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

#[main]
fn main() {
    static WHITE: sdl2::pixels::Color = sdl2::pixels::RGB(0xFF, 0xFF, 0xFF);

    let game_width:  u32 = 800;
    let game_height: u32 = 600;
    let grid_size:   u32 = 20;

    sdl2::init([sdl2::InitVideo]);

    // Initialise the window
    let window =
        match video::Window::new("Simple Snake Game", video::PosCentered,
            video::PosCentered, game_width as int, game_height as int, [video::OpenGL]) {
            Ok(window) => window,
            Err(err) => fail!(format!("failed to create window: {}", err))
    };

    // Initialise the renderer
    let renderer =
        match render::Renderer::from_window(window, render::DriverAuto, [render::Accelerated]) {
            Ok(renderer) => renderer,
            Err(err) => fail!(format!("failed to create renderer: {}", err))
    };

    // Initialise the game
    let mut game = Game::init(game_width/grid_size, game_height/grid_size, grid_size);

    let mut prev_ticks = timer::get_ticks();

    'main: loop {
        'event: loop {
            match event::poll_event() {
                event::QuitEvent(_) => break 'main,

                event::KeyDownEvent(_, _, code, _, _) => {
                    match code {
                        // Game exit
                        keycode::EscapeKey => break 'main,

                        // Player movement
                        keycode::UpKey    => game.snakes[0].set_move(game::snake::Up),
                        keycode::DownKey  => game.snakes[0].set_move(game::snake::Down),
                        keycode::LeftKey  => game.snakes[0].set_move(game::snake::Left),
                        keycode::RightKey => game.snakes[0].set_move(game::snake::Right),

                        // Anything else
                        _ => {}
                    }
                },

                event::NoEvent => break,

                _ => {}
            }
        }

        let ticks = timer::get_ticks();
        let dt = (ticks - prev_ticks) as f32 / 1000.0;
        prev_ticks = ticks;

        // Update the game
        game.update(dt);
        if game.snakes[0].dead {
            break 'main;
        }

        // Clear the screen
        renderer.set_draw_color(WHITE);
        renderer.clear();

        // Draw the game
        game.draw(renderer);

        // Refresh the screen
        renderer.present();
    }
}

#[cfg(test)]
mod tests {
}
