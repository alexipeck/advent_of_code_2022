use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;
use std::path::Path;

fn get_lines(path: &Path) -> Vec<String> {
    match File::open(path) {
        Ok(file) => BufReader::new(file)
            .lines()
            .map(|t| t.unwrap())
            .collect::<Vec<String>>(),
        Err(err) => {
            panic!("Error opening file: {}", err);
        }
    }
}

#[allow(dead_code)]
fn day_1_part_1(input_path: &Path) {
    {
        let mut elves: HashMap<usize, usize> = HashMap::new();
        let mut elf_counter: usize = 0;
        for line in get_lines(input_path).iter() {
            if line.is_empty() {
                elf_counter += 1;
            } else {
                *elves.entry(elf_counter).or_insert(0) += line.parse::<usize>().unwrap();
            }
        }
        println!("{}", elves.values().max().unwrap());
    }
    
    {
        let mut elves: [usize; 236] = [0; 236];
        let mut elf_counter: u16 = 0;
        for line in get_lines(input_path).iter() {
            if line.is_empty() {
                elf_counter += 1;
            } else {
                elves[elf_counter as usize] += line.parse::<usize>().unwrap();
            }
        }
        println!("{}", elves.iter().max().unwrap());
    }

    {
        let mut elves: Vec<usize> = Vec::new();
        let mut elf_counter: u16 = 0;
        for line in get_lines(input_path).iter() {
            if line.is_empty() {
                elf_counter += 1;
            } else {
                if elves.get(elf_counter as usize).is_none() {
                    elves.insert(elf_counter as usize, 0)
                };
                elves[elf_counter as usize] += line.parse::<usize>().unwrap();
            }
        }
        println!("{}", elves.iter().max().unwrap());
    }
}

#[allow(dead_code)]
fn day_1_part_2(input_path: &Path) {
    let mut elves: Vec<usize> = Vec::new();
    let mut elf_counter: u16 = 0;
    for line in get_lines(input_path).iter() {
        if line.is_empty() {
            elf_counter += 1;
        } else {
            if elves.get(elf_counter as usize).is_none() {
                elves.insert(elf_counter as usize, 0)
            };
            elves[elf_counter as usize] += line.parse::<usize>().unwrap();
        }
    }
    elves.sort_unstable();
    let top_3_sum: usize = elves.iter().rev().take(3).sum();
    println!("{}", top_3_sum);
}

fn main() {
    let current_directory = env::current_dir().unwrap();
    //day_1_part_1(&current_directory.join("input_day_1.txt"));
    day_1_part_2(&current_directory.join("input_day_1.txt"));
}
