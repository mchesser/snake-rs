use macroquad::{
    prelude::utils,
    time::get_frame_time,
    window::{next_frame, request_new_screen_size},
};

use crate::game::Game;

pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;
pub const GRID_SIZE: u32 = 20;

pub async fn run() -> Result<(), String> {
    request_new_screen_size(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

    let mut game = Game::new(SCREEN_WIDTH / GRID_SIZE, SCREEN_HEIGHT / GRID_SIZE, GRID_SIZE);
    let events_subscriber = utils::register_input_subscriber();
    loop {
        utils::repeat_all_miniquad_input(&mut game, events_subscriber);
        game.update(get_frame_time());
        game.draw();
        next_frame().await
    }
}
