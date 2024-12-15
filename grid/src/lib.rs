use strum::EnumIter;

#[derive(EnumIter, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn translate(&self, x: i64, y: i64) -> (i64, i64) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }
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

    pub fn add_direction(&self, dir: &Direction) -> (i64, i64) {
        let x = self.x as i64;
        let y = self.y as i64;
        dir.translate(x, y)
    }

    pub fn add_diagonal_direction(&self, dir: (&Direction, &Direction)) -> (i64, i64) {
        let mut x = self.x as i64;
        let mut y = self.y as i64;

        (x, y) = dir.0.translate(x, y);
        dir.1.translate(x, y)
    }
}

pub fn new_point_if_in_bounds<T>(grid: &Vec<Vec<T>>, x: i64, y: i64) -> Option<Point> {
    if x < 0 || y < 0 || x >= grid[0].len() as i64 || y >= grid.len() as i64 {
        None
    } else {
        Some(Point::new(x, y))
    }
}
