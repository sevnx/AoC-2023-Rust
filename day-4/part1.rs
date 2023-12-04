const INPUT_FILE: &str = "input.txt";

fn get_points_of_line(line: String) -> usize {
    let mut number_split = line.split("|");
    let winning_numbers: Vec<u32> = number_split.next().unwrap().split_whitespace()
        .filter_map(|x| x.trim().parse::<u32>().ok())
        .collect();
    let number_of_matches = number_split.next().unwrap().split_whitespace()
        .filter(|x| winning_numbers.contains(&x.trim().parse::<u32>().unwrap()))
        .count();
    if number_of_matches == 0 { 0 } else { 2usize.pow(number_of_matches as u32 - 1) }
}


fn aoc_day4_part1(file_content: String) -> usize {
    let file_line_split = file_content.split("\n");
    let mut total_points = 0;
    for line in file_line_split {
        let start_pos = line.find(":").unwrap()+1;
        total_points += get_points_of_line(line[start_pos..].to_string());
    }
    return total_points;
}

fn main(){
    let file_content = std::fs::read_to_string(INPUT_FILE).expect("Error reading file");
    let result = aoc_day4_part1(file_content);
    println!("Result: {}", result);
}