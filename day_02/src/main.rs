use std::fs;
use std::collections::HashMap;

fn main() {
    part_2();
}


fn part_2() {
    let file_path = "strategy-guide.txt";
    let contents = fs::read_to_string(file_path).expect("couldn't read file");

    let shape_score = HashMap::from([
        ("Rock", 1),
        ("Paper", 2),
        ("Scissors", 3)
    ]);

    let opponent_map = HashMap::from([
        ("A", "Rock"),
        ("B", "Paper"),
        ("C", "Scissors")
    ]);

    // R loses to L
    let loss_map = HashMap::from([
        ("Rock", "Scissors"),
        ("Scissors", "Paper"),
        ("Paper", "Rock"),
    ]);

    // R wins over L
    let win_map = HashMap::from([
        ("Scissors", "Rock"),
        ("Paper", "Scissors"),
        ("Rock", "Paper"),
    ]);

    let mut score = 0;
    for line in contents.lines() {
        let mut plays = line.split(" ");
        let opponent_play = opponent_map[plays.next().unwrap()];
        let my_play;
        match plays.next().unwrap() {
            "X" => {  // Loss
                my_play = loss_map[opponent_play];
            },
            "Y" => {  // Draw
                my_play = opponent_play.clone();
                score += 3;
            },
            "Z" => {  // Win
                my_play = win_map[opponent_play];
                score += 6;
            },
            other => {
                panic!("Unexpected value {}", other);
            }
        }
        score += shape_score[my_play];

        // println!("Played {} against {}. New score: {}, shape score: {}", my_play, opponent_play, score, shape_score[my_play]);
    }

    println!("Total score: {}", score);
}


fn part_1() {
    let file_path = "strategy-guide.txt";
    let contents = fs::read_to_string(file_path).expect("couldn't read file");

    let shape_score = HashMap::from([
        ("Rock", 1),
        ("Paper", 2),
        ("Scissors", 3)
    ]);

    let opponent_map = HashMap::from([
        ("A", "Rock"),
        ("B", "Paper"),
        ("C", "Scissors")
    ]);

    let my_map = HashMap::from([
        ("X", "Rock"),
        ("Y", "Paper"),
        ("Z", "Scissors")
    ]);

    // L beats R
    let rules = HashMap::from([
        ("Rock", "Scissors"),
        ("Scissors", "Paper"),
        ("Paper", "Rock"),
    ]);

    let mut score = 0;
    for line in contents.lines() {
        let mut plays = line.split(" ");
        let opponent_play = opponent_map[plays.next().unwrap()];
        let my_play = my_map[plays.next().unwrap()];

        if rules[my_play] == opponent_play {
            score += 6;  // Win
        } else if rules[opponent_play] != my_play {
            score += 3;  // Draw
        }  // No points for a loss

        score += shape_score[my_play];

        // println!("Played {} against {}. New score: {}, shape score: {}", my_play, opponent_play, score, shape_score[my_play]);
    }

    println!("Total score: {}", score);
}
