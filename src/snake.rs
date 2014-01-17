use std::vec;
use point::Point;

mod point;

#[deriving(Eq)]
pub enum Move {
    Up,
    Down,
    Left,
    Right
}

pub struct Snake {
    priv pos          : Point,
    priv tail         : ~[Move],
    priv current_move : Move,
    priv next_move    : Move,
    priv move_delay   : f32,
    priv wait_time    : f32,
    score             : uint,
    dead              : bool
}

impl Snake {
    /// Initialise a snake with 3 segments starting with a specified start
    /// direction and speed
    pub fn init(pos: Point, start_dir: Move, move_delay: f32) -> Snake {
        Snake {
            pos          : pos,
            tail         : box [start_dir, start_dir, start_dir],
            current_move : start_dir,
            next_move    : start_dir,
            move_delay   : move_delay,
            wait_time    : 0.,
            score        : 10,
            dead         : false
        }
    }

    /// Initialise a snake with 3 segments with default speed and direction
    pub fn init_with_defaults(pos: Point) -> Snake {
        Snake::init(pos, Right, 0.05)
    }

    /// Add a new segment to the end of the snake
    pub fn add_segment(&mut self) {
        let dir = *self.tail.last();
        self.tail.push(dir);
    }

    /// Update the snake's position based on the amount of time elapsed since
    /// the last update.
    pub fn update(&mut self, elapsed_time: f32) {
        self.wait_time += elapsed_time;
        if (self.wait_time >= self.move_delay) {
            self.wait_time -= self.move_delay;

            // Move the head based on the direction
            match self.next_move {
                Up    => self.pos.y -= 1,
                Down  => self.pos.y += 1,
                Left  => self.pos.x -= 1,
                Right => self.pos.x += 1
            }

            // Move the rest of the components
            for i in range(1, self.tail.len()).invert() {
                self.tail[i] = self.tail[i-1];
            }
            self.tail[0] = self.next_move;

            self.current_move = self.next_move;
        }
    }

    /// Checks for colision with the map edges and any obstacles in the map.
    /// # Arguments
    /// `map_width` the width of the map.
    /// `map_height` the height of the map.
    /// `walls` a vector of obstacles in the map.
    ///
    /// # Return
    /// `true` if the snake hits anything, `false` otherwise.
    pub fn check_collision(&self, map_width: u32, map_height: u32,
            walls: &[Point]) -> bool {
        // Check map bounds
        if (self.pos.x < 0 || self.pos.y < 0 ||
                self.pos.x >= map_width as i32|| self.pos.y >= map_height as i32) {
            return true;
        }

        // Check obstacles
        for &wall in walls.iter() {
            if self.pos == wall {
                return true;
            }
        }

        false
    }

    /// Get the position of the snake's head.
    pub fn get_head(&self) -> Point {
        self.pos
    }

    /// Sets the snake's next move, if possible
    pub fn set_move(&mut self, next_move: Move) {
        match next_move {
            Up    => if self.current_move != Down  { self.next_move = Up;    },
            Down  => if self.current_move != Up    { self.next_move = Down;  },
            Left  => if self.current_move != Right { self.next_move = Left;  },
            Right => if self.current_move != Left  { self.next_move = Right; }
        }
    }

    /// Converts the tail of the snake to a vector of points.
    pub fn tail_to_points(&self) -> ~[Point] {
        let mut acc = vec::with_capacity(self.tail.len());

        /* Note: segment directions point to the position of the next block, not
         * the position of next block to the segment. */
        let mut next = self.pos;
        for segment in self.tail.iter() {
            next =
                match *segment {
                    Up    => next + Point::new(0, 1),
                    Down  => next + Point::new(0, -1),
                    Left  => next + Point::new(1, 0),
                    Right => next + Point::new(-1, 0)
                };
            acc.push(next);
        }
        return acc;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use point::Point;

    #[test]
    fn test_tail_to_points() {
        let snake = Snake::init(Point::new(10, 10), Right, 0.05);
        assert_eq!(snake.tail_to_points(),
            ~[Point::new(9, 10), Point::new(8, 10), Point::new(7, 10)])
    }

    #[test]
    fn test_get_head() {
        let snake = Snake::init(Point::new(10, 10), Right, 0.05);
        assert_eq!(snake.get_head(), Point::new(10, 10));
    }

    #[test]
    fn test_update_same_direction() {
        let mut snake = Snake::init(Point::new(10, 10), Right, 1.0);
        snake.update(1.0);
        assert_eq!(snake.get_head(), Point::new(11, 10));
        assert_eq!(snake.tail_to_points(),
            ~[Point::new(10, 10), Point::new(9, 10), Point::new(8, 10)]);
    }

    #[test]
    fn test_update_different_direction() {
        let mut snake = Snake::init(Point::new(10, 10), Right, 1.0);
        snake.set_move(Down);
        snake.update(1.0);
        assert_eq!(snake.get_head(), Point::new(10, 11));
        assert_eq!(snake.tail_to_points(),
            ~[Point::new(10, 10), Point::new(9, 10), Point::new(8, 10)]);
    }
}
