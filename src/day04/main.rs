use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut total = 0;

    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(assignments) = line {
                let elves: Vec<&str> = assignments.split(',').collect();
                let elf_1_times: Vec<u16> = elves[0].split('-').map(|x| x.parse::<u16>().unwrap()).collect();
                let elf_2_times: Vec<u16> = elves[1].split('-').map(|x| x.parse::<u16>().unwrap()).collect();
                // println!("1: {:?}, 2:{:?}", elf_1_times, elf_2_times);

                if inclusive(elf_1_times, elf_2_times) {
                    total += 1;
                    // println!("1: {:?}", elves);

                    // println!("Inclusive")
                }
            }
        }
    }

    println!("Part 1 Total: {}", total);
}

fn part2() {
    let mut total = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(assignments) = line {
                let elves: Vec<&str> = assignments.split(',').collect();
                let elf_1_times: Vec<u16> = elves[0].split('-').map(|x| x.parse::<u16>().unwrap()).collect();
                let elf_2_times: Vec<u16> = elves[1].split('-').map(|x| x.parse::<u16>().unwrap()).collect();
                // println!("1: {:?}, 2:{:?}", elf_1_times, elf_2_times);

                if inclusive(elf_1_times.clone(), elf_2_times.clone()) || overlap(elf_1_times.clone(), elf_2_times.clone()) {
                    total += 1;
                    // println!("1: {:?}", elves);

                    // println!("Inclusive")
                }
            }
        }
    }
    println!("Part 2 Total: {}", total);
}

fn inclusive(vec1: Vec<u16>, vec2: Vec<u16>) -> bool {
    if vec1[0] >= vec2[0] && vec1[1] <= vec2[1] {
        // println!("true1 inc: {} >= {} && {} <= {}", vec1[0], vec2[0], vec1[1], vec2[1]);
        return true;
    }

    if vec1[0] <= vec2[0] && vec1[1] >= vec2[1] {
        // println!("true2 inc: {} <= {} && {} >= {}", vec1[0], vec2[0], vec1[1], vec2[1]);
        return true;
    }

    return false;
}

fn overlap(vec1: Vec<u16>, vec2: Vec<u16>) -> bool {
    if vec1[0] < vec2[0] && vec1[1] >= vec2[0] {
        // println!("true1 overlap: {} < {} && {} >= {}", vec1[0], vec2[0], vec1[1], vec2[1]);
        return true;
    }
    if vec1[0] <= vec2[1] && vec1[1] > vec2[1] {
        // println!("true2 overlap: {} >= {} && {} > {}", vec1[0], vec2[0], vec1[1], vec2[1]);
        return true;
    }

    return false;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
