use std::fs;


fn main() {
    let filename = "databuffer-input.txt";
    let contents = fs::read_to_string(filename).unwrap();

    let mut fifo: Vec<char> = vec!['*'; 14];

    for (i, c) in contents.chars().enumerate() {
        let mut all_unique = true;
        for d in &fifo {
            if *d == '*' {  // Ignore placeholder values
                all_unique = false;
            }
            if fifo.iter().filter(|&n| *n == *d).count() > 1 {
                all_unique = false;
            }
        }
        if all_unique {
            println!("Start of packet at position {}", i);
            return;
        }
        fifo.insert(0, c);
        fifo.pop();
    }
}
