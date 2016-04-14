extern crate clock_ticks;

use std::thread;
use std::time::Duration;

use sdl2;
use sdl2::event::Event;
use sdl2::pixels::Color;

use game::Game;

pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;
pub const GRID_SIZE: u32 = 20;

pub fn run() -> Result<(), String> {
    const WHITE: Color = Color::RGB(0xFF, 0xFF, 0xFF);

    let sdl_context = try!(sdl2::init());

    let window = try!(sdl_context.video())
                    .window("Snake", SCREEN_WIDTH, SCREEN_HEIGHT)
                    .position_centered().opengl().build().unwrap();
    let mut renderer = window.renderer().build().unwrap();

    let mut game = Game::new(SCREEN_WIDTH/GRID_SIZE, SCREEN_HEIGHT/GRID_SIZE, GRID_SIZE);

    let mut prev_time = clock_ticks::precise_time_ns();
    let mut frame_time = 0;

    let mut events = try!(sdl_context.event_pump());

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'main,
                Event::KeyDown{ keycode: Some(keycode), .. } => game.key_down(keycode),

                _ => {},
            }
        }

        renderer.set_draw_color(WHITE);
        renderer.clear();
        game.draw(&mut renderer);
        renderer.present();

        let current_time = clock_ticks::precise_time_ns();
        frame_time += current_time - prev_time;
        prev_time = current_time;

        const TARGET_TIME_STEP: u64 = 16666667;
        while frame_time >= TARGET_TIME_STEP {
            frame_time -= TARGET_TIME_STEP;
            game.update(TARGET_TIME_STEP as f32 / 1000_000_000.0);
            if game.snakes[0].dead {
                break 'main;
            }
        }

        thread::sleep(Duration::new(0, (TARGET_TIME_STEP - frame_time) as u32));
    }
    Ok(())
}
