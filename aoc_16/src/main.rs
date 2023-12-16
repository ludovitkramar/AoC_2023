fn main() {
    let input = include_str!("input");

    let answer = part_one(input);
    println!("Part one: {}", answer);

    let answer = part_two(input);
    println!("Part two: {}", answer);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
    facing: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn to_bitflag(&self) -> u8 {
        match self {
            Direction::Right => status::RIGHT,
            Direction::Left => status::LEFT,
            Direction::Up => status::UP,
            Direction::Down => status::DOWN,
        }
    }
}

mod status {
    pub const UNVISITED: u8 = 0x00;
    pub const RIGHT: u8 = 0x01;
    pub const LEFT: u8 = 0x02;
    pub const UP: u8 = 0x04;
    pub const DOWN: u8 = 0x08;
}

struct Map {
    data: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

fn read(input: &str) -> Map {
    let data: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let height = data.len();
    let width = data.first().unwrap().len();

    Map {
        data,
        height,
        width,
    }
}

fn part_one(input: &str) -> usize {
    let map = read(input);

    let starting_pos = Position {
        x: 0,
        y: 0,
        facing: Direction::Right,
    };

    count_energized_from_pos(starting_pos, &map)
}

fn part_two(input: &str) -> usize {
    let map = read(input);
    let mut max = 0;

    // Top edge
    for i in 0..map.width {
        let starting_pos = Position {
            x: i,
            y: 0,
            facing: Direction::Down
        };
        let energized_count = count_energized_from_pos(starting_pos, &map);
        max = std::cmp::max(max, energized_count);
    }

    // Bottom edge
    for i in 0..map.width {
        let starting_pos = Position {
            x: i,
            y: map.height - 1,
            facing: Direction::Up
        };
        let energized_count = count_energized_from_pos(starting_pos, &map);
        max = std::cmp::max(max, energized_count);
    }

    // Left edge
    for i in 0..map.height {
        let starting_pos = Position {
            x: 0,
            y: i,
            facing: Direction::Right
        };
        let energized_count = count_energized_from_pos(starting_pos, &map);
        max = std::cmp::max(max, energized_count);
    }

    // Right edge
    for i in 0..map.height {
        let starting_pos = Position {
            x: map.width - 1,
            y: i,
            facing: Direction::Left
        };
        let energized_count = count_energized_from_pos(starting_pos, &map);
        max = std::cmp::max(max, energized_count);
    }

    max
}

fn count_energized_from_pos(starting_pos: Position, map: &Map) -> usize {
    let mut energized = vec![vec![status::UNVISITED; map.width]; map.height];

    let mut current_positions = vec![starting_pos];

    while take_step(&mut current_positions, &mut energized, &map) {}

    let count = energized
        .iter()
        .flat_map(|x| x.iter().filter(|&&status| status != status::UNVISITED))
        .count();

    count
}

/// Take a step in the map, if all the current positions have been visited before (in the same direction), return false.
fn take_step(
    current_positions: &mut Vec<Position>,
    energized: &mut Vec<Vec<u8>>,
    map: &Map,
) -> bool {
    let mut end_positions = Vec::new();

    // Filter out dead-end positions.
    for position in current_positions.iter() {
        let facing_dir = position.facing.to_bitflag();
        let tile_status = energized[position.y][position.x];

        if tile_status == status::UNVISITED {
            // If position is fully unvisited tile.
            energized[position.y][position.x] = facing_dir;
        } else if tile_status & facing_dir != 0 {
            // If this tile has been visited from the SAME direction, stop.
            end_positions.push(*position);
            continue;
        } else {
            // Combine the tile status bitflag.
            energized[position.y][position.x] = tile_status | facing_dir;
        }
    }

    current_positions.retain(|pos| !end_positions.contains(&pos));

    // Advance to the next positions.
    let mut new_positions = Vec::new();
    for position in current_positions.iter() {
        let tile = map.data[position.y][position.x];
        match tile {
            '.' => match position.facing {
                Direction::Right => new_positions.push(Position {
                    x: position.x + 1,
                    y: position.y,
                    facing: Direction::Right,
                }),
                Direction::Left => {
                    if position.x > 0 {
                        new_positions.push(Position {
                            x: position.x - 1,
                            y: position.y,
                            facing: Direction::Left,
                        })
                    }
                }
                Direction::Up => {
                    if position.y > 0 {
                        new_positions.push(Position {
                            x: position.x,
                            y: position.y - 1,
                            facing: Direction::Up,
                        })
                    }
                }
                Direction::Down => new_positions.push(Position {
                    x: position.x,
                    y: position.y + 1,
                    facing: Direction::Down,
                }),
            },
            '|' => match position.facing {
                Direction::Right | Direction::Left => {
                    if position.y > 0 {
                        new_positions.push(Position {
                            x: position.x,
                            y: position.y - 1,
                            facing: Direction::Up,
                        })
                    }
                    new_positions.push(Position {
                        x: position.x,
                        y: position.y + 1,
                        facing: Direction::Down,
                    })
                }
                Direction::Up => {
                    if position.y > 0 {
                        new_positions.push(Position {
                            x: position.x,
                            y: position.y - 1,
                            facing: Direction::Up,
                        })
                    }
                }
                Direction::Down => new_positions.push(Position {
                    x: position.x,
                    y: position.y + 1,
                    facing: Direction::Down,
                }),
            },
            '-' => match position.facing {
                Direction::Right => new_positions.push(Position {
                    x: position.x + 1,
                    y: position.y,
                    facing: Direction::Right,
                }),
                Direction::Left => {
                    if position.x > 0 {
                        new_positions.push(Position {
                            x: position.x - 1,
                            y: position.y,
                            facing: Direction::Left,
                        })
                    }
                }
                Direction::Up | Direction::Down => {
                    if position.x > 0 {
                        new_positions.push(Position {
                            x: position.x - 1,
                            y: position.y,
                            facing: Direction::Left,
                        })
                    }
                    new_positions.push(Position {
                        x: position.x + 1,
                        y: position.y,
                        facing: Direction::Right,
                    })
                }
            },
            '\\' => match position.facing {
                Direction::Right => new_positions.push(Position {
                    x: position.x,
                    y: position.y + 1,
                    facing: Direction::Down,
                }),
                Direction::Left => {
                    if position.y > 0 {
                        new_positions.push(Position {
                            x: position.x,
                            y: position.y - 1,
                            facing: Direction::Up,
                        })
                    }
                }
                Direction::Up => {
                    if position.x > 0 {
                        new_positions.push(Position {
                            x: position.x - 1,
                            y: position.y,
                            facing: Direction::Left,
                        })
                    }
                }
                Direction::Down => new_positions.push(Position {
                    x: position.x + 1,
                    y: position.y,
                    facing: Direction::Right,
                }),
            },
            '/' => match position.facing {
                Direction::Right => {
                    if position.y > 0 {
                        new_positions.push(Position {
                            x: position.x,
                            y: position.y - 1,
                            facing: Direction::Up,
                        })
                    }
                }
                Direction::Left => new_positions.push(Position {
                    x: position.x,
                    y: position.y + 1,
                    facing: Direction::Down,
                }),
                Direction::Up => new_positions.push(Position {
                    x: position.x + 1,
                    y: position.y,
                    facing: Direction::Right,
                }),
                Direction::Down => {
                    if position.x > 0 {
                        new_positions.push(Position {
                            x: position.x - 1,
                            y: position.y,
                            facing: Direction::Left,
                        })
                    }
                }
            },
            _ => panic!(),
        }        
    }    

    // Update current positions
    current_positions.clear();
    for position in new_positions {
        current_positions.push(position);
    }

    // Retain only positions can be in bounds.
    current_positions.retain(|pos| {
        match pos.facing {
            Direction::Right => pos.x < map.width,
            Direction::Down => pos.y < map.height,
            // usize can't be negative
            _ => true,
        }
    });

    current_positions.len() > 0
}

#[test]
fn test() {
    let example = include_str!("example");
    let input = include_str!("input");

    let answer = part_one(example);
    assert_eq!(answer, 46);

    let answer = part_one(input);
    assert_eq!(answer, 6994);
    
    let answer = part_two(example);
    assert_eq!(answer, 51);

    let answer = part_two(input);
    assert_eq!(answer, 7488);
}