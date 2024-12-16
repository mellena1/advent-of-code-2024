use std::{fmt, fs::read_to_string};

use grid::{Direction, Point};

fn main() {
    let mut map = read_input("input.txt").expect("failed to read input");
    println!("Part 1: {}", part1(&mut map));
}

fn part1(map: &mut RobotMap) -> u64 {
    map.move_the_robot_all_steps();
    let boxes = map.get_box_locs();

    boxes
        .iter()
        .fold(0, |acc, pos| acc + pos.y as u64 * 100 + pos.x as u64)
}

fn read_input(path: &str) -> Result<RobotMap, std::io::Error> {
    let input = read_to_string(path)?;
    Ok(RobotMap::from(input.as_str()))
}

struct RobotMap {
    map: Vec<Vec<Object>>,
    cur_robot_loc: Point,
    robot_directions: Vec<Direction>,
}

impl RobotMap {
    fn new(value: &str) -> Self {
        let (map_str, dir_str) = value.split_once("\n\n").expect("should be a newline split");

        let map: Vec<_> = map_str
            .lines()
            .map(|line| line.chars().map(|c| Object::from(c)).collect::<Vec<_>>())
            .collect();

        let dirs = dir_str
            .chars()
            .filter_map(|c| {
                if c.is_whitespace() {
                    None
                } else {
                    Some(dir_from_char(c))
                }
            })
            .collect();

        let mut robot_loc = Point { x: 0, y: 0 };
        'outer: for (i, row) in map.iter().enumerate() {
            for (j, obj) in row.iter().enumerate() {
                if *obj == Object::Robot {
                    robot_loc.x = j;
                    robot_loc.y = i;
                    break 'outer;
                }
            }
        }

        Self {
            map,
            cur_robot_loc: robot_loc,
            robot_directions: dirs,
        }
    }

    fn get_box_locs(&self) -> Vec<Point> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().filter_map(move |(j, obj)| {
                    if *obj == Object::Box {
                        Some(Point { x: j, y: i })
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn move_the_robot_all_steps(&mut self) {
        let robot_dirs = self.robot_directions.clone();
        for dir in robot_dirs.iter() {
            self.move_robot(dir);
        }
    }

    fn move_robot(&mut self, dir: &Direction) {
        let next_pos = new_point_unsafe(&self.cur_robot_loc, dir);

        match self.get_obj_at_loc(&next_pos) {
            Object::Empty => {
                let cur_robot_loc = self.cur_robot_loc;
                self.set_obj_at_loc(&cur_robot_loc, Object::Empty);
                self.set_obj_at_loc(&next_pos, Object::Robot);
            }
            Object::Wall => {}
            Object::Box => {
                let mut next_empty_space = next_pos;
                loop {
                    next_empty_space = new_point_unsafe(&next_empty_space, dir);

                    match self.get_obj_at_loc(&next_empty_space) {
                        Object::Empty => break,
                        Object::Wall => return,
                        _ => {}
                    }
                }

                let robot_cur_pos = self.cur_robot_loc;
                let mut prev_space = next_empty_space;
                loop {
                    prev_space = new_point_unsafe(&prev_space, &dir.opposite());
                    next_empty_space = new_point_unsafe(&prev_space, &dir);
                    self.set_obj_at_loc(&next_empty_space, self.get_obj_at_loc(&prev_space));
                    if prev_space == robot_cur_pos {
                        self.set_obj_at_loc(&prev_space, Object::Empty);
                        break;
                    }
                }
            }
            Object::Robot => panic!("the robot should never be able to walk into itself"),
        }
    }

    fn get_obj_at_loc(&self, pos: &Point) -> Object {
        self.map[pos.y][pos.x]
    }

    fn set_obj_at_loc(&mut self, pos: &Point, obj: Object) {
        self.map[pos.y][pos.x] = obj;
        if obj == Object::Robot {
            self.cur_robot_loc = pos.clone();
        }
    }
}

impl From<&str> for RobotMap {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for RobotMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.map.iter().fold(String::new(), |acc, row| {
            acc + &row
                .iter()
                .fold(String::new(), |acc2: String, obj| acc2 + &obj.to_string())
                + "\n"
        });

        write!(f, "{}", s)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Object {
    Wall,
    Box,
    Robot,
    Empty,
}

impl From<char> for Object {
    fn from(value: char) -> Self {
        match value {
            '#' => Object::Wall,
            'O' => Object::Box,
            '@' => Object::Robot,
            '.' => Object::Empty,
            _ => panic!("no idea what this obj is"),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Object::Wall => '#',
                Object::Box => 'O',
                Object::Robot => '@',
                Object::Empty => '.',
            }
        )
    }
}

fn dir_from_char(c: char) -> Direction {
    match c {
        '<' => Direction::Left,
        '>' => Direction::Right,
        'v' => Direction::Down,
        '^' => Direction::Up,
        _ => panic!("no idea what this direction is"),
    }
}

fn new_point_unsafe(p: &Point, dir: &Direction) -> Point {
    let (x, y) = p.add_direction(dir);
    Point::new(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let mut map = read_input("example.txt").expect("failed to read input");
        assert_eq!(part1(&mut map), 10092);
    }
}
