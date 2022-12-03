use std::fs;

fn convert_move_code(move_code: &str) -> &str {
    if move_code == "A" {
        return "ROCK";
    }
    if move_code == "B" {
        return "PAPER";
    }
    if move_code == "C" {
        return "SCISSORS";
    }

    if move_code == "X" {
        return "LOSS";
    }
    if move_code == "Y" {
        return "DRAW";
    }
    if move_code == "Z" {
        return "WIN";
    }

    return move_code;
}

fn get_your_move<'a>(their_move: &'a str, desired_result: &'a str) -> &'a str {
    if their_move == "ROCK" {
        if desired_result == "WIN" {
            return "PAPER";
        }
        if desired_result == "LOSS" {
            return "SCISSORS";
        }
    }
    if their_move == "PAPER" {
        if desired_result == "WIN" {
            return "SCISSORS";
        }
        if desired_result == "LOSS" {
            return "ROCK";
        }
    }
    if their_move == "SCISSORS" {
        if desired_result == "WIN" {
            return "ROCK";
        }
        if desired_result == "LOSS" {
            return "PAPER";
        }
    }

    return their_move;
}

fn get_move_score(their_move_raw: &str, desired_result_raw: &str) -> i32 {
    let their_move = convert_move_code(their_move_raw);
    let desired_result = convert_move_code(desired_result_raw);
    let your_move = get_your_move(their_move, desired_result);

    println!("Their move: {their_move}; Your move: {your_move}; Desired result: {desired_result}");

    let mut your_move_score = 0;

    if your_move == "ROCK" {
        your_move_score = 1;
    }
    if your_move == "PAPER" {
        your_move_score = 2;
    }
    if your_move == "SCISSORS" {
        your_move_score = 3;
    }

    if their_move == your_move {
        your_move_score += 3;
    };

    if their_move == "ROCK" {
        if your_move == "SCISSORS" {
            // LOSS
        }
        if your_move == "PAPER" {
            // WIN
            your_move_score += 6;
        }
    }

    if their_move == "PAPER" {
        if your_move == "ROCK" {
            // LOSS
        }
        if your_move == "SCISSORS" {
            // WIN
            your_move_score += 6;
        }
    }

    if their_move == "SCISSORS" {
        if your_move == "PAPER" {
            // LOSS
        }
        if your_move == "ROCK" {
            // WIN
            your_move_score += 6;
        }
    }

    return your_move_score;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input file.");
    let games: Vec<&str> = input.lines().collect();

    let mut total_move_score = 0;

    for moves_string in games {
        let game_moves: Vec<&str> = moves_string.split(" ").collect();
        let their_move = game_moves[0];
        let desired_result = game_moves[1];

        println!("Their move: {their_move}; Desired result: {desired_result}");

        total_move_score += get_move_score(their_move, desired_result);
    }

    println!("Total move score: {total_move_score}");
}
