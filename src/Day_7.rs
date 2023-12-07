use itertools::Itertools;
use std::collections::HashMap;

pub fn solution(file_content: String) -> u64 {
    solution_part_one(file_content)
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn recognize_hand_type(hand: Vec<&Card>) -> HandType {
    let mut number_of_cards: HashMap<&Card, u8> = HashMap::new();
    for card in hand {
        match number_of_cards.get(&card) {
            Some(_) => {
                *number_of_cards.get_mut(&card).unwrap() += 1;
            }
            None => {
                number_of_cards.insert(card.clone(), 1);
            }
        }
    }

    let most_common_card = *number_of_cards.values().max().unwrap();

    if most_common_card == 5 {
        return HandType::FiveOfAKind;
    } else if most_common_card == 4 {
        return HandType::FourOfAKind;
    } else if most_common_card == 3 && number_of_cards.len() == 2 {
        return HandType::FullHouse;
    } else if most_common_card == 3 && number_of_cards.len() == 3 {
        return HandType::ThreeOfAKind;
    } else if most_common_card == 2 && number_of_cards.len() == 3 {
        return HandType::TwoPair;
    } else if most_common_card == 2 && number_of_cards.len() == 4 {
        return HandType::OnePair;
    }
    HandType::HighCard
}

fn solution_part_one(file_content: String) -> u64 {
    let char_to_card: HashMap<char, Card> = HashMap::from([
        ('2', Card::Two),
        ('3', Card::Three),
        ('4', Card::Four),
        ('5', Card::Five),
        ('6', Card::Six),
        ('7', Card::Seven),
        ('8', Card::Eight),
        ('9', Card::Nine),
        ('T', Card::T),
        ('J', Card::J),
        ('Q', Card::Q),
        ('K', Card::K),
        ('A', Card::A),
    ]);
    let lines = file_content
        .lines()
        .map(|line| line.split(" ").collect::<Vec<&str>>());
    let hands = lines.clone().map(|e| {
        e[0].chars()
            .map(|c: char| char_to_card.get(&c).unwrap())
            .collect::<Vec<&Card>>()
    });
    let bid = lines.clone().map(|e| e[1].parse::<u64>().ok().unwrap());
    let mut deals = hands.zip(bid).collect::<Vec<(Vec<&Card>, u64)>>();
    deals.sort_by(|deal_a, deal_b| {
        (&recognize_hand_type(deal_a.0.clone())).cmp(&recognize_hand_type(deal_b.0.clone()))
    });
    //deals.group_by(|deal_a, deal_b| recognize_hand_type(deal_a.0) == recognize_hand_type(deal_b.0));
    let mut indices_to_split_on: Vec<usize> = vec![];
    indices_to_split_on.push(0);
    for index in 0..(deals.len() - 1) {
        if recognize_hand_type(deals[index].0.clone())
            != recognize_hand_type(deals[index + 1].0.clone())
        {
            indices_to_split_on.push(index + 1);
        }
    }

    println!("{:?}", indices_to_split_on);

    let mut parted_deals: Vec<Vec<(Vec<&Card>, u64)>> = vec![];
    for i in 0..(indices_to_split_on.len() - 1) {
        parted_deals.push(deals[indices_to_split_on[i]..indices_to_split_on[i + 1]].to_vec());
    }
    parted_deals.push(deals[indices_to_split_on[indices_to_split_on.len() - 1]..1000].to_vec());

    for deal_types in &mut parted_deals {
        (*deal_types).sort_by(|deal_a, deal_b| (deal_a.0).cmp(&deal_b.0));
    }
    /*

        let list_to_sort = Vec::from([6, 5, 3, 2, 1, 4]);
        let sorted_list = {
            let tmp = list_to_sort.clone();
            tmp.sort();
            tmp
        }
    }

    */
    parted_deals
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(index, deal)| (index as u64 + 1) * deal.1)
        .sum()
}
