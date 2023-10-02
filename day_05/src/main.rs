use std::fs;

fn main() {
    let filename = "crates-input.txt";
    let contents = fs::read_to_string(filename).unwrap();

    let part = 2;  // Choose which part of the problem to solve

    // Nine empty vectors of chars
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 9];

    let mut setup_running = true;
    let mut main_running = false;

    for line in contents.lines() {
        if line.chars().nth(1) == Some('1') {
            setup_running = false;
            for stack in &mut stacks {
                stack.reverse();  // Reverse so that bottom crate item is in index 0
                // dbg!(stack);
            }
            continue;
        }
        if !setup_running {
            if line == "" {  // ignore blank line between setup and main
                continue;
            } else {
                main_running = true;
            }
        }

        if setup_running {
            write_line_to_stacks(&line.to_string(), &mut stacks);
        } else if main_running {
            let mut words = line.split_whitespace();
            // nth(x) advances iterator by x
            let quantity = words.nth(1).unwrap().parse::<usize>().unwrap();
            let source = words.nth(1)
                .unwrap().parse::<usize>()
                .unwrap() - 1;  // 1-indexed input data
            let dest = words.nth(1)
                .unwrap().parse::<usize>()
                .unwrap() - 1;

            let new_len = stacks[source].len() - quantity;
            let mut moved_crates = stacks[source].split_off(new_len);
            if part == 1 {
                moved_crates.reverse();  // Crates are moved one-by-one
            }
            stacks[dest].append(&mut moved_crates);
        }
    }

    let mut top_crates = String::new();
    for stack in stacks {
        let last = stack.last().unwrap();
        top_crates.push_str(&format!("{last}"));
    }
    println!("Top crates: {top_crates}");
}


fn write_line_to_stacks(line: &String, stacks: &mut Vec<Vec<char>>) {
    for col in 0..9 {
        let item = line.chars().nth(1 + col*4).unwrap_or_else(|| {
            panic!("Error, col={col}, line={line}");
        });

        if item == ' ' {
            continue;
        }

        stacks[col].push(item);
    }
}
