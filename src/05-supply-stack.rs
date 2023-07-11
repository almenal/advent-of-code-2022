use std::collections::HashMap;
use std::env;
use std::fs;

type StackMap = HashMap<i32, Vec<String>>;

fn main() {
    let fname: &String = &env::args().collect::<Vec<String>>()[1];
    let (stack_map, instructions) = parse_instructions(fname);
}

fn parse_instructions(fname: &String) -> (StackMap, Vec<String>) {
    let instructions = fs::read_to_string(fname).expect("Failed to read input");
    let mut stack_map = HashMap::new();
    let mut instructions = Vec::new();

    parse_stacks(&instructions, &mut stack_map);
    (stack_map, instructions)
}

fn parse_stacks(instructions: &String, stack_map: &mut StackMap) {
    // Let's do the lazy assumption that all lines that define the
    // initial stack configuration have the same number of characters
    for line in instructions.lines() {
        if has_numbers_only(line) {
            break;
        }
        let offset = 0;
        for c in line.chars() {}
    }
}

fn has_numbers_only(line: &str) -> bool {
    // Numbers in ASCII go from 48 (0) to 57 (9)
    let no_spaces = line.replace(" ", "");
    for c in no_spaces.chars() {
        match c as u8 {
            48..=57 => continue,
            _ => return false,
        };
    }
    return true;
}
