use std::{collections::HashSet, fs::read_to_string};

use strum::{EnumIter, IntoEnumIterator};

fn main() {
    let map = read_input("input.txt").expect("failed to read input");
    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
}

fn part1(map: &TrailMap) -> u64 {
    let trailheads = map.find_trailheads();
    trailheads
        .iter()
        .fold(0, |acc, trailhead| acc + map.get_trailhead_score(trailhead))
}

fn part2(map: &TrailMap) -> u64 {
    let trailheads = map.find_trailheads();
    trailheads.iter().fold(0, |acc, trailhead| {
        acc + map.get_trailhead_rating(trailhead)
    })
}

fn read_input(path: &str) -> Result<TrailMap, std::io::Error> {
    let input = read_to_string(path)?;
    Ok(TrailMap::from(input.as_str()))
}

#[derive(EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self {
            x: x as usize,
            y: y as usize,
        }
    }

    fn add_direction(&self, dir: Direction) -> (i64, i64) {
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

struct TrailMap {
    map: Vec<Vec<u8>>,
}

impl TrailMap {
    fn get_trailhead_score(&self, trailhead: &Point) -> u64 {
        let mut paths = Vec::new();
        self.traverse_to_9s(trailhead, vec![trailhead.clone()], &mut paths);

        paths
            .iter()
            .fold(HashSet::<Point>::new(), |mut acc, path| {
                acc.insert(path.last().expect("path should never be empty").clone());
                acc
            })
            .len() as u64
    }

    fn get_trailhead_rating(&self, trailhead: &Point) -> u64 {
        let mut paths = Vec::new();
        self.traverse_to_9s(trailhead, vec![trailhead.clone()], &mut paths);

        paths.len() as u64
    }

    fn traverse_to_9s(
        &self,
        cur_pos: &Point,
        cur_path: Vec<Point>,
        all_paths: &mut Vec<Vec<Point>>,
    ) {
        let cur_val = self.get_val_at_pos(cur_pos);

        if cur_val == 9 {
            all_paths.push(cur_path);
            return;
        }

        let next_pos_options = Direction::iter().filter_map(|dir| {
            let (x, y) = cur_pos.add_direction(dir);
            if self.is_in_bounds(x, y) {
                Some(Point::new(x, y))
            } else {
                None
            }
        });

        next_pos_options.for_each(|new_pos| {
            if self.get_val_at_pos(&new_pos) == cur_val + 1 {
                let mut new_path = cur_path.clone();
                new_path.push(new_pos.clone());
                self.traverse_to_9s(&new_pos, new_path, all_paths);
            }
        });
    }

    fn find_trailheads(&self) -> Vec<Point> {
        self.map
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut acc, (i, row)| {
                acc.extend(
                    row.iter()
                        .enumerate()
                        .fold(Vec::new(), |mut acc2, (j, obj)| {
                            if *obj == 0 {
                                acc2.push(Point { x: j, y: i });
                            }
                            acc2
                        }),
                );
                acc
            })
    }

    fn get_val_at_pos(&self, pos: &Point) -> u8 {
        self.map[pos.y][pos.x]
    }

    fn is_in_bounds(&self, x: i64, y: i64) -> bool {
        if x < 0 || y < 0 {
            return false;
        }

        x < self.map[0].len() as i64 && y < self.map.len() as i64
    }
}

impl From<&str> for TrailMap {
    fn from(value: &str) -> Self {
        let map = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("should be num") as u8)
                    .collect()
            })
            .collect();
        Self { map }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let map = read_input("example.txt").expect("failed to read input");
        let result = part1(&map);
        assert_eq!(result, 36);
    }

    #[test]
    fn part2_works() {
        let map = read_input("example.txt").expect("failed to read input");
        let result = part2(&map);
        assert_eq!(result, 81);
    }

    #[test]
    fn find_trailheads_works() {
        let map = TrailMap::from(
            "0123\n\
             1234\n\
             8065\n\
             9876",
        );
        let result = map.find_trailheads();
        assert_eq!(&result, &[Point { x: 0, y: 0 }, Point { x: 1, y: 2 }]);
    }
}
