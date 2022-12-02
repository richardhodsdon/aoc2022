use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // part1();
    part2();
}

fn part1() {
    let mut highest_cal = 0;
    let mut current_elf = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(mut calories) = line {
                if calories == "" {
                  if current_elf > highest_cal {
                    highest_cal = current_elf;
                  }

                  current_elf = 0;
                } else {
                  current_elf += calories.parse::<i32>().unwrap();
                }
                println!("line: {}", calories);
                println!("current_elf: {}", current_elf);
                println!("highest: {}", highest_cal);
            }
        }
    }
}

fn part2() {
    let mut highest_cal = [0,0,0];
    let mut current_elf = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(mut calories) = line {
                if calories == "" {
                  if current_elf > highest_cal[0] {
                    highest_cal[0] = current_elf;
                    highest_cal.sort();
                    // highest_cal.reverse();
                  }

                  current_elf = 0;
                } else {
                  current_elf += calories.parse::<i32>().unwrap();
                }
                println!("line: {}", calories);
                println!("current_elf: {}", current_elf);
                println!("highest: {:?}", highest_cal);
            }
        }
    }
    let sum: i32 = highest_cal.iter().sum();
    println!("the total sum is: {}", sum);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
