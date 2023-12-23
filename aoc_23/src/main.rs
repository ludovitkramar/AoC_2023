use std::collections::HashSet;

fn main() {
    let input = include_str!("example");

    let max_length = part_one(input);
    println!("The longest route was {} steps.", max_length);

    let max_length = part_two(input);
    println!("The longest going up the splippery slopes: {}", max_length);
}

#[test]
fn test() {
    let example = include_str!("example");
    let one = part_one(example);
    assert_eq!(one, 94);

    let two = part_two(example);
    assert_eq!(two, 154);

    let input = include_str!("input");
    let one = part_one(input);
    assert_eq!(one, 2362);
}

fn part_two(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut cost_map = map
        .iter()
        .map(|row| row.iter().map(|_| 0).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    let mut start = Position { row: 0, col: 0 };
    let first_row = map.first().unwrap();
    for (col, char) in first_row.iter().enumerate() {
        if *char == '.' {
            start = Position { row: 0, col };
            break;
        }
    }

    let mut goal = Position { row: 0, col: 0 };
    let last_row = map.last().unwrap();
    for (col, char) in last_row.iter().enumerate() {
        if *char == '.' {
            goal = Position {
                row: map.len() - 1,
                col,
            };
            break;
        }
    }

    let mut routes = vec![Route::new(start)];
    let mut max_length = 0;

    loop {
        routes.retain(|route| !route.dead_end && !route.completed);
        let can_continue = routes.len() > 0;

        if !can_continue {
            break;
        }

        let routes_count = routes.len();
        println!("Exploring {} different routes.", routes_count);

        let mut new_splits = Vec::new();
        for route in routes.iter_mut() {
            route.visited.insert(route.current);

            let current_route_length = route.visited.len() as u32;
            let current_max_length = cost_map[route.current.row][route.current.col];

            // println!(
            //     "Current length: {}. Previous Max Length: {}",
            //     current_route_length, current_max_length
            // );

            if current_max_length > current_route_length {
                route.dead_end = true;
                continue;
            }

            cost_map[route.current.row][route.current.col] = current_route_length;

            if route.completed || route.dead_end {
                panic!();
            }

            if route.current == goal {
                route.completed = true;
                max_length = std::cmp::max(max_length, route.visited.len() - 1);
                // println!("One route reached the goal.");
                continue;
            }

            let mut next_positions = Vec::new();
            for pos in route.current.get_surrounding() {
                let tile = map[pos.row][pos.col];

                if route.visited.contains(&pos) {
                    continue;
                }

                if tile != '#' {
                    next_positions.push(pos);
                }
            }

            match next_positions.len() {
                0 => {
                    route.dead_end = true;
                }
                1 => {
                    route.current = *next_positions.first().unwrap();
                }
                _ => {
                    for (i, pos) in next_positions.iter().enumerate() {
                        if i == 0 {
                            route.current = *pos;
                        } else {
                            let mut split = route.clone();
                            split.current = *pos;
                            new_splits.push(split);
                        }
                    }
                }
            }
        }

        for split in new_splits {
            routes.push(split);
        }
    }

    max_length
}

fn part_one(input: &str) -> usize {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut start = Position { row: 0, col: 0 };
    let first_row = map.first().unwrap();
    for (col, char) in first_row.iter().enumerate() {
        if *char == '.' {
            start = Position { row: 0, col };
            break;
        }
    }

    let mut goal = Position { row: 0, col: 0 };
    let last_row = map.last().unwrap();
    for (col, char) in last_row.iter().enumerate() {
        if *char == '.' {
            goal = Position {
                row: map.len() - 1,
                col,
            };
            break;
        }
    }

    println!("Start: {:?}. Goal: {:?}", start, goal);

    let mut routes = vec![Route::new(start)];

    loop {
        routes.retain(|route| !route.dead_end);
        let can_continue = routes.iter().any(|route| !route.completed);

        if !can_continue {
            break;
        }

        let mut new_splits = Vec::new();
        for route in routes.iter_mut() {
            route.visited.insert(route.current);

            if route.current == goal {
                route.completed = true;
                continue;
            }

            let next_positions = route.get_next_positions(&map);

            match next_positions.len() {
                0 => {
                    route.dead_end = true;
                }
                1 => {
                    route.current = *next_positions.first().unwrap();
                }
                _ => {
                    for (i, pos) in next_positions.iter().enumerate() {
                        if i == 0 {
                            route.current = *pos;
                        } else {
                            let mut split = route.clone();
                            split.current = *pos;
                            new_splits.push(split);
                        }
                    }
                }
            }
        }

        for split in new_splits {
            routes.push(split);
        }
    }

    let mut max_length = 0;
    for route in routes {
        let length = route.visited.len() - 1;
        if route.completed {
            println!("Completed route. Length: {}", length);
            max_length = std::cmp::max(max_length, length);
        }
    }

    max_length
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Route {
    current: Position,
    visited: HashSet<Position>,
    completed: bool,
    dead_end: bool,
}

impl Route {
    fn new(start: Position) -> Self {
        Route {
            current: start,
            visited: HashSet::new(),
            completed: false,
            dead_end: false,
        }
    }

    fn get_next_positions(&self, map: &Vec<Vec<char>>) -> Vec<Position> {
        let mut next_positions = Vec::new();
        for pos in self.current.get_surrounding() {
            let tile = map[pos.row][pos.col];

            if self.visited.contains(&pos) {
                continue;
            }

            match tile {
                '.' => {
                    next_positions.push(pos);
                }
                '>' => {
                    if !self.visited.contains(&pos.move_steps(1, 0)) {
                        next_positions.push(pos);
                    }
                }
                '<' => {
                    if !self.visited.contains(&pos.move_steps(-1, 0)) {
                        next_positions.push(pos);
                    }
                }
                '^' => {
                    if !self.visited.contains(&pos.move_steps(0, -1)) {
                        next_positions.push(pos);
                    }
                }
                'v' => {
                    if !self.visited.contains(&pos.move_steps(0, 1)) {
                        next_positions.push(pos);
                    }
                }

                _ => {}
            }
        }
        next_positions
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    /// y
    row: usize,
    /// x
    col: usize,
}

impl Position {
    fn get_surrounding(&self) -> Vec<Position> {
        let mut surrounding = vec![self.move_steps(1, 0), self.move_steps(0, 1)];

        if self.row > 0 {
            surrounding.push(self.move_steps(0, -1));
        }

        if self.col > 0 {
            surrounding.push(self.move_steps(-1, 0));
        }

        surrounding
    }

    fn move_steps(&self, x: i32, y: i32) -> Position {
        Position {
            row: (self.row as i32 + y).try_into().unwrap(),
            col: (self.col as i32 + x).try_into().unwrap(),
        }
    }
}
