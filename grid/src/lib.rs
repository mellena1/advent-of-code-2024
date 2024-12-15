use strum::EnumIter;

#[derive(EnumIter)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self {
            x: x as usize,
            y: y as usize,
        }
    }

    pub fn add_direction(&self, dir: Direction) -> (i64, i64) {
        let x = self.x as i64;
        let y = self.y as i64;
        match dir {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }
}

pub fn new_point_if_in_bounds<T>(grid: &Vec<Vec<T>>, x: i64, y: i64) -> Option<Point> {
    if x < 0 || y < 0 || x >= grid[0].len() as i64 || y >= grid.len() as i64 {
        None
    } else {
        Some(Point::new(x, y))
    }
}
