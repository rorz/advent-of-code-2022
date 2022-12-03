use std::fs;

fn convert_move_code(move_code: &str) -> &str {
    if move_code == "A" || move_code == "X" {
        return "ROCK";
    }
    if move_code == "B" || move_code == "Y" {
        return "PAPER";
    }
    if move_code == "C" || move_code == "Z" {
        return "SCISSORS";
    }

    return move_code;
}

fn get_move_score(their_move_raw: &str, your_move_raw: &str) -> i32 {
    let their_move = convert_move_code(their_move_raw);
    let your_move = convert_move_code(your_move_raw);


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
        let your_move = game_moves[1];

        total_move_score += get_move_score(their_move, your_move);
    }

    println!("Total move score: {total_move_score}");
}
