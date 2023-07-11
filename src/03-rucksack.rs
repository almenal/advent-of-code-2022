use std::env;
use std::fs;

fn main() {
    let file_contents: String = read_input_from_args();
    solve_part_1(&file_contents);
    solve_part_2(&file_contents);
}

fn solve_part_1(file_contents: &String) {
    let mut total_priority: u16 = 0;
    for line in file_contents.lines() {
        let repeated_letter = get_repeated_letter(line);
        let priority = map_priority(repeated_letter);
        total_priority += priority;
        // println!(
        //     "Duplicated letter: {} Priority: {}",
        //     repeated_letter, priority
        // );
    }
    println!("Total priority I: {}", total_priority);
}

fn read_input_from_args() -> String {
    let args: Vec<String> = env::args().collect();
    let input_path: &String = &args[1];
    fs::read_to_string(input_path).expect("Failed to read file")
}

fn get_repeated_letter(s: &str) -> char {
    // Rust handles strings cautiously because UTF8,
    // but here we don't have to worry about that
    let str_len = s.len();
    let first_half = &s[0..(str_len / 2)];
    let second_half = &s[(str_len / 2)..str_len];
    for c in first_half.chars() {
        if second_half.contains(c) {
            return c;
        }
    }
    panic!(
        "No letters in common between '{}' and '{}' (from '{}')",
        first_half, second_half, s,
    );
}

fn map_priority(letter: char) -> u16 {
    match letter.is_uppercase() {
        false => (letter as u16) - ('a' as u16) + 1,
        true => (letter as u16) - ('A' as u16) + 27,
    }
}

fn solve_part_2(file_contents: &String) {
    // for group in file_contents.lines().array_chunks::<3>() {
    let lines: Vec<&str> = file_contents.lines().collect();
    let n_group = lines.len() / 3;
    let mut total_priority: u16 = 0;
    for group_idx in 0..n_group {
        let start = 3 * group_idx;
        let stop = start + 3;
        let group: Vec<&str> = lines[start..stop].to_vec();
        let common_letter = match get_common_letter(group) {
            Ok(c) => c,
            Err(e) => panic!("{}", e),
        };
        let group_priority = map_priority(common_letter);
        total_priority += group_priority;
        // println!(
        //     "Common letter: {} Priority: {}",
        //     common_letter, group_priority
        // );
    }
    println!("Total priority II: {}", total_priority);
}

fn get_common_letter(group: Vec<&str>) -> Result<char, &str> {
    let unique_letters = get_unique_letters(&group);
    for letter in unique_letters {
        let mut letter_is_common = true;
        for member in &group {
            if !member.contains(letter) {
                letter_is_common = false;
            }
        }
        if letter_is_common {
            return Ok(letter);
        }
    }
    Err("Could not find a letter in common")
}

fn get_unique_letters(group: &Vec<&str>) -> Vec<char> {
    let mut unique_letters: Vec<char> = Vec::new();
    for member in group {
        for c in member.chars() {
            if !unique_letters.contains(&c) {
                unique_letters.push(c);
            }
        }
    }
    unique_letters.sort();
    // println!(
    //     "Group: {} \nUnique letters: {:?}",
    //     group.join("-"),
    //     unique_letters
    // );
    unique_letters
}
