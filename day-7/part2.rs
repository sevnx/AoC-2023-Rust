use std::cmp::Ordering;
use std::collections::HashMap;

const INPUT_FILE: &str = "input.txt";
const JOKER_VALUE: usize = 1;

fn match_card_to_value(card: &char) -> usize {
    match card {
        'J' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => panic!("Invalid card value")
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    FiveOfAKind = 0,
    FourOfAKind = 1,
    FullHouse = 2,
    ThreeOfAKind = 3,
    TwoPairs = 4,
    OnePair = 5,
    HighCard = 6
}

#[derive(Debug)]
struct CamelCard {
    hand: [usize; 5],
    bid: usize,
    hand_type: HandType,
}

fn get_hand_value_hashmap(hand: &[usize; 5]) -> HashMap<usize, usize> {
    let mut count: HashMap<usize, usize> = HashMap::new();
    for &card in hand {
        *count.entry(card).or_insert(0) += 1;
    }
    if count.contains_key(&JOKER_VALUE) && count.len() > 1 {
        let joker_count = count.get(&JOKER_VALUE).unwrap().clone();
        count.remove(&JOKER_VALUE);
        let (max_card, max_count) = count.iter().max_by_key(|(_, &v)| v).unwrap();
        count.insert(*max_card, max_count + joker_count);
    }
    return count;
}


fn get_max_count(count: &HashMap<usize, usize>) -> usize {
    count.values().max().unwrap_or(&0).clone()
}

fn get_number_of_pairs(count: &HashMap<usize, usize>) -> usize {
    count.values().filter(|&&v| v == 2).count()
}

fn get_type_of_card(count: &HashMap<usize, usize>) -> HandType {
    match get_max_count(count) {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 if get_number_of_pairs(count) == 1 => HandType::FullHouse,
        3 => HandType::ThreeOfAKind,
        2 if get_number_of_pairs(count) == 2 => HandType::TwoPairs,
        2 => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

impl CamelCard {
    fn new(hand: [usize; 5], bid: usize) -> CamelCard {
        CamelCard {
            hand,
            bid,
            hand_type: get_type_of_card(&get_hand_value_hashmap(&hand)),
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
    let (hand, bid) = line.split_once(" ").unwrap();
    CamelCard::new(get_array_of_card_values(hand), bid.parse::<usize>().unwrap())
}

fn compare_cards(card1: &CamelCard, card2: &CamelCard) -> Ordering {
    card1.hand_type.cmp(&card2.hand_type).then_with(|| card2.hand.cmp(&card1.hand))
}


fn aoc_day7_part2(file_content: &str) -> usize {
    let mut cards : Vec<CamelCard> = file_content.lines().map(to_camel_card).collect();
    cards.sort_by(compare_cards);
    return cards.iter().rev().enumerate().map(|(i, card)| card.bid * (i + 1)).sum();
}


fn main() {
    let file_contents = std::fs::read_to_string(INPUT_FILE).expect("Error reading input file");
    println!("{}", aoc_day7_part2(&file_contents));
}
