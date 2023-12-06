const INPUT_FILE: &str = "input_test.txt";

fn parse_line(line: &str) -> Vec<usize> {
    line.split_whitespace()
        .filter_map(|x| x.trim().parse::<usize>().ok())
        .collect()
}

fn does_win(time_left: usize, speed: usize, distance: usize) -> bool {
    return time_left * speed > distance;
}

fn ways_to_win(time: usize, distance: usize) -> usize {
    (0..time).filter(|&speed| does_win(time - speed, speed, distance)).count()
}

fn aoc_day_6_part1(input: &str) -> usize {
    let (time, distance) = input.split_once("\n").unwrap();
    parse_line(time).iter()
        .zip(parse_line(distance).iter())
        .map(|(&t, &d)| ways_to_win(t, d))
        .product()
}

fn main() {
    let file_content = std::fs::read_to_string(INPUT_FILE).expect("Error reading file");
    println!("{}", aoc_day_6_part1(&file_content));
}