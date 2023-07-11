use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file_contents = fs::read_to_string(file_path).expect("Error while reading file");
    let maximum_calories = get_maximum_calories(file_contents);
    println!("{}", maximum_calories);
}

fn get_maximum_calories(calories_list: String) -> i32 {
    let calories_per_elf: Vec<&str> = calories_list.split("\n\n").collect();
    let mut max_calories: i32 = 0;
    for calorie_group in calories_per_elf {
        // let total_calories: i32 = calorie_group.split("\n").map(|x| x.parse()).sum();
        let total_calories: i32 = get_elf_calories(calorie_group);
        if total_calories > max_calories {
            max_calories = total_calories;
        }
    }
    max_calories
}

fn get_elf_calories(calorie_group: &str) -> i32 {
    let mut calorie_sum: i32 = 0;
    let calories_split: Vec<&str> = calorie_group.trim().split("\n").collect();
    for calories in calories_split {
        //println!("{}", calories);
        let calories: i32 = calories.parse().expect("This should have been a number");
        calorie_sum += calories;
    }
    calorie_sum
}
