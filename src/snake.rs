use macroquad::prelude::IVec2;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    pos: IVec2,
    tail: Vec<Move>,
    current_move: Move,
    next_move: Move,
    move_delay: f32,
    wait_time: f32,

    pub score: u64,
    pub dead: bool,
}

impl Snake {
    /// Initialise a snake with 3 segments starting with a specified start
    /// direction and speed
    pub fn new(pos: IVec2, start_dir: Move, move_delay: f32) -> Snake {
        Snake {
            pos,
            tail: vec![start_dir, start_dir, start_dir],
            current_move: start_dir,
            next_move: start_dir,
            move_delay,
            wait_time: 0.0,

            score: 10,
            dead: false,
        }
    }

    /// Initialise a snake with 3 segments with default speed and direction
    pub fn new_with_defaults(pos: IVec2) -> Snake {
        Snake::new(pos, Move::Right, 0.05)
    }

    /// Add a new segment to the end of the snake
    pub fn add_segment(&mut self) {
        let tail = *self.tail.last().unwrap_or(&Move::Right);
        self.tail.push(tail);
    }

    /// Update the snake's position based on the amount of time elapsed since
    /// the last update.
    pub fn update(&mut self, elapsed_time: f32) {
        self.wait_time += elapsed_time;
        if self.wait_time >= self.move_delay {
            self.wait_time -= self.move_delay;

            // Move the head based on the direction
            match self.next_move {
                Move::Up => self.pos.y -= 1,
                Move::Down => self.pos.y += 1,
                Move::Left => self.pos.x -= 1,
                Move::Right => self.pos.x += 1,
            }

            // Move the rest of the components
            for i in (1..self.tail.len()).rev() {
                self.tail[i] = self.tail[i - 1];
            }
            self.tail[0] = self.next_move;

            self.current_move = self.next_move;
        }
    }

    /// Checks for colision with the map edges and any obstacles in the map.
    pub fn check_collision(&self, map_width: u32, map_height: u32, walls: &[IVec2]) -> bool {
        // Check map bounds
        if self.pos.x < 0
            || self.pos.y < 0
            || self.pos.x >= map_width as i32
            || self.pos.y >= map_height as i32
        {
            return true;
        }

        // Check obstacles
        for &wall in walls {
            if self.pos == wall {
                return true;
            }
        }

        false
    }

    /// Get the position of the snake's head.
    pub fn get_head(&self) -> IVec2 {
        self.pos
    }

    /// Sets the snake's next move, if possible
    pub fn set_move(&mut self, next_move: Move) {
        match next_move {
            Move::Up => {
                if self.current_move != Move::Down {
                    self.next_move = Move::Up;
                }
            }
            Move::Down => {
                if self.current_move != Move::Up {
                    self.next_move = Move::Down;
                }
            }
            Move::Left => {
                if self.current_move != Move::Right {
                    self.next_move = Move::Left;
                }
            }
            Move::Right => {
                if self.current_move != Move::Left {
                    self.next_move = Move::Right;
                }
            }
        }
    }

    /// Converts the tail of the snake to a vector of points.
    pub fn tail_to_points(&self) -> Vec<IVec2> {
        let mut acc = Vec::with_capacity(self.tail.len());

        // Note: segment directions point to the position of the next block, not
        // the position of next block to the segment.
        let mut next = self.pos;
        for segment in &self.tail {
            next = match *segment {
                Move::Up => next + IVec2::new(0, 1),
                Move::Down => next + IVec2::new(0, -1),
                Move::Left => next + IVec2::new(1, 0),
                Move::Right => next + IVec2::new(-1, 0),
            };
            acc.push(next);
        }
        acc
    }
}
