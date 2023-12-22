use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn surrounding(&self) -> Vec<Position> {
        let mut surrounding = vec![
            Position {
                col: self.col + 1,
                row: self.row,
            },
            Position {
                col: self.col,
                row: self.row + 1,
            },
        ];

        if self.row > 0 {
            surrounding.push(Position {
                row: self.row - 1,
                col: self.col,
            })
        }

        if self.col > 0 {
            surrounding.push(Position {
                col: self.col - 1,
                row: self.row,
            })
        }

        surrounding
    }
}

fn main() {
    let input = include_str!("input");

    let count = part_one(input, 64);
    println!("There are {} positions.", count);
}

#[test]
fn test() {
    let example = include_str!("example");
    let count = part_one(example, 6);
    assert_eq!(count, 16);

    let input = include_str!("input");
    let count = part_one(input, 64);
    assert_eq!(count, 3788);
}

fn part_one(input: &str, max_steps: i32) -> usize {
    let data: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut start = Position { row: 0, col: 0 };
    for (row, line) in data.iter().enumerate() {
        for (col, char) in line.iter().enumerate() {
            if *char == 'S' {
                start = Position { row, col };
                println!("Starting at: {:?}", start);
                break;
            }
        }
    }

    let mut current_positions = HashSet::new();
    current_positions.insert(start);

    for _ in 0..max_steps {
        let mut next_positions = Vec::new();

        for value in current_positions.iter() {
            for pos in value.surrounding() {
                let tile = data.get(pos.row).and_then(|row| row.get(pos.col));
                match tile {
                    Some(tile) => {
                        if *tile != '#' {
                            next_positions.push(pos);
                        }
                    }
                    None => {}
                }
            }
        }

        current_positions.clear();
        for pos in next_positions {
            current_positions.insert(pos);
        }
    }

    let count = current_positions.len();
    count
}
