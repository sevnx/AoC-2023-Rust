use std::collections::HashMap;

const INPUT_FILE : &str = "input_test.txt";

fn get_location_of_seed(seed: u32) -> usize {
    return 0;
}

fn aoc_day5_part1(file_content: &str) -> usize {
    let split = file_content.split_once("\n\n");
    let seeds: &Vec<u32> = &split.unwrap().0[split.unwrap().0.find(":").unwrap()..].trim().split_whitespace().filter_map(|x| x.trim().parse::<u32>().ok()).collect();
    let conversion_maps : Vec<HashMap<u32,u32>>;
    for conversion_map in split.unwrap().1.split("\n\n"){
        println!("{}",conversion_map);
    }
    return 0;
}

fn main() {
    let file_content = std::fs::read_to_string(INPUT_FILE).expect("Error reading file");
    println!("{}",aoc_day5_part1(&file_content));
}
