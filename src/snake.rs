use std::vec;

#[deriving(Eq)]
pub enum Move {
    Up,
    Down,
    Left,
    Right
}

struct Snake {
    priv x            : uint,
    priv y            : uint,
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
    pub fn init(x: uint, y: uint, start_dir: Move, move_delay: f32) -> Snake {
        Snake {
            x            : x,
            y            : y,
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
    pub fn init_with_defaults(x: uint, y: uint) -> Snake {
        Snake::init(x, y, RIGHT, 0.05)
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
                Up    => self.y -= 1,
                Down  => self.y += 1,
                Left  => self.x -= 1,
                Right => self.x += 1
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
    pub fn check_collision(&self, map_width: uint, map_height: uint,
            walls: &[[uint,..2]]) -> bool 
    {
        // Check map bounds
        if (self.x < 0 || self.y < 0 ||
                self.x >= map_width || self.y >= map_height) 
        {
            return true;
        }
    
        // Check obstacles
        for wall in walls.iter() {
            if (self.x == wall[0] && self.y == wall[1]) {
                return true;
            }
        }
        
        return false;
    }
    
    /// Get the position of the snake's head.
    pub fn get_head(&self) -> [uint,..2] {
        [self.x, self.y]
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
    pub fn tail_to_points(&self) -> ~[[uint,..2]] {
        let mut acc = vec::with_capacity(self.tail.len());
        
        /* Note: segment directions point to the position of the next block, not
         * the position of next block to the segment. */
        let mut next = [self.x, self.y];
        for segment in self.tail.iter() {
            next = 
                match *segment {
                    Up    => [next[0], next[1] + 1],
                    Down  => [next[0], next[1] - 1],
                    Left  => [next[0] + 1, next[1]],
                    Right => [next[0] - 1, next[1]]
                };
            acc.push(next);
        }
        return acc;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_tail_to_points() {
        let snake = init(10, 10, RIGHT, 0.05);
        assert_eq!(snake.tail_to_points(), ~[[9, 10], [8, 10], [7,10]])
    }
    
    #[test]
    fn test_get_head() {
        let snake = init(10, 10, RIGHT, 0.05);
        assert_eq!(snake.get_head(), [10, 10]);
    }
    
    #[test]
    fn test_update_same_direction() {
        let mut snake = init(10, 10, RIGHT, 1.0);
        snake.update(1.0);
        assert_eq!(snake.get_head(), [11, 10]);
        assert_eq!(snake.tail_to_points(), ~[[10, 10], [9, 10], [8,10]]);
    }
    
    #[test]
    fn test_update_different_direction() {
        let mut snake = init(10, 10, RIGHT, 1.0);
        snake.set_move(DOWN);
        snake.update(1.0);
        assert_eq!(snake.get_head(), [10, 11]);
        assert_eq!(snake.tail_to_points(), ~[[10, 10], [9, 10], [8,10]]);
    }
}
