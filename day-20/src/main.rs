use core::panic;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
};

use grid::Point;

fn main() {
    let mut map = read_input("input.txt").expect("failed to read input");
    println!("Part 1: {}", part1(&mut map, 100));
}

fn part1(map: &mut Map, save_at_least: usize) -> usize {
    let cheat_spots = map.get_possible_one_move_cheat_spots();
    let non_cheating_dist = map.dijkstra();

    cheat_spots
        .iter()
        .filter(|p| {
            map.grid[p.y][p.x] = GridObject::Empty;
            let dist_with_cheat = map.dijkstra();
            map.grid[p.y][p.x] = GridObject::Wall;

            non_cheating_dist - dist_with_cheat >= save_at_least
        })
        .count()
}

fn read_input(path: &str) -> Result<Map, std::io::Error> {
    let input = read_to_string(&path)?;
    Ok(Map::from(input.as_str()))
}

struct Map {
    grid: Vec<Vec<GridObject>>,
    start_loc: Point,
    end_loc: Point,
}

impl Map {
    fn dijkstra(&self) -> usize {
        let mut dist: HashMap<Point, usize> = (0..self.grid[0].len())
            .flat_map(|x| (0..self.grid.len()).map(move |y| (Point { x, y }, usize::MAX)))
            .collect();

        let mut heap = BinaryHeap::new();

        dist.insert(self.start_loc, 0);
        heap.push(State {
            cost: 0,
            position: self.start_loc,
        });

        while let Some(State { cost, position }) = heap.pop() {
            if position == self.end_loc {
                return cost;
            }

            if cost > *dist.get(&position).unwrap() {
                continue;
            }

            for neighbor in self.neighbors(&position) {
                let dist_to_neighbor = dist.get(&position).unwrap() + 1;
                if dist_to_neighbor < *dist.get(&neighbor).unwrap() {
                    dist.insert(neighbor, dist_to_neighbor);
                    heap.push(State {
                        cost: dist_to_neighbor,
                        position: neighbor,
                    });
                }
            }
        }

        *dist.get(&self.end_loc).unwrap()
    }

    fn neighbors(&self, p: &Point) -> Vec<Point> {
        (-1_i64..=1)
            .flat_map(|i| {
                (-1_i64..=1).filter_map(move |j| {
                    if i != 0 && j != 0 {
                        return None; // can't move diagonally
                    }

                    let new_x = p.x as i64 + j;
                    let new_y = p.y as i64 + i;

                    if self.is_in_bounds(new_x, new_y) {
                        let p = Point::new(new_x, new_y);
                        if self.grid[p.y][p.x] == GridObject::Empty {
                            Some(p)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn is_in_bounds(&self, x: i64, y: i64) -> bool {
        x >= 0 && x < self.grid[0].len() as i64 && y >= 0 && y < self.grid.len() as i64
    }

    fn get_possible_one_move_cheat_spots(&self) -> Vec<Point> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate().filter_map(move |(j, obj)| {
                    if *obj != GridObject::Wall {
                        return None;
                    }

                    let p = Point { x: j, y: i };
                    let neighbors = self.neighbors(&p);

                    if neighbors.len() < 2 || neighbors.len() > 3 {
                        return None;
                    }

                    for i in 0..neighbors.len() {
                        for j in i + 1..neighbors.len() {
                            if neighbors[i].x == neighbors[j].x || neighbors[i].y == neighbors[j].y
                            {
                                return Some(p);
                            }
                        }
                    }

                    None
                })
            })
            .collect()
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut start_loc = Point { x: 0, y: 0 };
        let mut end_loc = Point { x: 0, y: 0 };
        let grid = value
            .lines()
            .enumerate()
            .map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| match c {
                        'S' => {
                            start_loc = Point { x: j, y: i };
                            GridObject::Empty
                        }
                        'E' => {
                            end_loc = Point { x: j, y: i };
                            GridObject::Empty
                        }
                        _ => GridObject::from(c),
                    })
                    .collect()
            })
            .collect();

        Self {
            grid,
            start_loc,
            end_loc,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum GridObject {
    Wall,
    Empty,
}

impl From<char> for GridObject {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            '.' => Self::Empty,
            _ => panic!("invalid char for gridobject"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Point,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let mut map = read_input("example.txt").expect("failed to read input");
        assert_eq!(part1(&mut map, 20), 5);
        assert_eq!(part1(&mut map, 12), 8);
        assert_eq!(part1(&mut map, 10), 10);
    }
}
