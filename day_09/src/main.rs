use std::fs;
use std::ops;


#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    x: i32,
    y: i32
}


impl ops::Add for Position {
    type Output = Position;

    fn add (self, rhs: Position) -> Position {
        println!("{:?}.add{:?} was called", self, rhs);
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl ops::AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        };
    }
}

impl ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl ops::Mul<i32> for Position {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}


fn main() {
    let filename = "motion-input.txt";
    let contents = fs::read_to_string(filename).unwrap();
    part_2(contents);
}


fn part_2(contents: String) {
    let mut rope: Vec<Position> = vec![Position{x: 0, y: 0}; 10];
    let mut positions_visited: Vec<Position> = Vec::new();

    for line in contents.lines() {
        let mut words = line.split_ascii_whitespace();
        let direction = words.next().unwrap();
        let head_displacement_direction;
        match direction {
            "L" => head_displacement_direction = Position{x: -1, y: 0},
            "R" => head_displacement_direction = Position{x: 1, y: 0},
            "U" => head_displacement_direction = Position{x: 0, y: 1},
            "D" => head_displacement_direction = Position{x: 0, y: -1},
            other => panic!("Unknown displacement direction {other}"),
        }
        let distance: i32 = words.next().unwrap().parse().unwrap();

        for _ in 0..distance {
            rope[0] += head_displacement_direction;
            for i in 1..rope.len() {
                // segment is the current rope element being processed
                // rope[0] is the head of the rope

                let seg_move = calculate_segment_move(&rope[i-1], &rope[i]);
                rope[i] += seg_move;

                if i == rope.len() - 1 && !positions_visited.contains(&rope[i]) {
                    positions_visited.push(rope[i]);
                }
            }
        }
    }

    println!("{} positions visited by last element of rope", positions_visited.len());
}

fn part_1(contents: String) {
    let mut head = Position {x: 0, y: 0};
    let mut tail = head;

    let mut positions_visited: Vec<Position> = Vec::new();

    for line in contents.lines() {
        let mut words = line.split_ascii_whitespace();
        let head_displacement_direction: Position;
        let direction = words.next().unwrap();
        match direction {
            "L" => head_displacement_direction = Position{x: -1, y: 0},
            "R" => head_displacement_direction = Position{x: 1, y: 0},
            "U" => head_displacement_direction = Position{x: 0, y: 1},
            "D" => head_displacement_direction = Position{x: 0, y: -1},
            other => panic!("Unknown displacement direction {other}"),
        }
        let distance: i32 = words.next().unwrap().parse().unwrap();

        for _ in 0..distance {
            head += head_displacement_direction;

            // let head_tail_displacement = head - tail;
            // println!("\nHead={:?}, displacement={:?}, tail={:?}", head, head_tail_displacement, tail);

            let tail_move = calculate_segment_move(&head, &tail);
            tail += tail_move;

            if tail_move != (Position{x: 0, y: 0}) {
                println!("Tail moved by {:?} to {:?}", tail_move, tail);
            }

            if !positions_visited.contains(&tail) {
                positions_visited.push(tail);
                println!("New tail position added");
            }
        }
    }

    println!("Tail positions visited: {}", positions_visited.len());
}


fn calculate_segment_move(head: &Position, tail: &Position) -> Position {
    let head_tail_displacement = *head - *tail;
    let mut tail_move = Position{x: 0, y: 0};
    if head_tail_displacement.x == 0 {
        if head_tail_displacement.y > 1 {
            tail_move.y = 1;
        } else if head_tail_displacement.y < -1 {
            tail_move.y = -1;
        }
    } else if head_tail_displacement.y == 0 {
        if head_tail_displacement.x > 1 {
            tail_move.x = 1;
        } else if head_tail_displacement.x < -1 {
            tail_move.x = -1
        }
    } else if !touching(head, tail) {
        // Move one step diagonally closer to head
        tail_move = Position {x: 1, y: 1};

        if head_tail_displacement.x.is_negative() {
            tail_move.x = -1;
        }
        if head_tail_displacement.y.is_negative() {
            tail_move.y = -1;
        }
    }

    tail_move
}


fn touching(a: &Position, b: &Position) -> bool {
    let difference = *a - *b;
    if difference.x.abs() > 1 || difference.y.abs() > 1 {
        return false;
    }
    true
}