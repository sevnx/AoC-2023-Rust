const INPUT_FILE: &str = "input.txt";

fn get_number_of_matches(line: String) -> usize {
    let mut number_split = line.split("|");
    let winning_numbers: Vec<u32> = number_split.next().unwrap().split_whitespace()
        .filter_map(|x| x.trim().parse::<u32>().ok())
        .collect();
    return number_split.next().unwrap().split_whitespace()
        .filter(|x| winning_numbers.contains(&x.trim().parse::<u32>().unwrap()))
        .count();
}

fn aoc_day4_part2(file_content: String) -> usize {
    let file_line_split = file_content.split("\n");
    let mut vec_number_of_matches: Vec<(usize,usize)> = Vec::new();
    for line in file_line_split {
        let start_pos = line.find(":").unwrap() + 1;
        vec_number_of_matches.push((1,get_number_of_matches(line[start_pos..].to_string())));
    }
    let mut possessed_count = 0;
    for index in 0..vec_number_of_matches.len() {
        let current = vec_number_of_matches[index];
        possessed_count += current.0;
        for j in 1..=current.1 {
            vec_number_of_matches[index+j].0 += current.0;
        }
    }
    return possessed_count;
}

fn main(){
    let file_content = std::fs::read_to_string(INPUT_FILE).expect("Error reading file");
    let result = aoc_day4_part2(file_content);
    println!("Result: {}", result);
}