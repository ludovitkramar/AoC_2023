#[derive(PartialEq, Debug)]
struct Cubes {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl std::cmp::PartialOrd for Cubes {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.red <= other.red && self.green <= other.green && self.blue <= other.blue {
            return Some(std::cmp::Ordering::Equal);
        }
        if self.red < other.red && self.green < other.green && self.blue < other.blue {
            return Some(std::cmp::Ordering::Less);
        }
        return Some(std::cmp::Ordering::Greater);
    }
}

impl From<&str> for Cubes {
    fn from(value: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for data in value.split(',').map(|x| x.trim().to_lowercase()) {
            let num = data.chars().filter(|c| c.is_digit(10));
            let n = String::from_iter(num).parse::<u32>().unwrap();
            // println!("{}, {}", data, n);

            if data.contains("red") {
                red = n;
            } else if data.contains("green") {
                green = n;
            } else if data.contains("blue") {
                blue = n;
            }
        }

        Cubes { red, green, blue }
    }
}

#[test]
fn compare() {
    let a = Cubes {
        red: 2,
        green: 3,
        blue: 4,
    };
    let b = Cubes {
        red: 1,
        green: 3,
        blue: 4,
    };
    assert_eq!(b < a, false);
    assert_eq!(b <= a, true);
    assert_eq!(a == b, false);
}

fn main() {
    let input = include_str!("input");

    let limit = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };

    // Sum of possible games' ids.
    let mut id_sum: u32 = 0;
    let mut power_sum: u32 = 0;

    for line in input.lines() {
        let ia = line.find("Game");
        let ib = line.find(':');
        if ia.is_some() && ib.is_some() {
            let id_r = line
                .get(ia.unwrap() + 4..ib.unwrap())
                .unwrap()
                .trim()
                .parse::<u32>();
            match id_r {
                Ok(id) => {
                    println!(" == {} == ", line);

                    let mut max_r = 0;
                    let mut max_g = 0;
                    let mut max_b = 0;

                    let data = line.get(ib.unwrap() + 1..).unwrap();
                    let mut ok = true;
                    for reveal in data.split(';') {
                        let cubes = Cubes::from(reveal);
                        println!("Input: {}, data: {:?}", reveal, cubes);

                        max_r = std::cmp::max(max_r, cubes.red);
                        max_g = std::cmp::max(max_g, cubes.green);
                        max_b = std::cmp::max(max_b, cubes.blue);

                        if cubes > limit {
                            ok = false;
                        }
                    }

                    let power = max_b * max_r * max_g;
                    power_sum += power;

                    if ok {
                        println!("Id: {} is good!", id);
                        id_sum += id;
                    }
                }
                _ => {}
            }
        }
    }
    println!("Result: {}", id_sum);
    println!("Sum of powers: {}", power_sum);
}
