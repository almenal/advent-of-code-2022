use std::env;
use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Strategy {
    Lose,
    Draw,
    Win,
}

const GAME_WON: i32 = 6;
const GAME_DRAW: i32 = 3;
const GAME_LOST: i32 = 0;
const VALUE_ROCK: i32 = 1;
const VALUE_PAPER: i32 = 2;
const VALUE_SCISSORS: i32 = 3;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path: &String = &args[1];
    let variant: &String = &args[2];

    let game_plan: String = fs::read_to_string(input_path).expect("Error while reading file");
    let game_plan = game_plan.trim();

    let mut total_points: i32 = 0;
    for round in game_plan.split("\n") {
        let round_points = get_round_points(round, variant);
        total_points += round_points;
    }
    println!("{}", total_points);
}

fn get_round_points(round: &str, variant: &str) -> i32 {
    let round_movements: Vec<&str> = round.split(" ").collect();
    let opponent_move = round_movements[0];
    let my_move = round_movements[1];

    let my_move_value = value_move(my_move, variant);
    let game_result = resolve_game(opponent_move, my_move, variant);
    game_result + my_move_value
}

fn value_move(my_move: &str, variant: &str) -> i32 {
    if variant == "v1" {
        match my_move {
            "X" => VALUE_ROCK,
            "Y" => VALUE_PAPER,
            "Z" => VALUE_SCISSORS,
            _ => panic!("Unrecognised move, should be one of [X,Y,Z]"),
        }
    } else if variant == "v2" {
        match my_move {
            "X" => GAME_LOST,
            "Y" => GAME_DRAW,
            "Z" => GAME_WON,
            _ => panic!("Unrecognised move, should be one of [X,Y,Z]"),
        }
    } else {
        panic!("Unrecognized variant '{}'", variant)
    }
}

fn resolve_game(opponent_move: &str, my_move: &str, variant: &str) -> i32 {
    match variant {
        "v1" => resolve_known_hand(opponent_move, my_move),
        "v2" => resolve_known_outcome(opponent_move, my_move),
        _ => panic!("Unrecognized variant '{}'", variant),
    }
}

fn resolve_known_hand(opponent_move: &str, my_move: &str) -> i32 {
    let opponent_decoded: Move = decode_move(opponent_move);
    let my_move_decoded: Move = decode_move(my_move);
    if opponent_decoded == my_move_decoded {
        return GAME_DRAW;
    }
    let round: [Move; 2] = [opponent_decoded, my_move_decoded];
    match round {
        [Move::Rock, Move::Scissors] => GAME_LOST,
        [Move::Rock, Move::Paper] => GAME_WON,
        [Move::Paper, Move::Rock] => GAME_LOST,
        [Move::Paper, Move::Scissors] => GAME_WON,
        [Move::Scissors, Move::Rock] => GAME_WON,
        [Move::Scissors, Move::Paper] => GAME_LOST,
        _ => panic!("This case should have been handled"),
    }
}

fn decode_move(move_: &str) -> Move {
    match move_ {
        "A" | "X" => Move::Rock,
        "B" | "Y" => Move::Paper,
        "C" | "Z" => Move::Scissors,
        _ => panic!("Unrecognised move, should be one of [X,Y,Z]"),
    }
}

fn resolve_known_outcome(opponent_move: &str, my_move: &str) -> i32 {
    let opponent_decoded: Move = decode_move(opponent_move);
    let my_strat_decoded: Strategy = decode_strategy(my_move);
    let round: (Move, Strategy) = (opponent_decoded, my_strat_decoded);
    match round {
        (Move::Rock, Strategy::Lose) => VALUE_SCISSORS,
        (Move::Rock, Strategy::Draw) => VALUE_ROCK,
        (Move::Rock, Strategy::Win) => VALUE_PAPER,
        (Move::Paper, Strategy::Lose) => VALUE_ROCK,
        (Move::Paper, Strategy::Draw) => VALUE_PAPER,
        (Move::Paper, Strategy::Win) => VALUE_SCISSORS,
        (Move::Scissors, Strategy::Lose) => VALUE_PAPER,
        (Move::Scissors, Strategy::Draw) => VALUE_SCISSORS,
        (Move::Scissors, Strategy::Win) => VALUE_ROCK,
    }
}

fn decode_strategy(move_: &str) -> Strategy {
    match move_ {
        "X" => Strategy::Lose,
        "Y" => Strategy::Draw,
        "Z" => Strategy::Win,
        _ => panic!("Unrecognised move, should be one of [X,Y,Z]"),
    }
}
