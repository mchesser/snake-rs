extern mod rsfml;
use rsfml::graphics::{RenderWindow, sfClose, Color, RectangleShape};

use std::rand::{task_rng, Rng};

mod snake;

// Game structure
struct Game {
    snakes: ~[snake::Snake],
    fruit : [uint,..2],
    width : uint,
    height: uint,
}

impl Game {
    /// Initialises the game
    pub fn init(width: uint, height: uint) -> Game {
        let mut game = Game {
            snakes: box [snake::Snake::init_with_defaults(5, 5)],
            fruit: [0, 0],
            width: width,
            height: height
        };
        
        game.fruit = self.rand_grid_point();
        return game;
    }
    
    /// Updates the game using the time elapsed since the last update
    pub fn update(&mut self, elapsed_time: f32) {
        self.player.update(elapsed_time);
        let collision = self.player.check_collision(self.width, self.height,
                self.player.tail_to_points());
        
        if collision {
            self.player.dead = true;
        }
        
        let player_head = self.player.get_head();
        if player_head[0] == self.fruit[0] && player_head[1] == self.fruit[1] {
            self.player.score += 10;
            self.player.add_segment();
            self.new_fruit();
        }
    }
    
    fn rand_grid_point(&self) -> [uint,..2] {
        [
            task_rng().gen::<uint>() % self.width,
            task_rng().gen::<uint>() % self.height
        ]
    }
    
    /// Randomizes the position of the fruit
    pub fn new_fruit(&mut self) {
        // FIXME: snakes should return iterators that iterate through their
        //        components instead of allocating vectors.
        let mut walls = box [];
        for snake in self.snakes.iter() {
            walls.append(snake.tail_to_points());
            walls.push(snake.get_head());
        }
        
        // Move until the fruit is not covered by the snake
        while walls.iter().any(|w| self.fruit[0] == w[0] && self.fruit[1] == w[1]) {
            self.fruit = self.rand_grid_point();
        }
    }
}
