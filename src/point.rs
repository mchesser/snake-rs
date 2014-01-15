#[deriving(Eq, Clone)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}


impl Add<Point, Point> for Point {
    fn add(&self, rhs: &Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Point, Point> for Point {
    fn sub(&self, rhs: &Point) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}


#[cfg(test)]
mod tests {
    use point::Point;
    
    #[test]
    fn test_eq() {
        assert!(Point::new(1, 1) == Point::new(1, 1));
    }
    
    #[test]
    fn test_add() {
        assert_eq!(Point::new(1, 1) + Point::new(2, 1), Point::new(3, 2));
    }
    
    #[test]
    fn test_sub() {
        assert_eq!(Point::new(1, 1) - Point::new(1, 1), Point::new(0, 0));
    }
    
}