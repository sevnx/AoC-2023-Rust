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

fn is_game_possible(game: &Game) -> bool {
    const MAX_RED_CUBES: u32 = 12;
    const MAX_GREEN_CUBES: u32 = 13;
    const MAX_BLUE_CUBES: u32 = 14;
    for set_of_cubes in &game.set_of_cubes {
        if set_of_cubes.red_cubes > MAX_RED_CUBES || set_of_cubes.green_cubes > MAX_GREEN_CUBES || set_of_cubes.blue_cubes > MAX_BLUE_CUBES {
            return false;
        }
    }
    true
}

fn sum_up_id_of_games_with_equal_number_of_cubes(file_name: &str) -> u32 {
    let mut games = Vec::new();
    let mut file = File::open(file_name).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Something went wrong reading the file");
    for line in contents.lines() {
        let game = parse_line_to_game(line);
        if is_game_possible(&game) {
            games.push(game);
        }
    }
    let mut sum = 0;
    for game in games {
        sum += game.id;
    }
    sum
}

fn main() {
    const FILE_NAME: &str = "games.txt";
    println!("Sum : {}", sum_up_id_of_games_with_equal_number_of_cubes(FILE_NAME));
}