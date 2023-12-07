fn main() {
    println!("Hello, world!");

    // Time:        45     97     72     95
    // Distance:   305   1062   1110   1695

    let a = ways_to_beat_record(45, 305);
    let b = ways_to_beat_record(97, 1062);
    let c = ways_to_beat_record(72, 1110);
    let d = ways_to_beat_record(95, 1695);

    let ans = a * b * c * d;
    println!("{}", ans);

    let time: i64 = 45977295;
    let distance: i64 = 305106211101695;

    let part_two = ways_to_beat_record(time, distance);
    println!("Part two: {}", part_two);
}

fn ways_to_beat_record(available_time: i64, record_distance: i64) -> i64 {
    let mut counter = 0;

    for hold_time in 0..available_time {
        let distance = calculate_distance(hold_time, available_time);

        if distance > record_distance {            
            counter += 1;
        }
    }

    counter
}

fn calculate_distance(hold_time: i64, available_time: i64) -> i64 {
    hold_time * available_time - hold_time * hold_time
}

#[test]
fn test_example() {
    let a = ways_to_beat_record(7, 9);
    let b = ways_to_beat_record(15, 40);
    let c = ways_to_beat_record(30, 200);

    assert_eq!(a, 4);
    assert_eq!(b, 8);
    assert_eq!(c, 9);

    let ans = a * b * c;
    assert_eq!(ans, 288);

    let part_two = ways_to_beat_record(71530, 940200);
    assert_eq!(part_two, 71503);
}
