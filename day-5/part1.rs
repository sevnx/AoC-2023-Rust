const INPUT_FILE : &str = "input_test.txt";
struct ConversionValue {
    start_value: usize,
    start_index: usize,
    length: usize,
}

impl ConversionValue {
    fn map_to(&self, index: usize) -> Option<usize> {
        let diff = index.checked_sub(self.start_index)?;
        if diff < self.length {
            Some(self.start_value + diff)
        } else {
            None
        }
    }
}

struct ConversionMap {
    conversion_values: Vec<ConversionValue>
}

impl ConversionMap {
    fn new() -> Self {
        Self {
            conversion_values: Vec::new(),
        }
    }

    fn add_conversion_value(&mut self, conversion_value: ConversionValue) {
        self.conversion_values.push(conversion_value);
    }

    fn get_location_of_seed(&self, seed: usize) -> usize {
        for conversion_value in &self.conversion_values {
            if let Some(val) = conversion_value.map_to(seed) {
                return val;
            }
        }
        seed
    }
}

fn get_location_of_seed(seed: usize, conversion_map: &[ConversionMap]) -> usize {
    conversion_map.iter().fold(seed, |acc, x| x.get_location_of_seed(acc))
}

fn get_conversion_map_from_info(info: &str) -> ConversionMap {
    let mut conversion_map = ConversionMap::new();
    for line in info.split("\n") {
        if line.contains(":") {
            continue;
        }
        let mut number_split = line.trim().split(" ");
        conversion_map.add_conversion_value(ConversionValue {
            start_value: number_split.next().unwrap().parse::<usize>().unwrap(),
            start_index: number_split.next().unwrap().parse::<usize>().unwrap(),
            length: number_split.next().unwrap().parse::<usize>().unwrap(),
        });
    }
    return conversion_map;
}

fn aoc_day5_part1(file_content: &str) -> usize {
    let split = file_content.split_once("\n\r\n");
    let seeds= split.unwrap().0[split.unwrap().0.find(":").unwrap()+1..]
        .split_once("\r").unwrap().0
        .split_whitespace()
        .filter_map(|x| x.trim().parse::<usize>().ok())
        .collect::<Vec<usize>>();

    let mut conversion_maps : Vec<ConversionMap> = Vec::new();
    for conversion_map in split.unwrap().1.split("\n\r\n") {
        let conversion_map = get_conversion_map_from_info(conversion_map);
        conversion_maps.push(conversion_map);
    }
    let mut min:Option<usize> = None;
    for seed in seeds {
        let location = get_location_of_seed(seed, &conversion_maps);
        if min == None {
            min = Some(location);
        } else if min.unwrap() > location {
            min = Some(location);
        }
    }
    return min.unwrap();
}
fn main() {
    let file_content = std::fs::read_to_string(INPUT_FILE).expect("Error reading file");
    println!("{}", aoc_day5_part1(&file_content));
}

