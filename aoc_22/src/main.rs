use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
    str::FromStr,
    vec,
};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}
#[derive(Debug)]
struct ParsePointError;
impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s
            .split(",")
            .map(|n| n.parse::<usize>())
            .collect::<Vec<Result<usize, ParseIntError>>>()[..]
        {
            [x, y, z] => {
                let x = x.clone().map_err(|_| ParsePointError)?;
                let y = y.clone().map_err(|_| ParsePointError)?;
                let z = z.clone().map_err(|_| ParsePointError)?;

                Ok(Point { x, y, z })
            }
            _ => Err(ParsePointError),
        }
    }
}
#[derive(Debug)]
struct Brick {
    from: Point,
    to: Point,
}

impl Brick {
    fn get_blocks(&self) -> Vec<Point> {
        let mut blocks = vec![self.from];

        let x = self.from.x;
        let y = self.from.y;
        let z = self.from.z;

        let mut check = false;
        if self.from.x != self.to.x {
            assert_eq!(check, false);
            check = true;
            for x in self.from.x..self.to.x {
                let x = x + 1;
                blocks.push(Point { x, y, z });
            }
        }

        if self.from.y != self.to.y {
            assert_eq!(check, false);
            check = true;
            for y in self.from.y..self.to.y {
                let y = y + 1;
                blocks.push(Point { x, y, z });
            }
        }

        if self.from.z != self.to.z {
            assert_eq!(check, false);
            for z in self.from.z..self.to.z {
                let z = z + 1;
                blocks.push(Point { x, y, z });
            }
        }

        blocks
    }
}

#[derive(Debug)]
struct ParseBrickError;
impl FromStr for Brick {
    type Err = ParseBrickError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pts = s
            .split('~')
            .map(|p| p.parse::<Point>().map_err(|_| ParseBrickError))
            .collect::<Result<Vec<Point>, ParseBrickError>>()?;

        let from = pts.get(0).ok_or(ParseBrickError)?.to_owned();
        let to = pts.get(1).ok_or(ParseBrickError)?.to_owned();

        {
            let mut difference_count = 0;
            if from.x != to.x {
                difference_count += 1;
            }
            if from.y != to.y {
                difference_count += 1;
            }
            if from.z != to.z {
                difference_count += 1;
            }
            if difference_count > 1 {
                panic!("It is assumed that the bricks are straight.")
            }
        }

        Ok(Brick { from, to })
    }
}

#[derive(Debug, Clone)]
struct BrickInstance {
    index: usize,
    blocks: Vec<Point>,
}

impl BrickInstance {
    fn is_vertical(&self) -> bool {
        let first = self.blocks.first().unwrap();
        let different_x_or_y = self.blocks.iter().any(|p| p.x != first.x || p.y != first.y);

        !different_x_or_y
    }

    fn lowest(&self) -> &Point {
        self.blocks.iter().min_by(|a, b| a.z.cmp(&b.z)).unwrap()
    }

    fn highest(&self) -> &Point {
        self.blocks.iter().max_by(|a, b| a.z.cmp(&b.z)).unwrap()
    }

    fn fall(&mut self, map: &mut Vec<Vec<Vec<usize>>>) {
        for p in self.blocks.iter() {
            map[p.x][p.y][p.z] = 0;
        }
        for p in self.blocks.iter_mut() {
            p.z -= 1;
            map[p.x][p.y][p.z] = self.index;
        }
    }

    fn on_ground(&self) -> bool {
        self.blocks.iter().any(|p| p.z == 1)
    }

    fn get_above(&self, map: &Vec<Vec<Vec<usize>>>) -> Vec<usize> {
        if self.is_vertical() {
            let highest = self.highest();
            let mut above_indices = Vec::new();
            let above = map[highest.x][highest.y].get(highest.z + 1);
            match above {
                Some(&above) => {
                    if above != 0 {
                        above_indices.push(above);
                    }
                }
                None => {}
            }

            return above_indices;
        }

        let mut above_indices: Vec<usize> = self
            .blocks
            .iter()
            .map(|p| map[p.x][p.y][p.z + 1])
            .filter(|&n| n != 0)
            .collect();
        above_indices.dedup();
        above_indices
    }

    fn get_below(&self, map: &Vec<Vec<Vec<usize>>>) -> Vec<usize> {
        if self.is_vertical() {
            let lowest = self.lowest();
            let mut below_indices = Vec::new();
            let below = map[lowest.x][lowest.y][lowest.z - 1];
            if below != 0 {
                below_indices.push(below);
            }

            return below_indices;
        }

        let mut below_indices: Vec<usize> = self
            .blocks
            .iter()
            .map(|p| map[p.x][p.y][p.z - 1])
            .filter(|&n| n != 0)
            .collect();
        below_indices.dedup();
        below_indices
    }
}

fn main() {
    let input = include_str!("input");

    let one = part_one(input);
    println!("\nCan safely remove: {} bricks.", one);

    let two = part_two(input);
    println!("\nPart two: {}", two);
}

#[test]
fn test() {
    let example = include_str!("example");
    let one = part_one(example);
    assert_eq!(one, 5);

    let two = part_two(example);
    assert_eq!(two, 7);

    let input = include_str!("input");
    let one = part_one(input);
    assert_eq!(one, 501);

    let two = part_two(input);
    assert_eq!(two, 80948);
}

fn part_one(input: &str) -> u32 {
    let (mut instances, mut map) = read(input);

    let fallen_count = do_fall(&mut instances, &mut map, None);
    println!("{} blocks have fallen.", fallen_count);

    let can_safely_remove_count = calculate_can_safely_remove_count(&instances, &map);

    can_safely_remove_count
}

fn part_two(input: &str) -> usize {
    let (mut instances, mut map) = read(input);
    do_fall(&mut instances, &mut map, None);

    let mut sum = 0;

    for brick in instances.values() {
        let mut instances = instances.clone();
        let mut map = map.clone();

        let fallen_count = do_fall(&mut instances, &mut map, Some(brick.index));
        sum += fallen_count;
    }

    sum
}

fn read(input: &str) -> (HashMap<usize, BrickInstance>, Vec<Vec<Vec<usize>>>) {
    let bricks = input
        .lines()
        .map(|line| line.parse::<Brick>())
        .collect::<Result<Vec<Brick>, ParseBrickError>>()
        .unwrap();

    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    for brick in bricks.iter() {
        max_x = std::cmp::max(max_x, brick.from.x);
        max_x = std::cmp::max(max_x, brick.to.x);
        max_y = std::cmp::max(max_y, brick.from.y);
        max_y = std::cmp::max(max_y, brick.to.y);
        max_z = std::cmp::max(max_z, brick.from.z);
        max_z = std::cmp::max(max_z, brick.to.z);
    }

    let max_x: usize = max_x.try_into().unwrap();
    let max_y: usize = max_y.try_into().unwrap();
    let max_z: usize = max_z.try_into().unwrap();

    let mut map = vec![vec![vec![0; max_z + 1]; max_y + 1]; max_x + 1];
    println!("Created map with x: {}, y: {}, z: {}", max_x, max_y, max_z);

    let mut instances = HashMap::new();

    for (i, brick) in bricks.iter().enumerate() {
        let blocks = brick.get_blocks();
        println!("Brick: {:?} has blocks: \n{:?}", brick, blocks);

        let index = i + 1;
        assert!(index != 0);

        for p in blocks.iter() {
            map[p.x][p.y][p.z] = index;
        }

        instances.insert(index, BrickInstance { index, blocks });
    }

    (instances, map)
}

fn do_fall(instances: &mut HashMap<usize, BrickInstance>, map: &mut Vec<Vec<Vec<usize>>>, ignore_index: Option<usize>) -> usize {
    let mut fallen_bricks = HashSet::new();
    loop {
        let mut have_fallen = false;
        println!("\nInfo: Starting fall loop");
        for instance in instances.values_mut() {
            // make it fall down if possible
            if instance.on_ground() {
                println!("Brick {} is on the ground.", instance.index);
                continue;
            }

            if instance.is_vertical() {
                let lowest = instance.lowest();
                let index_below = map[lowest.x][lowest.y][lowest.z - 1];

                if is_empty(index_below, ignore_index) {
                    instance.fall(map);
                    fallen_bricks.insert(instance.index);
                    have_fallen = true;
                } else {
                    println!(
                        "Below vertical block: {} is block: {}",
                        instance.index, index_below
                    );
                }
            } else {
                let cant_fall = instance.blocks.iter().any(|p| !is_empty(map[p.x][p.y][p.z - 1], ignore_index));

                if cant_fall {
                    println!("Brick: {} can't fall.", instance.index);
                } else {
                    instance.fall(map);
                    fallen_bricks.insert(instance.index);
                    have_fallen = true;
                }
            }
        }

        if !have_fallen {
            println!("Info: No more bricks have fallen, ending fall loop.");

            for brick in instances.values() {
                println!("Brick: {} is at: {:?}", brick.index, brick.blocks);
            }
            break;
        }
    }

    fallen_bricks.len()
}

fn is_empty(value: usize, ignore_index: Option<usize>) -> bool {
    match ignore_index {
        Some(ignore_value) => value == 0 || value == ignore_value,
        None => value == 0,
    }
}

fn calculate_can_safely_remove_count(
    instances: &HashMap<usize, BrickInstance>,
    map: &Vec<Vec<Vec<usize>>>,
) -> u32 {
    let mut can_safely_remove_count = 0;

    for brick in instances.values() {
        // which bricks are we supporting?
        let supporting = brick.get_above(map);
        println!("Brick {} is supporting {:?}", brick.index, supporting);

        let mut can_remove = true;
        for brick_index in supporting {
            let supported_by = instances.get(&brick_index).unwrap().get_below(map);
            println!("Brick {} is supported by {:?}", brick_index, supported_by);

            assert!(supported_by.contains(&brick.index));
            if supported_by.len() == 1 {
                can_remove = false;
                break;
            }
        }

        if can_remove {
            can_safely_remove_count += 1;
        }
    }

    can_safely_remove_count
}
