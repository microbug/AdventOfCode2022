use std::fs;
use std::collections::HashMap;


fn main() {
    let filename = "shell-log-input.txt";
    let contents = fs::read_to_string(filename).unwrap();

    let mut directories: HashMap<Vec<String>, u32> = HashMap::new();
    let mut current_directory: Vec<String> = Vec::new();
    let mut total_size_over_threshold = 0;
    let threshold = 100000;
    for line in contents.lines() {
        let mut words = line.split_ascii_whitespace();
        match words.next().unwrap() {
            "$" => {
                match words.next().unwrap() {

                    "cd" => {
                        let new_dir = words.next().unwrap();

                        if new_dir == ".." {
                            println!("Leaving directory {}", current_directory.last().unwrap());
                            let current_size
                                = directories[&current_directory];
                            if current_size <= threshold {
                                total_size_over_threshold += current_size;
                                println!("Adding directory {:?} size {} to total", current_directory, current_size);
                            }
                            println!("");

                            current_directory.pop();
                        } else {
                            println!("Entering directory {new_dir}");
                            current_directory.push(new_dir.to_string());
                        }
                    },

                    "ls" => {
                        // println!("ls found: {line}");
                    },

                    other => {
                        panic!("Unknown shell command: {other}");
                    }
                }
            },
            other => {
                match other {
                    "dir" => {
                        // Do nothing
                    },

                    file_size_str => {
                        let file_size: u32 = file_size_str.parse().unwrap();
                        let mut dir = current_directory.clone();

                        // Add file size to all parent directories
                        while dir.len() > 0 {
                            // Insert an entry with value 0 if this directory isn't already in the HashMap
                            let dir_size = directories.entry(dir.clone()).or_insert(0);
                            *dir_size += file_size;
                            println!("Added file size {} to directory {:?}", file_size, dir);
                            dir.pop();  // Go up the directory tree
                        }
                    }
                }
            }
        }
    }

    println!("Sum of sizes of all directories over {threshold}: {total_size_over_threshold}");


    // Part 2 ---------------------------------------------------
    let root_dir: Vec<String> = vec!["/".to_string()];
    let root_dir_size = directories[&root_dir];
    println!("Root filesystem size: {}", root_dir_size);
    
    let total_disk_space = 70000000;
    let min_update_space = 30000000;
    let required_root_dir_size = total_disk_space - min_update_space;
    let min_dir_size = root_dir_size - required_root_dir_size;
    
    let mut smallest_dir_size_to_free : u32 = 0xFFFFFFFF;
    for (_, size) in directories {
        if size < smallest_dir_size_to_free && size > min_dir_size {
            smallest_dir_size_to_free = size;
        }
    }
    println!("Smallest directory size that would free {} has size {}", min_dir_size, smallest_dir_size_to_free);
}
