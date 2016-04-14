use rand::{thread_rng, Rng};

use sdl2::render;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use basic2d::Vec2;

use snake::{Snake, Move};

// Game structure
pub struct Game {
    pub snakes: Vec<Snake>,
    fruit: Vec2<i32>,
    width: u32,
    height: u32,
    grid_size: u32
}

impl Game {
    /// Initialises the game
    pub fn new(width: u32, height: u32, grid_size: u32) -> Game {
        let mut game = Game {
            snakes: vec![Snake::new_with_defaults(Vec2::new(5, 5))],
            fruit: Vec2::new(0, 0),
            width: width,
            height: height,
            grid_size: grid_size
        };

        game.fruit = game.rand_grid_point();
        game
    }

    /// Draws the game
    pub fn draw(&mut self, renderer: &mut render::Renderer) {
        // Draw fruit
        renderer.set_draw_color(Color::RGB(0xAA, 0x30, 0x30));
        renderer.fill_rect(self.point_to_rect(self.fruit)).unwrap();

        // Draw snakes
        renderer.set_draw_color(Color::RGB(0x60, 0xAA, 0x60));
        for snake in &self.snakes {
            let head = snake.get_head();
            renderer.fill_rect(self.point_to_rect(head)).unwrap();

            let tail_components = snake.tail_to_points();
            for &component in &tail_components {
                renderer.fill_rect(self.point_to_rect(component)).unwrap();
            }
        }
    }

    fn point_to_rect(&self, point: Vec2<i32>) -> Rect {
        Rect::new(
            self.grid_size as i32 * point.x,
            self.grid_size as i32 * point.y,
            self.grid_size,
            self.grid_size,
        )
    }

    /// Updates the game using the time elapsed since the last update
    pub fn update(&mut self, elapsed_time: f32) {
        for i in 0..self.snakes.len() {
            self.snakes[i].update(elapsed_time);
            let collision = self.snakes[i].check_collision(self.width, self.height,
                &self.snakes[i].tail_to_points());

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

    pub fn key_down(&mut self, keycode: Keycode) {
        match keycode {
            Keycode::Up => self.snakes[0].set_move(Move::Up),
            Keycode::Down => self.snakes[0].set_move(Move::Down),
            Keycode::Left => self.snakes[0].set_move(Move::Left),
            Keycode::Right => self.snakes[0].set_move(Move::Right),
            _ => {},
        }
    }

    // Generates a random point on the grid
    fn rand_grid_point(&self) -> Vec2<i32> {
       Vec2::new(
            (thread_rng().gen::<u32>() % self.width) as i32,
            (thread_rng().gen::<u32>() % self.height) as i32
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
