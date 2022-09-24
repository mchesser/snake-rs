mod client;
mod game;
mod snake;

#[macroquad::main("Snake")]
async fn main() {
    client::run().await.unwrap();
}
