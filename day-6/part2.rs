const INPUT_FILE: &str = "input_test.txt";

fn parse_line(line: &str) -> usize {
    return line.split_once(":").unwrap().1
        .replace(" ", "").trim()
        .parse::<usize>().unwrap();
}

fn does_win(time_left: usize, speed: usize, distance: usize) -> bool {
    return time_left * speed > distance;
}

fn ways_to_win(time: usize, distance: usize) -> usize {
    (0..time).filter(|&speed| does_win(time - speed, speed, distance)).count()
}

fn aoc_day_6_part2(input: &str) -> usize {
    let (time, distance) = input.split_once("\n").unwrap();
    ways_to_win(parse_line(time), parse_line(distance))
}

fn main() {
    let file_content = std::fs::read_to_string(INPUT_FILE).expect("Error reading file");
    println!("{}", aoc_day_6_part2(&file_content));
}