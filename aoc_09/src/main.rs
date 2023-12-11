use std::num::ParseIntError;

fn main() {
    let input = include_str!("input");
    let data = read(input);

    let a = part_one(&data);
    println!("Result: {}", a);

    let b = part_two(&data);
    println!("Part two. Result: {}", b);
}

fn part_one(data: &Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;

    for nums in data {
        let prediction = predict(&nums);
        sum += prediction;
        println!("For {:?} predicted: {}", nums, prediction);
    }

    sum
}

fn part_two(data: &Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;

    for nums in data {
        let history = extrapolate_backwards(&nums);
        sum += history;
        println!("For {:?} extrapolated history: {}", nums, history);
    }

    sum
}

#[test]
fn test() {
    let example = include_str!("example");
    let ex_data = read(example);
    let a = part_one(&ex_data);
    assert_eq!(a, 114);

    let b = part_two(&ex_data);
    assert_eq!(b, 2);

    let input = include_str!("input");
    let data = read(input);
    let a = part_one(&data);
    assert_eq!(a, 1969958987);
}

fn calc_diff_till_zeros(numbers: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut steps = Vec::new();
    let mut differences = calc_differences(numbers);

    while differences.iter().any(|num| *num != 0) {
        let new = calc_differences(&differences);
        steps.push(differences);
        differences = new;
    }

    steps
}

fn predict(numbers: &Vec<i64>) -> i64 {
    let steps = calc_diff_till_zeros(&numbers);

    let mut carry = 0;
    for step in steps.iter().rev() {
        let last = step.last().unwrap();

        carry += last;
    }

    carry + numbers.last().unwrap()
}

fn extrapolate_backwards(numbers: &Vec<i64>) -> i64 {
    let steps = calc_diff_till_zeros(&numbers);

    let mut carry = 0;
    for step in steps.iter().rev() {
        let last = step.first().unwrap();

        carry = last - carry;
    }

    numbers.first().unwrap() - carry
}

fn calc_differences(numbers: &[i64]) -> Vec<i64> {
    let mut ret = Vec::new();

    let mut iter = numbers.iter();
    let mut previous = iter.next().unwrap();

    for num in iter {
        ret.push(num - previous);

        previous = num;
    }

    ret
}

fn read(input: &str) -> Vec<Vec<i64>> {
    let mut ret = Vec::new();
    for line in input.lines() {
        let numbers = line
            .split(' ')
            .into_iter()
            .map(|x| x.trim().parse::<i64>())
            .collect::<Result<Vec<_>, ParseIntError>>();

        match numbers {
            Ok(numbers) => ret.push(numbers),
            _ => panic!(),
        }
    }

    ret
}
