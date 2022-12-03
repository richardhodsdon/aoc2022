use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    let score_map: HashMap<char, u32> = HashMap::from([
        ('a', 1),  ('A', 27),
        ('b', 2),  ('B', 28),
        ('c', 3),  ('C', 29),
        ('d', 4),  ('D', 30),
        ('e', 5),  ('E', 31),
        ('f', 6),  ('F', 32),
        ('g', 7),  ('G', 33),
        ('h', 8),  ('H', 34),
        ('i', 9),  ('I', 35),
        ('j', 10), ('J', 36),
        ('k', 11), ('K', 37),
        ('l', 12), ('L', 38),
        ('m', 13), ('M', 39),
        ('n', 14), ('N', 40),
        ('o', 15), ('O', 41),
        ('p', 16), ('P', 42),
        ('q', 17), ('Q', 43),
        ('r', 18), ('R', 44),
        ('s', 19), ('S', 45),
        ('t', 20), ('T', 46),
        ('u', 21), ('U', 47),
        ('v', 22), ('V', 48),
        ('w', 23), ('W', 49),
        ('x', 24), ('X', 50),
        ('y', 25), ('Y', 51),
        ('z', 26), ('Z', 52),
    ]);

    let mut total = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(backpack) = line {
                let midway = backpack.len() / 2;
                let first = &backpack[..midway];
                let second = &backpack[midway..];

                let common_char = second.chars().find(|c| first.contains(*c));
                let score = score_map[&common_char.unwrap()];

                // println!("{}", first);
                // println!("{}", second);
                // println!("common_char: {:?}:{}", common_char, score);
                total += score;
            }
        }
    }
    println!("Part 1 Total: {}", total);
}

fn part2() {
let score_map: HashMap<char, u32> = HashMap::from([
        ('a', 1),  ('A', 27),
        ('b', 2),  ('B', 28),
        ('c', 3),  ('C', 29),
        ('d', 4),  ('D', 30),
        ('e', 5),  ('E', 31),
        ('f', 6),  ('F', 32),
        ('g', 7),  ('G', 33),
        ('h', 8),  ('H', 34),
        ('i', 9),  ('I', 35),
        ('j', 10), ('J', 36),
        ('k', 11), ('K', 37),
        ('l', 12), ('L', 38),
        ('m', 13), ('M', 39),
        ('n', 14), ('N', 40),
        ('o', 15), ('O', 41),
        ('p', 16), ('P', 42),
        ('q', 17), ('Q', 43),
        ('r', 18), ('R', 44),
        ('s', 19), ('S', 45),
        ('t', 20), ('T', 46),
        ('u', 21), ('U', 47),
        ('v', 22), ('V', 48),
        ('w', 23), ('W', 49),
        ('x', 24), ('X', 50),
        ('y', 25), ('Y', 51),
        ('z', 26), ('Z', 52),
    ]);

    let mut total = 0;
    let mut group_counter = 0;
    let mut groups_bp = Vec::new();

    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(backpack) = line {
                group_counter += 1;
                groups_bp.push(backpack);
                groups_bp.sort_by(|a, b| b.len().cmp(&a.len())); // sort longest first

                if group_counter % 3 == 0 {
                    let mut common_char: char = '$';
                    for c in groups_bp[0].chars() {
                        for c1 in groups_bp[1].chars() {
                            if c == c1 {
                                // check if its in the last group
                                for c2 in groups_bp[2].chars() {
                                    if c1 == c2 {
                                        // we found it
                                        common_char = c;
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    let score = score_map[&common_char];


                    // println!("char: {}, score: {}, 3 groups {:?}",common_char, score, groupsBP);
                    groups_bp = Vec::new();
                    total += score;
                }

                // let midway = backpack.len() / 2;
                // let first = &backpack[..midway];
                // let second = &backpack[midway..];
                // let second = &backpack[midway..];

                // let common_char = second.chars().find(|c| first.contains(*c));
                // let score = score_map[&common_char.unwrap()];

                // println!("{}", first);
                // println!("{}", second);
                // println!("common_char: {:?}:{}", common_char, score);

            }
        }
    }
    println!("Part 2 Total: {}", total);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
