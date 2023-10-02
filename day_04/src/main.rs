use std::fs;


fn main() {
    let file_path = "assignments.txt";
    let contents = fs::read_to_string(file_path).expect("couldn't read file");

    part_2(contents);
}

fn part_2(contents: String) {
    let mut total_pairs = 0;
    let mut useful_pairs = 0;
    for line in contents.lines() {
        let sections_str = line.split([',', '-']);
        let sections: Vec<u32> = sections_str.map(|x| x.parse::<u32>().unwrap()).collect();
        // elf 1 is sections[0..2], elf 2 is sections[2..4]
        total_pairs += 1;
        if sections[1] < sections[2] {
            useful_pairs += 1;
        } else if sections[0] > sections[3] {
            useful_pairs += 1;
        }
    }
    let redundant_pairs = total_pairs - useful_pairs;
    println!("{} redundant pairs", redundant_pairs);
}

fn part_1(contents: String) {
    let mut redundant_pairs = 0;
    for line in contents.lines() {
        let sections_str = line.split([',', '-']);
        let sections: Vec<u32> = sections_str.map(|x| x.parse::<u32>().unwrap()).collect();
        // elf 1 is sections[0..2], elf 2 is sections[2..4]
        if sections[0] <= sections[2] && sections[1] >= sections[3] {
            redundant_pairs += 1;
        } else if sections[2] <= sections[0] && sections[3] >= sections[1] {
            redundant_pairs += 1;
        }
    }
    println!("{} redundant pairs", redundant_pairs);
}
