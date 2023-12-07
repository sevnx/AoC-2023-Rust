use std::cmp::Ordering;
use std::collections::HashMap;

const INPUT_FILE: &str = "input.txt";

fn match_card_to_value(card: &char) -> usize {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid card value")
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 0,
    FourOfAKind = 1,
    FullHouse = 2,
    ThreeOfAKind = 3,
    TwoPairs = 4,
    OnePair = 5,
    HighCard = 6
}

struct CamelCard {
    hand: [usize; 5],
    bid: usize,
    hand_type: HandType
}

fn get_hand_value_hashmap(hand: &[usize; 5]) -> HashMap<usize, usize> {
    let mut count: HashMap<usize, usize> = HashMap::new();
    for i in 0..5 {
        if count.contains_key(&hand[i]) {
            count.insert(hand[i], count.get(&hand[i]).unwrap() + 1);
        } else {
            count.insert(hand[i], 1);
        }
    }
    return count;
}

fn get_max_count(hand: &[usize; 5]) -> usize {
    let count = get_hand_value_hashmap(hand);
    let mut max_count = 0;
    for (_, value) in count {
        if value > max_count {
            max_count = value;
        }
    }
    return max_count;
}

fn is_full_house(hand: &[usize; 5]) -> bool {
    get_hand_value_hashmap(hand).iter().count() == 2 && get_max_count(hand) == 3
}

fn get_number_of_pairs(hand: &[usize; 5]) -> usize {
    let count: HashMap<usize, usize> = get_hand_value_hashmap(hand);
    let mut number_of_pairs = 0;
    for (_, value) in count {
        if value == 2 {
            number_of_pairs += 1;
        }
    }
    return number_of_pairs;
}

fn get_type_of_card(hand: &[usize; 5]) -> HandType {
    return if get_max_count(hand) == 5 {
        HandType::FiveOfAKind
    } else if get_max_count(hand) == 4 {
        HandType::FourOfAKind
    } else if is_full_house(hand) {
        HandType::FullHouse
    } else if get_max_count(hand) == 3 {
        HandType::ThreeOfAKind
    } else if get_max_count(hand) == 2 {
        if get_number_of_pairs(hand) == 2 {
            HandType::TwoPairs
        } else {
            HandType::OnePair
        }
    } else {
        HandType::HighCard
    }
}

impl CamelCard {
    fn new(hand: [usize; 5], bid: usize) -> CamelCard {
        CamelCard {
            hand,
            bid,
            hand_type: get_type_of_card(&hand)
        }
    }
}

fn get_array_of_card_values(hand: &str) -> [usize; 5] {
    let mut card_values: [usize; 5] = [0; 5];
    let mut i = 0;
    for card in hand.chars() {
        card_values[i] = match_card_to_value(&card);
        i += 1;
    }
    card_values
}

fn to_camel_card(line: &str) -> CamelCard {
    let split = line.split_once(" ");
    return CamelCard::new(get_array_of_card_values(split.unwrap().0), split.unwrap().1.parse::<usize>().unwrap());
}

fn compare_cards(card1: &CamelCard, card2: &CamelCard) -> Ordering {
    if card1.hand_type == card2.hand_type {
        for i in 0..5 {
            if card1.hand[i] != card2.hand[i] {
                return card2.hand[i].cmp(&card1.hand[i]);
            }
        }
    }
    return card1.hand_type.cmp(&card2.hand_type);
}

fn aoc_day7_part1(file_content: &str) -> usize {
    let mut cards : Vec<CamelCard> = file_content.lines().map(to_camel_card).collect();
    cards.sort_by(|a, b| compare_cards(a, b));
    cards = cards.into_iter().rev().collect();
    let mut value = 0;
    for (i, card) in cards.iter().enumerate() {
        value += card.bid * (i + 1);
    }
    return value;
}

fn main() {
    let file_contents = std::fs::read_to_string(INPUT_FILE).expect("Error reading input file");
    println!("{}", aoc_day7_part1(&file_contents));
}
