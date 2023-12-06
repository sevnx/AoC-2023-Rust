const INPUT_FILE: &str = "input.txt";

fn get_line_value(line: &str) -> u32 {
    let digits: Vec<u32> = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    return digits.get(0).unwrap()*10 + digits.get(digits.len()-1).unwrap();
}

fn aoc_day1_part1(input: &str) -> u32 {
    input.lines().map(get_line_value).sum()
}

fn main() {
    let file_content = std::fs::read_to_string(INPUT_FILE).expect("Error reading input file");
    println!("{}", aoc_day1_part1(&file_content));
}
