use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


const ROCK: char = 'A';
const PAPER: char = 'B';
const SCISSORS: char = 'C';
const ROCK_RESPONSE: char = 'X';
const PAPER_RESPONSE: char = 'Y';
// const SCISSORS_RESPONSE: char = 'Z';
const LOST: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;

const NEED_TO_LOSE: char = 'X';
const NEED_TO_DRAW: char = 'Y';
const NEED_TO_WIN: char = 'Z';

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut total = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(round) = line {
                let chars: Vec<_> =round.chars().collect();
                total +=  score(chars[0], convert_to_same(chars[2]));
                // println!("round score: {}", score(chars[0], convert_to_same(chars[2])));
            }
        }
    }
    println!("total score: {}", total);
}

fn part2() {
    let mut total = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(round) = line {
                let chars: Vec<_> =round.chars().collect();
                total +=  score(chars[0], convert_to_result(chars[0], chars[2]));
                // println!("round score: {}", score(chars[0], convert_to_same(chars[2])));
            }
        }
    }
    println!("total score: {}", total);
}

fn convert_to_same(mine: char) -> char {
    if mine == ROCK_RESPONSE {
       return ROCK
    }
    if mine == PAPER_RESPONSE {
        return PAPER
    }
    SCISSORS
}

fn convert_to_result(opp:char, required_result: char) -> char {
    return match required_result {
        NEED_TO_LOSE => {
            match opp {
                ROCK => SCISSORS,
                SCISSORS => PAPER,
                PAPER => ROCK,
                _ => opp
            }
        },
        NEED_TO_DRAW => opp,
        NEED_TO_WIN => {
            match opp {
                ROCK => PAPER,
                SCISSORS => ROCK,
                PAPER => SCISSORS,
                _ => opp
            }
        },
        _ => ROCK,
    };
}

fn score(opponent: char, mine: char) -> i32 {
    let score_map: HashMap<char, i32> = HashMap::from([
        (ROCK, 1),
        (PAPER, 2),
        (SCISSORS, 3),
    ]);

    let mut result: i32 = score_map[&mine];
    // draw condition
    if opponent == mine {
        result += DRAW;
        return result;
    }

    match opponent {
        ROCK => {
            if mine == PAPER {
                // win
                result += WIN;
            } else {
                // lost
                result += LOST;
            }
        },
        PAPER => {
            if mine == SCISSORS {
                // win
                result += WIN;
            } else {
                // lost
                result += LOST;
            }
        },
        SCISSORS => {
            if mine == ROCK {
                // win
                result += WIN;
            } else {
                // lost
                result += LOST;
            }
        },
        _ => println!("None"),
    }

    result
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
