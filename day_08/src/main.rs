use std::fs;
use std::cmp;

fn main() {
    let filename = "trees-input.txt";
    let contents = fs::read_to_string(filename).unwrap();

    let mut trees: Vec<Vec<u32>> = Vec::new();
    for (i, line) in contents.lines().enumerate() {
        trees.push(Vec::new());
        for c in line.chars() {
            trees[i].push(c.to_digit(10).unwrap());
        }
    }

    part_1(&trees);
    part_2(&trees);
}


fn part_2(trees: &Vec<Vec<u32>>) {
    let mut max_score = 0;
    for (x, row) in trees.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            max_score = cmp::max(max_score, visibility(trees, x, y));
            // println!("x={x}, y={y}, max_score={max_score}");
        }
    }

    println!("Max visibility score is {max_score}");
}


fn part_1(trees: &Vec<Vec<u32>>) {
    let mut visible_count = 0;
    for (y, row) in trees.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if is_visible(&trees, x, y) {
                visible_count += 1;
            }
        }
    }

    println!("Visible trees: {visible_count}");
}


fn is_visible(trees: &Vec<Vec<u32>>, tree_x: usize, tree_y: usize) -> bool {
    let tree = &trees[tree_y][tree_x];
    let row = &trees[tree_y];

    if tree_x == 0 || tree_y == 0
        || tree_x == row.len()-1 || tree_y == trees.len()-1 {
        return true;
    }

    if row[0..tree_x].iter().max().unwrap() < tree {
        return true;
    }
    if row[tree_x+1..row.len()].iter().max().unwrap() < tree {
        return true;
    }

    let mut max = 0; 
    for y in 0..tree_y {
        max = cmp::max(max, trees[y][tree_x]);
    }
    if max < *tree {
        return true;
    }

    max = 0;
    for y in tree_y+1..trees.len() {
        max = cmp::max(max, trees[y][tree_x]);
    }
    if max < *tree {
        return true;
    }

    false
}


fn visibility(trees: &Vec<Vec<u32>>, tree_x: usize, tree_y: usize) -> u32 {
    let tree = &trees[tree_y][tree_x];

    let mut vis_left = 0;
    let mut vis_right = 0;
    let mut vis_up = 0;
    let mut vis_down = 0;

    // Counts down from tree_x-1
    for x in (0..tree_x).rev() {
        vis_left += 1;
        if trees[tree_y][x] >= *tree {
            break;
        }
    }

    for x in (tree_x+1)..trees[tree_y].len() {
        vis_right += 1;
        if trees[tree_y][x] >= *tree {
            break;
        }
    }

    for y in (0..tree_y).rev() {
        vis_up += 1;
        if trees[y][tree_x] >= *tree {
            break;
        }
    }

    for y in (tree_y+1)..trees.len() {
        vis_down += 1;
        if trees[y][tree_x] >= *tree {
            break;
        }
    }

    let vis_score = vis_left * vis_right * vis_up * vis_down;
    if tree_x==43 && tree_y==15 {
        println!("x={tree_x} y={tree_y} left={vis_left} right={vis_right} up={vis_up} down={vis_down} score={vis_score}");
    }
    vis_score
}