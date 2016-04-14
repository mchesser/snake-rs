extern crate sdl2;
extern crate basic2d;
extern crate rand;

mod game;
mod client;
mod snake;

fn main() {
    client::run().unwrap();
}
