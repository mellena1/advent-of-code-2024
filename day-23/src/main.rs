use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let network = read_input("input.txt").expect("failed to read input");
    println!("Part 1: {}", part1(&network));
}

fn part1(network: &NetworkMap) -> usize {
    network
        .get_set_of_three_conns()
        .iter()
        .filter(|conns| conns.iter().any(|comp| comp.starts_with("t")))
        .count()
}

fn read_input(path: &str) -> Result<NetworkMap, std::io::Error> {
    let input = read_to_string(&path)?;
    Ok(NetworkMap::from(input.as_str()))
}

struct NetworkMap {
    connections: Vec<(String, String)>,
}

impl NetworkMap {
    fn get_set_of_three_conns(&self) -> Vec<[String; 3]> {
        let conn_map = self.get_connection_map();

        let comps: Vec<_> = conn_map.keys().collect();

        let mut set = Vec::new();

        for (i, comp) in comps.iter().enumerate() {
            let conns_with_this_comp_opt = conn_map.get(*comp);
            if conns_with_this_comp_opt == None {
                continue;
            }
            let conns_with_this_comp = conns_with_this_comp_opt.unwrap();

            if i == comps.len() - 2 {
                break;
            }

            for (j, comp2) in conns_with_this_comp.iter().enumerate() {
                if j == conns_with_this_comp.len() - 1 {
                    break;
                }

                // already looked at this earlier
                if comps.iter().position(|c| *c == comp2).unwrap() < i {
                    continue;
                }

                for comp3 in &conns_with_this_comp[j + 1..] {
                    // already looked at this earlier
                    if comps.iter().position(|c| *c == comp3).unwrap() < i {
                        continue;
                    }

                    if let Some(conns_with_comp2) = conn_map.get(comp2) {
                        if conns_with_comp2.contains(comp3) {
                            set.push([comp.to_string(), comp2.to_string(), comp3.to_string()]);
                        }
                    }
                }
            }
        }

        set
    }

    fn get_connection_map(&self) -> HashMap<String, Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        self.connections.iter().for_each(|conn| {
            let (left, right) = conn;

            if let Some(cur_list) = map.get_mut(left) {
                cur_list.push(right.clone());
            } else {
                map.insert(left.clone(), vec![right.clone()]);
            }

            if let Some(cur_list) = map.get_mut(right) {
                cur_list.push(left.clone());
            } else {
                map.insert(right.clone(), vec![left.clone()]);
            }
        });

        map
    }
}

impl From<&str> for NetworkMap {
    fn from(value: &str) -> Self {
        let connections = value
            .lines()
            .map(|line| {
                let (left, right) = line.split_once("-").expect("must have a hyphen");
                (left.into(), right.into())
            })
            .collect();

        Self { connections }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let network = read_input("example.txt").expect("failed to read input");
        assert_eq!(part1(&network), 7);
    }
}
