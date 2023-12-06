const INPUT_FILE: &str = "input.txt";

fn get_line_value(line: &str) -> u32 {
    let digits: Vec<u32> = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    return digits.get(0).unwrap()*10 + digits.get(digits.len()-1).unwrap();
}

fn number_to_text(number: u32) -> &'static str {
    match number {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        _ => "nine",
    }
}

fn change_digit_in_text_to_number(text: &mut String) {
    loop {
        let (mut lowest_index, mut lowest_val) = (None, None);
        for i in 0..=9 {
            let number_to_text = number_to_text(i);
            let index = text.find(number_to_text);
            if index.is_some() {
                if lowest_val.is_none() || index.unwrap() < lowest_index.unwrap() {
                    lowest_index = index;
                    lowest_val = Some(i as usize);
                }
            }
        }
        if lowest_index.is_none() {
            break;
        } else {
            text.replace_range(lowest_index.unwrap()..lowest_index.unwrap()+1, &lowest_val.unwrap().to_string());
        }
    }
}

fn aoc_day1_part1(input: &str) -> u32 {
    input.lines().map(get_line_value).sum()
}

fn aoc_day1_part2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut line = line.to_string();
        change_digit_in_text_to_number(&mut line);
        sum += get_line_value(&line);
    }
    return sum;
}

fn main() {
    let file_content = std::fs::read_to_string(INPUT_FILE).expect("Error reading input file");
    println!("{}", aoc_day1_part1(&file_content));
    println!("{}", aoc_day1_part2(&file_content));
}