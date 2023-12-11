fn main() {
    let data = read(include_str!("input"));
    let steps = part_one(&data);
    println!("Steps: {}", steps);

    let steps_ghost = part_two(&data);
    println!("Steps ghosts: {}", steps_ghost);
}

fn part_one(data: &Data) -> usize {
    let start = "AAA";
    let end = "ZZZ";

    let mut current = data.nodes.iter().find(|x| x.name == start).unwrap();
    let mut steps = 0;
    while current.name != end {
        let index = steps % data.instructions.len();
        let instruction = data.instructions.get(index).unwrap();

        steps += 1;
        match instruction {
            Instruction::Left => {
                current = data.nodes.iter().find(|x| x.name == current.left).unwrap()
            }
            Instruction::Right => {
                current = data.nodes.iter().find(|x| x.name == current.right).unwrap()
            }
        }
    }

    steps
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Node {
    pub left: usize,
    pub right: usize,
    pub is_end: bool,
    pub is_start: bool,
}

fn create_lean_node_list(data: &Data) -> Vec<Node> {
    let mut nodes = Vec::new();
    for node in &data.nodes {
        let name = node.name.clone();
        let left = data.nodes.iter().position(|x| x.name == node.left).unwrap();
        let right = data
            .nodes
            .iter()
            .position(|x| x.name == node.right)
            .unwrap();

        let is_end = name.ends_with('Z');
        let is_start = name.ends_with('A');

        if is_end {
            assert_eq!(is_start, false);
        }

        if is_start {
            assert_eq!(is_end, false);
        }

        nodes.push(Node {
            left,
            right,
            is_end,
            is_start,
        });
    }

    nodes
}

fn part_two(data: &Data) -> u64 {
    let nodes = create_lean_node_list(data);

    let starts = nodes
        .iter()
        .filter(|x| x.is_start)
        .map(|x| (*x).clone())
        .collect::<Vec<_>>();

    let mut periods = Vec::new();
    for node in starts {
        let mut period = 0;

        let mut current = node;
        loop {
            let i = period % data.instructions.len();
            let instr = data.instructions.get(i).unwrap();
            period += 1;

            let next = match instr {
                Instruction::Left => nodes.get(current.left).unwrap(),
                Instruction::Right => nodes.get(current.right).unwrap(),
            };

            current = *next;

            if current.is_end {
                break;
            }
        }

        periods.push(period as u64);
    }

    let mut steps: u64 = 1;
    for value in periods {
        steps = lcm(steps.try_into().unwrap(), value);
    }

    steps
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

#[test]
fn test_lcm() {
    let n = lcm(1, 2);
    assert_eq!(n, 2);

    let n = lcm(3, 7);
    assert_eq!(n, 21);

    let n = lcm(12, 15);
    assert_eq!(n, 60);
}

fn gcd(num_a: u64, num_b: u64) -> u64 {
    let mut a = num_a;
    let mut b = num_b;

    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }

    return a;
}

#[test]
fn test_gcd() {
    let a = gcd(21, 14);
    assert_eq!(a, 7);

    let a = gcd(81, 45);
    assert_eq!(a, 9);

    let a = gcd(121, 144);
    assert_eq!(a, 1);

    let a = gcd(56, 48);
    assert_eq!(a, 8);

    let a = gcd(10, 5);
    assert_eq!(a, 5);

    let a = gcd(10, 10);
    assert_eq!(a, 10);

    let a = gcd(0, 0);
    assert_eq!(a, 0);
}

#[derive(Debug)]
struct NodeData {
    pub name: String,
    pub left: String,
    pub right: String,
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct Data {
    pub instructions: Vec<Instruction>,
    pub nodes: Vec<NodeData>,
}

fn read(input: &str) -> Data {
    let mut instructions = Vec::new();

    for char in input.lines().next().unwrap().chars() {
        if char == 'R' {
            instructions.push(Instruction::Right);
        }
        if char == 'L' {
            instructions.push(Instruction::Left);
        }
    }

    let mut nodes = Vec::new();
    for line in input.lines() {
        if line.contains("=") {
            let assg = line
                .split("=")
                .into_iter()
                .map(|x| x.trim())
                .collect::<Vec<_>>();

            let name = assg.get(0).unwrap().to_string();
            let children = assg.get(1).unwrap();

            let opening = '(';
            let closing = ')';
            let mut start = 0;
            let mut end = 0;
            for (i, char) in children.chars().enumerate() {
                if char == opening {
                    start = i;
                }
                if char == closing {
                    end = i;
                }
            }

            let child_nodes = children
                .get(start + 1..end)
                .unwrap()
                .split(",")
                .into_iter()
                .map(|a| a.trim())
                .collect::<Vec<_>>();

            let left = child_nodes.get(0).unwrap().to_string();
            let right = child_nodes.get(1).unwrap().to_string();

            let node = NodeData { name, left, right };
            nodes.push(node);
        }
    }

    Data {
        nodes,
        instructions,
    }
}

#[test]
fn test_example() {
    let data1 = read(include_str!("example_1"));
    let steps1 = part_one(&data1);
    assert_eq!(steps1, 2);

    let data2 = read(include_str!("example_2"));
    let steps2 = part_one(&data2);
    assert_eq!(steps2, 6);

    let data_input = read(include_str!("input"));
    let steps_input = part_one(&data_input);
    assert_eq!(steps_input, 15989);

    let data3 = read(include_str!("example_3"));
    let steps3 = part_two(&data3);
    assert_eq!(steps3, 6);

    let data_part_two = read(include_str!("input"));
    let steps_part_two = part_two(&data_part_two);
    assert_eq!(steps_part_two, 13830919117339);
}
