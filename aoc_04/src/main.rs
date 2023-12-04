fn parse(input: &str) -> Vec<u32> {
    let mut res = Vec::new();

    for part in input.split(" ").map(|p| p.trim()) {
        let n = part.parse::<u32>();
        if n.is_ok() {
            res.push(n.unwrap());
        }
    }

    return res;
}

fn main() {
    let input = include_str!("input");

    let mut points = 0;

    for line in input.lines() {
        let t0: Vec<&str> = line.split("|").collect();
        let card: Vec<&str> = t0.get(0).unwrap().split(":").collect();
        let my_numbers = t0.get(1).unwrap();

        let winning_numbers = card.get(1).unwrap();

        let win = parse(&winning_numbers);
        let mut my = parse(&my_numbers);

        my.retain(|n| win.contains(n));

        let count = my.len();
        if count > 0 {
            let point = 2_i32.pow((count - 1).try_into().unwrap());
            points += point;
        }
    }

    println!("\nTotal: {}", points);
}
