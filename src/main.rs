extern mod native;
extern mod sdl2;

use sdl2::{video, render, keycode, timer, event};
use sdl2::rect::Rect;

use game::Game;

mod snake;
mod game;
mod point;

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

#[main]
fn main() {
    static WHITE: sdl2::pixels::Color = sdl2::pixels::RGB(0xFF, 0xFF, 0xFF);
    static BLACK: sdl2::pixels::Color = sdl2::pixels::RGB(0x00, 0x00, 0x00);
    static GREEN: sdl2::pixels::Color = sdl2::pixels::RGB(0x60, 0xAA, 0x60);
    static RED: sdl2::pixels::Color = sdl2::pixels::RGB(0xAA, 0x60, 0x60);
    
    let game_width = 800;
    let game_height = 600;
    let grid_size: i32 = 20;
    
    sdl2::init([sdl2::InitVideo]);
    
    // Initialise the window
    let window =
        match video::Window::new("Simple Snake Game", video::PosCentered, 
            video::PosCentered, game_width, game_height, [video::OpenGL]) { 
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
    let mut game = Game::init((game_width/grid_size as int) as uint,
        (game_height/grid_size as int) as uint);
        
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
        
        // Draw the fruit
        renderer.set_draw_color(RED);  
        renderer.fill_rect(&Rect::new(
            grid_size*game.fruit.x, grid_size*game.fruit.y, grid_size, grid_size)
        );
        
        // Draw the snake
        renderer.set_draw_color(GREEN);  
        let head = game.snakes[0].get_head();
        renderer.fill_rect(&Rect::new(
            grid_size*head.x, grid_size*head.y, grid_size, grid_size)
        );
        
        let tail_components = game.snakes[0].tail_to_points();
        for component in tail_components.iter() {
            renderer.fill_rect(&Rect::new(
                grid_size*component.x, grid_size*component.y, grid_size, grid_size)
            );
        }
        
        // Refresh the screen
        renderer.present();
    }
}
