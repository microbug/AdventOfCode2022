use std::fs;

fn main() {
    let file_path = "rucksacks-input.txt";
    let contents = fs::read_to_string(file_path).expect("couldn't read file");

    part_2(contents);
}


fn part_2(contents: String) {
    let mut priorities_sum = 0;
    let mut elves: [String; 3] = Default::default();
    let mut contents_iter = contents.lines();
    'line_loop: while let Some(e) = contents_iter.next() {
        elves[0] = e.to_string();
        elves[1] = contents_iter.next().unwrap().to_string();
        elves[2] = contents_iter.next().unwrap().to_string();
        for c in elves[0].chars() {
            if elves[1].contains(c) && elves[2].contains(c) {
                priorities_sum += get_priority(c);
                continue 'line_loop;
            }
        }

    }
    println!("Priorities sum is {}", priorities_sum);
}


fn part_1(contents: String) {
    let mut priorities_sum = 0;
    'line_loop: for line in contents.lines() {
        let split_point = line.len() / 2;
        let(first_compt, second_compt) = line.split_at(split_point);
        for c in first_compt.chars() {
            if second_compt.contains(c) {
                let delta = get_priority(c);
                // println!("Matching character: {}, adding {}", c, delta);
                priorities_sum += delta;
                continue 'line_loop;
            }
        }
    }
    println!("Sum of priorities of matching items is {}", priorities_sum);
}


fn get_priority(c: char) -> u32 {
    if c.is_uppercase() {
        let offset = 'A' as u32;
        return c as u32 - offset + 27;
    } else {
        let offset = 'a' as u32;
        return c as u32 - offset + 1;
    }
}