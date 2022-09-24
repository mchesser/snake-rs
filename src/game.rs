use macroquad::{
    miniquad::EventHandler,
    prelude::{Color, IVec2, KeyCode},
    shapes::draw_rectangle,
    time::get_frame_time,
};
use rand::{thread_rng, Rng};

use crate::snake::{Move, Snake};

// Game structure
pub struct Game {
    pub snakes: Vec<Snake>,
    fruit: IVec2,
    width: u32,
    height: u32,
    grid_size: u32,
}

impl Game {
    /// Initialises the game
    pub fn new(width: u32, height: u32, grid_size: u32) -> Game {
        let mut game = Game {
            snakes: vec![Snake::new_with_defaults(IVec2::new(5, 5))],
            fruit: IVec2::new(0, 0),
            width,
            height,
            grid_size,
        };

        game.fruit = game.rand_grid_point();
        game
    }

    /// Draws the game
    pub fn draw(&mut self) {
        // Draw fruit
        let fruit_color = Color::from_rgba(0xaa, 0x30, 0x30, 0xff);
        let (x, y, w, h) = self.point_to_rect(self.fruit);
        draw_rectangle(x, y, w, h, fruit_color);

        // Draw snakes
        let snake_color = Color::from_rgba(0x60, 0xAA, 0x60, 0xff);
        for snake in &self.snakes {
            let head = snake.get_head();
            let (x, y, w, h) = self.point_to_rect(head);
            draw_rectangle(x, y, w, h, snake_color);

            let tail_components = snake.tail_to_points();
            for &component in &tail_components {
                let (x, y, w, h) = self.point_to_rect(component);
                draw_rectangle(x, y, w, h, snake_color);
            }
        }
    }

    fn point_to_rect(&self, point: IVec2) -> (f32, f32, f32, f32) {
        (
            (self.grid_size as i32 * point.x) as f32,
            (self.grid_size as i32 * point.y) as f32,
            self.grid_size as f32,
            self.grid_size as f32,
        )
    }

    /// Updates the game using the time elapsed since the last update
    pub fn update(&mut self, elapsed_time: f32) {
        for i in 0..self.snakes.len() {
            self.snakes[i].update(elapsed_time);
            let collision = self.snakes[i].check_collision(
                self.width,
                self.height,
                &self.snakes[i].tail_to_points(),
            );

            if collision {
                self.snakes[i].dead = true;
            }

            let head = self.snakes[i].get_head();
            if head == self.fruit {
                self.snakes[i].score += 10;
                self.snakes[i].add_segment();
                self.new_fruit();
            }
        }
    }

    pub fn key_down(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Up => self.snakes[0].set_move(Move::Up),
            KeyCode::Down => self.snakes[0].set_move(Move::Down),
            KeyCode::Left => self.snakes[0].set_move(Move::Left),
            KeyCode::Right => self.snakes[0].set_move(Move::Right),
            _ => {}
        }
    }

    // Generates a random point on the grid
    fn rand_grid_point(&self) -> IVec2 {
        IVec2::new(
            (thread_rng().gen::<u32>() % self.width) as i32,
            (thread_rng().gen::<u32>() % self.height) as i32,
        )
    }

    /// Randomizes the position of the fruit
    pub fn new_fruit(&mut self) {
        // FIXME: snakes should return iterators that iterate through their
        //        components instead of allocating vectors.
        let mut walls = vec![];
        for snake in &self.snakes {
            walls.extend(snake.tail_to_points());
            walls.push(snake.get_head());
        }

        // Move until the fruit is not covered by the snake
        while walls.iter().any(|&w| self.fruit == w) {
            self.fruit = self.rand_grid_point();
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut macroquad::miniquad::Context) {
        self.update(get_frame_time())
    }

    fn draw(&mut self, _ctx: &mut macroquad::miniquad::Context) {
        self.draw()
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut macroquad::miniquad::Context,
        keycode: KeyCode,
        _keymods: macroquad::miniquad::KeyMods,
        _repeat: bool,
    ) {
        self.key_down(keycode)
    }
}
