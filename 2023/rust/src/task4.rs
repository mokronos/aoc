use std::collections::HashMap;

use rust::read_data;

#[allow(unused)]
const TEST_DATA: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

pub fn run4_1()
{
    let data = read_data("data/data4.txt");
    // let data = TEST_DATA;
    let mut total = 0;

    let mut cards: Vec<Card> = Vec::new();

    for line in data {
        cards.push(parse_data(&line));
    }

    for card in cards {
        let points = card.check();
        total += points;
    }

    println!("Task 4_1 Solution:\nTotal: {}", total);
}

pub fn run4_2()
{
    let data = read_data("data/data4.txt");
    // let data = TEST_DATA;
    let mut total = 0;

    let mut cards: Vec<Card> = Vec::new();

    for line in data {
        cards.push(parse_data(&line));
    }

    let mut counts: HashMap<u32, u32> = HashMap::new();

    for card in cards {
        let winners = card.winners();
        for winner in (card.id + 1) .. (card.id + 1 + winners as u32) {
            let cur = counts.get(&winner).unwrap_or(&1);
            counts.insert(winner, cur + counts.get(&card.id).unwrap_or(&1));
        }
        total += counts.get(&card.id).unwrap_or(&1);
    }

    println!("Task 4_2 Solution:\nTotal: {}", total);
}

struct Card
{
    pub id: u32,
    pub correct: Vec<u32>,
    pub drawn: Vec<u32>,
}

impl Card {
    fn check(&self) -> usize {
        let mut count:u32 = 0;
        for num in &self.drawn {
            if self.correct.contains(num) {
                count += 1;
            }
        }
        if count == 0 {
            return 0;
        }
        let base:usize = 2;
        let points = 1 * base.pow(count-1);
        points
    }
}

impl Card {
    fn winners(&self) -> usize {
        let mut winners:usize = 0;
        for num in &self.drawn {
            if self.correct.contains(num) {
                winners += 1;
            }
        }
        winners
    }
}


fn parse_data(line: &str) -> Card {
    let mut card = Card {
        id: 0,
        correct: Vec::new(),
        drawn: Vec::new(),
    };

    let id = line.split(":").next().unwrap().split_whitespace().nth(1).unwrap();
    card.id = id.parse::<u32>().unwrap();

    let correct = line.split(":").nth(1).unwrap().split("|").next().unwrap().trim();
    for num in correct.split_whitespace() {
        card.correct.push(num.parse::<u32>().unwrap());
    }
    let drawn = line.split("|").last().unwrap().trim();
    for num in drawn.split_whitespace() {
        card.drawn.push(num.parse::<u32>().unwrap());
    }

    card
}
