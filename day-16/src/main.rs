use std::{collections::HashSet, fs::read_to_string};

use grid::{Direction, Point};
use petgraph::{algo::dijkstra, prelude::DiGraphMap};
use strum::IntoEnumIterator;

fn main() {
    let map = read_input("input.txt").expect("failed to read input");
    println!("Part 1: {}", part1(&map));
}

fn part1(map: &Map) -> u64 {
    map.lowest_score()
}

fn read_input(path: &str) -> Result<Map, std::io::Error> {
    let input = read_to_string(path)?;
    Ok(Map::from(input.as_str()))
}

struct Map {
    graph: DiGraphMap<Node, u64>,
    starting_node: Node,
    ending_nodes: Vec<Node>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let raw_map: Vec<Vec<char>> = value.lines().map(|line| line.chars().collect()).collect();
        Self::new(&raw_map)
    }
}

impl Map {
    fn new(raw_map: &Vec<Vec<char>>) -> Self {
        let corners = find_corners_in_raw_map(&raw_map);
        let edges = find_edges(raw_map, &corners);

        let mut graph = DiGraphMap::new();
        let starting_node = Node {
            pos: Point {
                x: 1,
                y: raw_map.len() - 2,
            },
            reindeer_dir: Direction::Right,
        };
        let mut ending_nodes = Vec::new();

        edges.iter().for_each(|(c1, c2)| {
            let dir_to_c2 = c1.direction_to_point(c2);

            let c2_node = Node {
                pos: c2.clone(),
                reindeer_dir: dir_to_c2,
            };

            if raw_map[c2.y][c2.x] == 'E' {
                ending_nodes.push(c2_node);
            }

            Direction::iter().for_each(|dir| {
                let cost_of_turn = dir.turns_to_other_dir(&dir_to_c2) * 1000;
                let cost_of_walking = c1.distance_to_point(&c2);

                let c1_node = Node {
                    pos: c1.clone(),
                    reindeer_dir: dir,
                };

                graph.add_edge(c1_node, c2_node, cost_of_turn + cost_of_walking);
            });
        });

        Self {
            graph,
            starting_node,
            ending_nodes,
        }
    }

    fn lowest_score(&self) -> u64 {
        self.ending_nodes
            .iter()
            .filter_map(|end_node| {
                let result = dijkstra(
                    &self.graph,
                    self.starting_node,
                    Some(*end_node),
                    |(_, _, cost)| *cost,
                );
                result.get(end_node).and_then(|v| Some(v.clone()))
            })
            .min()
            .expect("should have a min weight")
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Node {
    pos: Point,
    reindeer_dir: Direction,
}

fn find_corners_in_raw_map(raw_map: &Vec<Vec<char>>) -> Vec<Point> {
    raw_map
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut corners, (i, row)| {
            row.iter().enumerate().for_each(|(j, c)| {
                if *c == '#' {
                    return;
                }

                let point = Point { x: j, y: i };

                if point_is_dead_end(raw_map, &point) || point_is_intersection(raw_map, &point) {
                    corners.push(point);
                }
            });
            corners
        })
}

fn point_is_intersection(raw_map: &Vec<Vec<char>>, point: &Point) -> bool {
    [
        [Direction::Up, Direction::Right],
        [Direction::Up, Direction::Left],
        [Direction::Down, Direction::Right],
        [Direction::Down, Direction::Left],
    ]
    .iter()
    .any(|corner_dirs| {
        corner_dirs.iter().all(|dir| {
            let (x, y) = point.add_direction(dir);
            raw_map[y as usize][x as usize] != '#'
        })
    })
}

fn point_is_dead_end(raw_map: &Vec<Vec<char>>, point: &Point) -> bool {
    Direction::iter()
        .filter(|dir| {
            let (x, y) = point.add_direction(dir);
            raw_map[y as usize][x as usize] == '#'
        })
        .collect::<Vec<_>>()
        .len()
        == 3
}

fn find_edges(raw_map: &Vec<Vec<char>>, corners: &Vec<Point>) -> HashSet<(Point, Point)> {
    corners.iter().fold(HashSet::new(), |mut edges, corner| {
        let connected_corners = find_corners_connecting_to_corner(raw_map, corners, corner);
        connected_corners.iter().for_each(|connected_corner| {
            edges.insert((*corner, *connected_corner));
        });
        edges
    })
}

fn find_corners_connecting_to_corner(
    raw_map: &Vec<Vec<char>>,
    corners: &Vec<Point>,
    start_corner: &Point,
) -> Vec<Point> {
    Direction::iter()
        .filter_map(|dir| {
            let mut cur_pos = start_corner.clone();

            loop {
                let (x, y) = cur_pos.add_direction(&dir);
                cur_pos = Point::new(x, y);

                if raw_map[cur_pos.y][cur_pos.x] == '#' {
                    return None;
                }

                if corners.contains(&cur_pos) {
                    return Some(cur_pos);
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let map = read_input("example.txt").expect("failed to read input");
        assert_eq!(part1(&map), 7036);
    }
}
