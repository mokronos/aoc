use rust::read_data;
use std::cmp::max;

pub fn run2_1()
{
    let data = read_data("data/data2.txt");
    let mut total = 0;

    let real = Reality {
        red: 12,
        green: 13,
        blue: 14, 
    };


    for line in &data{
        let game = parse_game(line);

        if check_round(&game, &real) {
            total += game.id;
        }
    }

    println!("Task 2_1 Solution:\nTotal: {}", total);
}

pub fn run2_2()
{
    let data = read_data("data/data2.txt");
    let mut total = 0;

    for line in &data{
        let game = parse_game(line);

        let power = get_power(&game);
        total += power;
    }

    println!("Task 2_2 Solution:\nTotal: {}", total);
}

struct Game {
    id: usize,
    rounds: Vec<Round>,
}

struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

struct Reality {
    red: usize,
    green: usize,
    blue: usize,
}

fn parse_game(line: &str) -> Game {
    let mut line = line.split(":");
    let left = line.next().unwrap();
    let id = left.split(" ").last().unwrap().parse::<usize>().unwrap();
    let right = line.next().unwrap();
    let mut rounds = Vec::new();

    for round in right.split(";") {
        let round_parsed = parse_round(round);
        rounds.push(round_parsed);
    }
        
    Game {
        id: id,
        rounds: rounds,
    }
}
    

fn parse_round(input: &str) -> Round {

    let mut number = String::new();
    let mut i = 0;
    let mut amount = 0;
    let mut result = Round {
        blue: 0,
        red: 0,
        green: 0,
    };
    let mut color = String::new();
    let input = input.trim().chars().collect::<Vec<char>>();
    while i < input.len() {
        if input[i].is_numeric() {
            number.push(input[i]);
        }
        if input[i] == ' ' && number.len() > 0 {
            amount = number.parse::<usize>().unwrap();
            number = String::new();
        }

        if input[i] == ',' {
            if color == "blue" {
                result.blue = amount;
            }
            if color == "red" {
                result.red = amount;
            }
            if color == "green" {
                result.green = amount;
            }

            amount = 0;
            color = String::new();
        }
        if input[i].is_alphabetic() {
            color.push(input[i]);
        }
        i += 1;
    }

    if color == "blue" {
        result.blue = amount;
    }
    if color == "red" {
        result.red = amount;
    }
    if color == "green" {
        result.green = amount;
    }

    result
}    

fn check_round(game: &Game, real: &Reality) -> bool {

    let len = game.rounds.len();

    for i in 0..len {
        if game.rounds[i].blue > real.blue {
            return false;
        }
        if game.rounds[i].red > real.red {
            return false;
        }
        if game.rounds[i].green > real.green {
            return false;
        }
    }
    true
}
    

fn get_power(game: &Game) -> usize {
    let (mut red, mut green, mut blue) = (0, 0, 0);

    for round in &game.rounds {
        red = max(red, round.red);
        green = max(green, round.green);
        blue = max(blue, round.blue);
    }

    red * green * blue
}
