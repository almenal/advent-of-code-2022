use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
    let input_file = fs::read_to_string(fname).expect("File not found");
    part_1(&input_file);
    part_2(&input_file);
}

fn part_1(input_file: &String) {
    let mut overlapping_sections: i32 = 0;
    for line in input_file.lines() {
        let is_overlap: bool = sections_overlap_fully(line);
        if is_overlap {
            overlapping_sections += 1;
        }
    }
    println!(
        "Part 1: Fully overlapping sections: {}",
        overlapping_sections
    );
}

fn sections_overlap_fully(elf_pair: &str) -> bool {
    let (elf_a, elf_b) = parse_to_ranges(elf_pair);
    let longer_range: &Vec<_>;
    let shorter_range: &Vec<_>;
    if elf_a[1] - elf_a[0] >= elf_b[1] - elf_b[0] {
        longer_range = &elf_a;
        shorter_range = &elf_b;
    } else {
        longer_range = &elf_b;
        shorter_range = &elf_a;
    }
    if shorter_range[0] >= longer_range[0] && shorter_range[1] <= longer_range[1] {
        true
    } else {
        false
    }
}

fn parse_to_ranges(elf_pair: &str) -> (Vec<i32>, Vec<i32>) {
    if let Some((elf_a, elf_b)) = elf_pair.split_once(",") {
        let elf_a: Vec<i32> = elf_a
            .split("-")
            .map(|x| x.parse::<i32>().expect("Failed to parse number"))
            .collect();
        let elf_b: Vec<i32> = elf_b
            .split("-")
            .map(|x| x.parse::<i32>().expect("Failed to parse number"))
            .collect();
        (elf_a, elf_b)
    } else {
        panic!("Failed to split string")
    }
}

fn part_2(input_file: &String) {
    let mut overlapping_sections: i32 = 0;
    for line in input_file.lines() {
        let is_overlap: bool = sections_overlap_partially(line);
        if is_overlap {
            overlapping_sections += 1;
        }
    }
    println!(
        "Part 2: Partially overlapping sections: {}",
        overlapping_sections
    );
}

fn sections_overlap_partially(elf_pair: &str) -> bool {
    let (elf_a, elf_b) = parse_to_ranges(elf_pair);
    let leftmost_range: &Vec<_>;
    let not_leftmost_range: &Vec<_>;
    if elf_a[0] <= elf_b[0] {
        leftmost_range = &elf_a;
        not_leftmost_range = &elf_b;
    } else {
        leftmost_range = &elf_b;
        not_leftmost_range = &elf_a;
    }

    if leftmost_range[1] >= not_leftmost_range[0] {
        true
    } else {
        false
    }
}
