use std::{num::ParseIntError, ops::Div};

fn main() {
    let input = include_str!("input");

    let range: (i64, i64) = (200000000000000, 400000000000000);
    let count = part_one(input, &range);

    println!("{} pairs intersect within: {:?}", count, range);
}

#[test]
fn test() {
    let example = include_str!("example");
    let range: (i64, i64) = (7, 27);

    let count = part_one(example, &range);
    assert_eq!(count, 2);

    let input = include_str!("input");
    let range: (i64, i64) = (200000000000000, 400000000000000);
    
    let count = part_one(input, &range);
    assert_eq!(count, 12343);    
}

fn part_one(input: &str, range: &(i64, i64)) -> usize {
    let data = input
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<Line<i64>>, ParseLineError>>()
        .unwrap();

    let pairs = build_pairs(&data);

    let result = pairs
        .iter()
        .map(|pair| test_intersection(pair, range))
        .collect::<Vec<_>>();

    let count = result.iter().filter(|&&x| x == true).count();
    count
}

fn test_intersection(pair: &(&Line<i64>, &Line<i64>), range: &(i64, i64)) -> bool {
    let intersection_point = pair.0.intersection_2d(pair.1);

    // is in range?
    if intersection_point.x < range.0 as f64
        || intersection_point.y < range.0 as f64
        || intersection_point.x > range.1 as f64
        || intersection_point.y > range.1 as f64
    {
        return false;
    }

    // is forwards for any of the two lines?
    let sign_a = pair.0.dir.x.signum() as f64;
    let diff_a = intersection_point.x - pair.0.point.x as f64;
    if sign_a * diff_a < 0f64 {
        return false;
    }

    let sign_b = pair.1.dir.x.signum() as f64;
    let diff_b = intersection_point.x - pair.1.point.x as f64;
    if sign_b * diff_b < 0f64 {
        return false;
    }

    return true;
}

fn build_pairs<T>(data: &Vec<T>) -> Vec<(&T, &T)> {
    let mut pairs = Vec::new();

    for i in 0..data.len() - 1 {
        let a = data.get(i);
        for j in i + 1..data.len() {
            let b = data.get(j);
            pairs.push((a.unwrap(), b.unwrap()));
        }
    }

    pairs
}

#[test]
fn test_pairs() {
    let data = vec![1, 2, 3];

    let pairs = build_pairs(&data);
    assert_eq!(pairs, vec![(&1, &2), (&1, &3), (&2, &3)]);
}

#[derive(Debug)]
struct ParseLineError;
fn parse_line(line: &str) -> Result<Line<i64>, ParseLineError> {
    let data = line
        .split('@')
        .map(|part| part.trim())
        .flat_map(|part| {
            part.split(',')
                .map(|x| x.trim())
                .map(|n| n.parse::<i64>())
                .collect::<Vec<Result<i64, ParseIntError>>>()
        })
        .collect::<Result<Vec<_>, ParseIntError>>()
        .map_err(|_| ParseLineError)?;

    match data[..] {
        [px, py, pz, vx, vy, vz] => Ok(Line {
            point: Point3 {
                x: px,
                y: py,
                z: pz,
            },
            dir: Vec3 {
                x: vx,
                y: vy,
                z: vz,
            },
        }),
        _ => Err(ParseLineError),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line<T: Div> {
    point: Point3<T>,
    dir: Vec3<T>,
}

impl Line<i64> {
    fn intersection_2d(&self, pair: &Line<i64>) -> Point2<f64> {
        let l1 = self; // y = ax + c
        let l2 = pair; // y = bx + d

        let a = l1.dir.y as f64 / l1.dir.x as f64;
        let c = l1.point.y as f64 - a * l1.point.x as f64;

        let b = l2.dir.y as f64 / l2.dir.x as f64;
        let d = l2.point.y as f64 - b * l2.point.x as f64;

        let x = (d - c) / (a - b);
        let y = a * (d - c) / (a - b) + c;

        Point2 { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point3<T> {
    x: T,
    y: T,
    z: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

#[derive(Debug)]
struct Point2<T> {
    x: T,
    y: T,
}
