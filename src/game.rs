extern mod sdl2;

use sdl2::render;
use sdl2::rect::Rect;
use std::rand::{task_rng, Rng};
use point::Point;

pub mod snake;
mod point;

// Game structure
pub struct Game {
    snakes: ~[snake::Snake],
    fruit: Point,
    width: u32,
    height: u32,
    grid_size: u32
}

impl Game {
    /// Initialises the game
    pub fn init(width: u32, height: u32, grid_size: u32) -> Game {
        let mut game = Game {
            snakes: box [snake::Snake::init_with_defaults(Point::new(5, 5))],
            fruit: Point::new(0, 0),
            width: width,
            height: height,
            grid_size: grid_size
        };

        game.fruit = game.rand_grid_point();
        return game;
    }

    /// Draws the game
    pub fn draw(&mut self, renderer: &render::Renderer) {
        // Draw fruit
        renderer.set_draw_color(sdl2::pixels::RGB(0xAA, 0x30, 0x30));
        renderer.fill_rect(&self.point_to_rect(self.fruit));

        // Draw snakes
        renderer.set_draw_color(sdl2::pixels::RGB(0x60, 0xAA, 0x60));
        for snake in self.snakes.iter() {
            let head = snake.get_head();
            renderer.fill_rect(&self.point_to_rect(head));

            let tail_components = snake.tail_to_points();
            for &component in tail_components.iter() {
                renderer.fill_rect(&self.point_to_rect(component));
            }
        }
    }

    /// Converts a point to an sdl rectangle to be drawn
    /// # Arugments
    /// `point` - the point to convert
    /// # Return
    /// An sdl rectangle
    fn point_to_rect(&self, point: Point) -> Rect {
        Rect::new(
            self.grid_size as i32 * point.x,
            self.grid_size as i32 * point.y,
            self.grid_size as i32,
            self.grid_size as i32
        )
    }


    /// Updates the game using the time elapsed since the last update
    pub fn update(&mut self, elapsed_time: f32) {
        for i in range(0, self.snakes.len()) {
            self.snakes[i].update(elapsed_time);
            let collision = self.snakes[i].check_collision(self.width, self.height,
                self.snakes[i].tail_to_points());

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

    // Generates a random point on the grid
    fn rand_grid_point(&self) -> Point {
       Point::new(
            (task_rng().gen::<u32>() % self.width) as i32,
            (task_rng().gen::<u32>() % self.height) as i32
        )
    }

    /// Randomizes the position of the fruit
    pub fn new_fruit(&mut self) {
        // FIXME: snakes should return iterators that iterate through their
        //        components instead of allocating vectors.
        let mut walls = box [];
        for snake in self.snakes.iter() {
            walls.push_all(snake.tail_to_points());
            walls.push(snake.get_head());
        }

        // Move until the fruit is not covered by the snake
        while walls.iter().any(|&w| self.fruit == w) {
            self.fruit = self.rand_grid_point();
        }
    }
}
