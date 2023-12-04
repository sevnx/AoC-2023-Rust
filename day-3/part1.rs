use std::fs;

struct FileParseResult {
    grid_numbers: Vec<u32>,
    grid_pos_grid_numbers: Vec<Vec<i32>>,
    pos_of_gear: Vec<(i32, i32)>,
}

const DIGIT_ONLY: u32 = 10;
const NOT_DIGIT : i32 = -1;

fn is_gear(candidate : char) -> bool {
    return candidate == '*';
}

fn is_pos_in_grid(grid: &Vec<Vec<i32>>, x: i32, y: i32) -> bool {
    return x < grid[0].len() as i32 && y < grid.len() as i32 && x >= 0 && y >= 0;
}

fn add_line_to_grid(grid: &mut Vec<Vec<char>>, grid_numbers: &mut Vec<u32>, grid_pos_grid_numbers: &mut Vec<Vec<i32>>, pos_of_gear: &mut Vec<(i32, i32)>, line: &str) {
    let mut line_vec: Vec<char> = Vec::new();
    let mut line_grid_pos_grid_numbers: Vec<i32> = Vec::new();
    let mut in_grid_number = false;
    let mut tmp_pos_in_grid_numbers: u32 = 0;

    for c in line.chars() {
        if c.is_digit(DIGIT_ONLY) {
            if !in_grid_number {
                tmp_pos_in_grid_numbers = grid_numbers.len() as u32;
                in_grid_number = true;
                grid_numbers.push(c.to_digit(DIGIT_ONLY).unwrap());
            } else {
                grid_numbers[tmp_pos_in_grid_numbers as usize] = grid_numbers[tmp_pos_in_grid_numbers as usize] * 10 + c.to_digit(DIGIT_ONLY).unwrap();
            }
            line_grid_pos_grid_numbers.push(tmp_pos_in_grid_numbers as i32);
        } else {
            in_grid_number = false;
            line_grid_pos_grid_numbers.push(NOT_DIGIT);
            if is_gear(c) {
                pos_of_gear.push((line_vec.len() as i32, grid.len() as i32));
            }
        }
        line_vec.push(c);
    }

    grid.push(line_vec);
    grid_pos_grid_numbers.push(line_grid_pos_grid_numbers);
}

fn get_adjacent_numbers(grid_pos_grid_numbers: &Vec<Vec<i32>>, pos_of_gear: &(i32, i32), grid_numbers : &Vec<u32>) -> Vec<u32> {
    let mut adjacent_numbers: Vec<u32> = Vec::new();
    for y in -1..=1 {
        for x in -1..=1 {
            if x == 0 && y == 0 { continue; }
            let pos_x = pos_of_gear.0 + x;
            let pos_y = pos_of_gear.1 + y;
            if is_pos_in_grid(grid_pos_grid_numbers, pos_x, pos_y) {
                let grid_number = grid_pos_grid_numbers[pos_y as usize][pos_x as usize];
                if grid_number != NOT_DIGIT {
                    let number_to_add = grid_numbers[grid_number as usize];
                    if !adjacent_numbers.contains(&number_to_add) {
                        adjacent_numbers.push(number_to_add);
                    }
                }
            }
        }
    }
    return adjacent_numbers;
}

fn get_gear_ratio(grid_pos_grid_numbers: &Vec<Vec<i32>>, pos_of_gear: &(i32, i32), grid_numbers : &Vec<u32>) -> Option<u32> {
    const CORRECT_NUMBER_OF_ADJACENT_NUMBERS: u32 = 2;
    let adjacent_numbers = get_adjacent_numbers(grid_pos_grid_numbers, pos_of_gear, &grid_numbers);
    if adjacent_numbers.len() == CORRECT_NUMBER_OF_ADJACENT_NUMBERS as usize {
        return Some(adjacent_numbers[0] * adjacent_numbers[1]);
    }
    return None;
}

fn file_content_to_grid(file_content: String) -> FileParseResult {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut grid_numbers: Vec<u32> = Vec::new();
    let mut grid_pos_grid_numbers: Vec<Vec<i32>> = Vec::new();
    let mut pos_of_gear: Vec<(i32, i32)> = Vec::new();

    file_content
        .lines()
        .for_each(|line| add_line_to_grid(&mut grid, &mut grid_numbers, &mut grid_pos_grid_numbers, &mut pos_of_gear, line));

    return FileParseResult {
        grid_numbers,
        grid_pos_grid_numbers,
        pos_of_gear
    };
}

fn advent_of_code_2023_day_3_part_2() -> u32 {
    let file_content = fs::read_to_string("day3.txt").expect("Error reading file");
    let file_parse_result = file_content_to_grid(file_content);
    file_parse_result
        .pos_of_gear
        .iter()
        .filter_map(|pos_of_gear| get_gear_ratio(&file_parse_result.grid_pos_grid_numbers, pos_of_gear, &file_parse_result.grid_numbers))
        .sum()
}

fn main() {
    println!("{}", advent_of_code_2023_day_3_part_2());
}

/*
fn advent_of_code_2023_day_3_part_1() -> u32 {
    let file_content = std::fs::read_to_string("day3.txt").expect("Error reading file");
    let (grid, grid_numbers) = file_content_to_grid(file_content);
    return grid_numbers
        .iter()
        .filter(|visual_representation_number| is_number_adjacent_to_symbol(visual_representation_number, &grid))
        .map(|visual_representation_number| visual_representation_number.number)
        .sum();

fn is_symbol(candidate : char) -> bool {
    return !(candidate == '.') && !candidate.is_digit(DIGIT_ONLY);
}

fn is_number_adjacent_to_symbol(visual_representation_number: &GridNumber, grid: &Vec<Vec<char>>) -> bool {
    let mut x: i32 = (visual_representation_number.start_x - 1) as i32;
    let mut y: i32 = visual_representation_number.start_y as i32;
    let length = visual_representation_number.length;
    let mut i: i32 = -1;
    while (i < length as i32 + 2)  {
        if is_pos_in_grid(grid, x + i, y) && is_symbol(grid[y as usize][(x + i) as usize]) {
            return true;
        }
        if is_pos_in_grid(grid, x + i, y + 1) && is_symbol(grid[(y + 1) as usize][(x + i) as usize]) {
            return true;
        }
        if is_pos_in_grid(grid, x + i, y - 1) && is_symbol(grid[(y - 1) as usize][(x + i) as usize]) {
            return true;
        }
        i += 1;
    }
    return false;
} */

