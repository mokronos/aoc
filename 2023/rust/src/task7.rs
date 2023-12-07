use rust::read_data;
use phf::{phf_map};
use std::collections::HashMap;

#[allow(unused)]
const TEST_DATA: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

const VALUES: phf::Map<char, u32> = phf_map! {
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
};

const JOKER_VALUES: phf::Map<char, u32> = phf_map! {
    'J' => 2,
    '2' => 3,
    '3' => 4,
    '4' => 5,
    '5' => 6,
    '6' => 7,
    '7' => 8,
    '8' => 9,
    '9' => 10,
    'T' => 11,
    'Q' => 12,
    'K' => 13,
    'A' => 14,
};

pub fn run7_1()
{
    let data = read_data("data/data7.txt");
    let data = TEST_DATA;
    let mut total = 0;
    let mut hands = parse_data(&data);

    hands.iter_mut().for_each(|hand| hand.calc_value());
    hands.sort_by_key(|hand| hand.value.unwrap());
    total = hands.iter().enumerate().map(|(i, hand)| hand.bid * (i as u32 + 1)).sum();

    println!("Task 7_1 Solution:\nTotal: {}", total);
}

pub fn run7_2()
{
    let data = read_data("data/data7.txt");
    // let data = TEST_DATA;
    let mut total = 0;
    let mut hands = parse_data(&data);

    hands.iter_mut().for_each(|hand| hand.calc_value_joker());
    hands.sort_by_key(|hand| hand.value.unwrap());
    total = hands.iter().enumerate().map(|(i, hand)| hand.bid * (i as u32 + 1)).sum();

    println!("Task 7_2 Solution:\nTotal: {}", total);
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    value: Option<u32>,
    bid: u32,
}

impl Hand {

    fn calc_value(&mut self) {

        let mut counts = self.count_cards();

        counts.sort();

        let amt: u32 = VALUES.len() as u32;
        let dist = amt.pow(self.cards.len() as u32);
        let sec_value = self.sec_value();
        let value = match counts.counts[0] {
            5 => 7,
            4 => 6,
            3 => match counts.counts[1] {
                    2 => 5,
                    other => 4,
            }
            2 => match counts.counts[1] {
                    2 => 3,
                    other => 2,
            },
            1 => 1,
            other => other,
        };

        self.value = Some(value * dist + sec_value);
    }

    fn calc_value_joker(&mut self) {

        let mut counts = self.count_cards_joker();

        counts.sort();
        counts.opt();

        let amt: u32 = JOKER_VALUES.len() as u32;
        let dist = amt.pow(self.cards.len() as u32);
        let sec_value = self.sec_value_joker();
        let value = match counts.counts[0] {
            5 => 7,
            4 => 6,
            3 => match counts.counts[1] {
                    2 => 5,
                    other => 4,
            }
            2 => match counts.counts[1] {
                    2 => 3,
                    other => 2,
            },
            1 => 1,
            other => other,
        };

        self.value = Some(value * dist + sec_value);
    }

    fn sec_value(&self) -> u32 {

        let mut val = 0;
        let amt: u32 = VALUES.len() as u32;

        for (i, card) in self.cards.iter().rev().enumerate() {
            val += VALUES[card] * amt.pow(i as u32) as u32;
        }

        val

    }

    fn sec_value_joker(&self) -> u32 {

        let mut val = 0;
        let amt: u32 = JOKER_VALUES.len() as u32;

        for (i, card) in self.cards.iter().rev().enumerate() {
            val += JOKER_VALUES[card] * amt.pow(i as u32) as u32;
        }

        val

    }

    fn count_cards(&self) -> Counts {

        let mut counts_unsorted = HashMap::new();

        for card in &self.cards {
            counts_unsorted.entry(*card).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut cards = Vec::new();
        let mut counts = Vec::new();
        for (card, count) in counts_unsorted {
            cards.push(card);
            counts.push(count);
        }

        let counts = Counts {
            cards,
            counts,
        };

        counts
    }

    fn count_cards_joker(&self) -> Counts {

        let mut counts_unsorted = HashMap::new();

        for card in &self.cards {
            counts_unsorted.entry(*card).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut cards = Vec::new();
        let mut counts = Vec::new();
        for (card, count) in counts_unsorted {
            cards.push(card);
            counts.push(count);
        }

        let counts = Counts {
            cards,
            counts,
        };

        counts
    }

}

struct Counts {
    cards: Vec<char>,
    counts: Vec<u32>,
}

impl Counts {

    fn sort(&mut self) {

        let mut sorted = Vec::new();

        for i in 0..self.cards.len() {
            sorted.push((self.cards[i], self.counts[i]));
        }

        sorted.sort_by(|a, b| b.1.cmp(&a.1));

        self.cards = Vec::new();
        self.counts = Vec::new();

        for (card, count) in sorted {
            self.cards.push(card);
            self.counts.push(count);
        }
    }

    fn opt(&mut self) {

        let mut jokers = 0;
        for i in 0..self.cards.len() {
            if self.cards[i] == 'J' {
                jokers += self.counts[i];
                self.cards.remove(i);
                self.counts.remove(i);
                break;
            }
        }
        if self.cards.len() == 0 {
            self.cards.push('J');
            self.counts.push(0);
        }
        self.counts[0] += jokers;
    }
}

fn parse_data(data: &str) -> Vec<Hand> {

    let mut draws = Vec::new();

    for line in data.lines() {
        draws.push(parse_draw(line));
    }

    draws
}

fn parse_draw(data: &str) -> Hand {

    let bid = data.split_whitespace().last().unwrap().parse::<u32>().unwrap();
    let cards = data.split_whitespace().next().unwrap().chars().collect::<Vec<char>>();

    Hand {
        cards,
        value: None,
        bid,
    }
}
