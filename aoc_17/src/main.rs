use std::collections::{BinaryHeap, HashSet};

fn main() {
    let input = include_str!("input");
    let graph = build_graph(input);

    let cost = part_one(&graph);
    println!("Shortest path cost: {}", cost);
}

fn part_one(graph: &Vec<Node>) -> u32 {
    let start = graph.first().unwrap();
    let goal = graph.last().unwrap();

    find_path(graph, start.clone(), goal.clone())
}

#[test]
fn test_example() {
    let example = include_str!("example");
    let example_graph = build_graph(example);

    let cost = part_one(&example_graph);
    assert_eq!(cost, 102);
}

#[test]
fn test_real() {
    let input = include_str!("input");
    let input_graph = build_graph(input);

    let cost = part_one(&input_graph);
    assert_eq!(cost, 1263);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct SearchNode {
    direction: Option<Direction>,
    same_dir_counter: u32,
    actual_cost: u32,
    heuristic_cost: u32,
    node: usize,
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let cost_a = self.actual_cost + self.heuristic_cost;
        let cost_b = other.actual_cost + other.heuristic_cost;

        Some(cost_b.cmp(&cost_a))
    }
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn find_path(graph: &Vec<Node>, start: Node, goal: Node) -> u32 {
    // Initialize a tree with the root node being the start node S.
    let start_node = SearchNode {
        direction: None,
        actual_cost: 0,
        same_dir_counter: 0,
        heuristic_cost: start.pos.distance(&goal.pos),
        node: graph.iter().position(|p| p == &start).unwrap(),
    };
    let goal_id = graph.iter().position(|p| p == &goal).unwrap();

    let mut closed: HashSet<SearchNode> = HashSet::new();
    let mut open = BinaryHeap::new();
    let mut open_hash = HashSet::new();
    open.push(start_node);

    let cost;
    loop {
        // Remove the top node from the open list for exploration.
        let current = open.pop().unwrap();

        // Add all nodes that have an incoming edge from the current node as child nodes in the tree.
        let current_node = graph.get(current.node).unwrap();
        for child_id in current_node.children.iter() {
            let child = graph.get(*child_id).unwrap();

            let dir = child.pos.relative_direction(&current_node.pos);

            if (&current.direction).is_some() {
                let current_direction = current.direction.unwrap();
                let allow = match dir.unwrap() {
                    Direction::Up => current_direction != Direction::Down,
                    Direction::Down => current_direction != Direction::Up,
                    Direction::Left => current_direction != Direction::Right,
                    Direction::Right => current_direction != Direction::Left,
                };
                if !allow {
                    // Can't go to where it came from.
                    continue;
                }
            }

            let dir_counter = match current.direction {
                Some(current_direction) => {
                    if current_direction == dir.unwrap() {
                        current.same_dir_counter + 1
                    } else {
                        0
                    }
                }
                None => 0,
            };

            if dir_counter >= 3 {
                // Can't go in same direction for more than three blocks.
                continue;
            }

            let child_node = SearchNode {
                direction: dir,
                same_dir_counter: dir_counter,
                actual_cost: child.heat_loss as u32 + current.actual_cost,
                heuristic_cost: child.pos.distance(&goal.pos),
                node: *child_id,
            };

            if closed.contains(&child_node) {
                continue;
            }

            if open_hash.contains(&child_node) {
                // Already contains node.
            } else {
                open_hash.insert(child_node.clone());
                open.push(child_node);
            }
        }

        // Add the current node to the closed list.
        if current.node == goal_id {
            cost = current.actual_cost;
            break;
        }

        closed.insert(current);
    }

    cost
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn distance(&self, other: &Position) -> u32 {
        let ax = self.x as f32;
        let ay = self.y as f32;
        let bx = other.x as f32;
        let by = other.y as f32;

        let dx = bx - ax;
        let dy = by - ay;

        (dx * dx + dy * dy).sqrt().ceil() as u32
    }

    fn relative_direction(&self, other: &Position) -> Option<Direction> {
        if self == other {
            return None;
        }

        if self.x < other.x {
            return Some(Direction::Left);
        }

        if self.x > other.x {
            return Some(Direction::Right);
        }

        if self.y > other.y {
            return Some(Direction::Down);
        }

        if self.y < other.y {
            return Some(Direction::Up);
        }

        return None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    pos: Position,
    heat_loss: u8,
    children: Vec<usize>,
}

fn build_graph(input: &str) -> Vec<Node> {
    let data = input
        .lines()
        .map(|x| x.trim())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let height = data.len();
    let width = data.first().unwrap().len();

    let mut nodes = Vec::new();

    for (i, row) in data.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            let heat_loss = value.to_string().parse::<u8>().unwrap();
            let pos = Position { x: j, y: i };

            let index = i * width + j;
            let mut children = Vec::new();

            if j > 0 {
                children.push(index - 1)
            }
            if j < width - 1 {
                children.push(index + 1)
            }
            if i > 0 {
                children.push(index - width)
            }
            if i < height - 1 {
                children.push(index + width)
            }

            nodes.push(Node {
                pos,
                heat_loss,
                children,
            })
        }
    }

    nodes
}
