// Graphics library imports
extern mod rsfml;
use rsfml::graphics::{RenderWindow, sfClose, Color, RectangleShape};
use rsfml::window::{VideoMode, ContextSettings, event, keyboard};
use rsfml::system::{Vector2f, Clock};

use point::Point;

mod snake;
mod game;
mod point;

// Run on main thread for MacOS
#[cfg(target_os="macos")]
#[start]
fn start(argc: int, argv: **u8) -> int {
    std::rt::start_on_main_thread(argc, argv, main)
}

fn main() {
    let game_width = 800;
    let game_height = 600;
    let grid_size = 20;
    
    // Initialise the window
    let settings = ContextSettings::default();
    let mut window = RenderWindow::new(
            VideoMode::new_init(game_width, game_height, 32),
            "Simple Snake Game", sfClose, &settings).unwrap();
    
    window.set_vertical_sync_enabled(true);
    
    // Initialise the game
    let mut game = Game::init(game_width/grid_size, game_height/grid_size);
    
    let mut clock = Clock::new();
    
    // Initialise drawing rectangle
    let mut rect = RectangleShape::new().unwrap();
    rect.set_size(&Vector2f { x: grid_size as f32, y: grid_size as f32 });
    rect.set_outline_thickness(0.0);
    rect.set_fill_color(&Color::new_RGB(100, 100, 200));
    
    // Game loop
    while window.is_open() && game.player.dead == false {
        // Event loop
        loop {
            match window.poll_event() {
                event::Closed => window.close(),
            
                event::KeyPressed{code, ..} => match code {
                    // Game exit
                    keyboard::Escape => { window.close(); break; },
                
                    // Player movement
                    keyboard::Up    => game.player.set_move(snake::Up),
                    keyboard::Down  => game.player.set_move(snake::Down),
                    keyboard::Left  => game.player.set_move(snake::Left),
                    keyboard::Right => game.player.set_move(snake::Right),
                
                    // Anything else
                    _ => {}
                },
            
                event::NoEvent => break,
            
                _ => {}
            }
        }
        
        // Update the game
        let dt = clock.restart().as_seconds();
        game.update(dt);
        
        //----------| Draw Code |----------//
        // Clear the screen
        window.clear(&Color::new_RGB(0xFF, 0xFF, 0xFF));
        
        // Draw fruit
        rect.set_position(&(Vector2f::new((grid_size*game.fruit.x as uint) as f32,
                (grid_size*game.fruit.y as uint) as f32)));
        window.draw(&rect);
        
        // Draw the snake
        let head = game.player.get_head();
        rect.set_position(&(Vector2f::new((grid_size*head.x as uint) as f32,
                (grid_size*head.y as uint) as f32)));
        window.draw(&rect);
        
        let tail_components = game.player.tail_to_points();
        for component in tail_components.iter() {
            rect.set_position(&(Vector2f::new((grid_size*component.x as uint) as f32,
                    (grid_size*component.y as uint) as f32)));
            window.draw(&rect);
        }
        
        window.display();
    }   
}
