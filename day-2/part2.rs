use std::fs::File;
use std::io::Read;

struct SetOfCubes {
    red_cubes: u32,
    green_cubes: u32,
    blue_cubes: u32,
}

struct Game {
    id: u32,
    set_of_cubes: Vec<SetOfCubes>,
}

fn get_id_from_game_string(string: &str) -> u32 {
    const START_OF_ID: usize = 5;
    string[START_OF_ID..].trim().parse::<u32>().unwrap()
}

fn get_cubes_for_game(string: &str) -> SetOfCubes {
    let mut cube_set = SetOfCubes { red_cubes: 0, green_cubes: 0, blue_cubes: 0 };
    for cubes in string.split(",") {
        let mut cubes_split = cubes.trim().split_whitespace();
        let tmp_value = cubes_split.next().unwrap().trim().parse::<u32>().unwrap();
        let tmp_color = cubes_split.next().unwrap().trim();
        match tmp_color {
            "red" => cube_set.red_cubes += tmp_value,
            "green" => cube_set.green_cubes += tmp_value,
            "blue" => cube_set.blue_cubes += tmp_value,
            _ => println!("Error"),
        }
    }
    cube_set
}

fn parse_line_to_game(string: &str) -> Game {
    let mut split = string.split(":");
    let id = get_id_from_game_string(split.next().unwrap());
    let set_of_cubes = split.next().unwrap().split(';').map(|game| get_cubes_for_game(game)).collect();
    Game { id, set_of_cubes }
}

fn get_power_of_game(game: &Game) -> u32 {
    let mut min_red_cubes = 0;
    let mut min_green_cubes = 0;
    let mut min_blue_cubes = 0;
    for set_of_cubes in &game.set_of_cubes {
        min_red_cubes = min_red_cubes.max(set_of_cubes.red_cubes);
        min_green_cubes = min_green_cubes.max(set_of_cubes.green_cubes);
        min_blue_cubes = min_blue_cubes.max(set_of_cubes.blue_cubes);
    }
    min_red_cubes * min_green_cubes * min_blue_cubes
}

fn sum_up_power_of_games(file_name: &str) -> u32 {
    let mut file = File::open(file_name).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    contents.lines().map(|line| parse_line_to_game(line)).map(|game| get_power_of_game(&game)).sum()
}

fn main() {
    const FILE_NAME: &str = "games.txt";
    println!("Sum : {}", sum_up_power_of_games(FILE_NAME));
}