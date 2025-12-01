enum Direction {
    Left,
    Right,
}

fn rotate(curr: &mut i32, n: i32, direction: Direction) {
    let units = n;

    match direction {
        Direction::Left => {
            *curr -= units;
            while *curr < 0 {
                *curr += 100;
            }
        },

        Direction::Right => {
            *curr += units;
            while *curr > 99 {
                *curr -= 100;
            }
        }
    }
}

fn count_ticked(curr: &mut i32, n: i32, direction: Direction) -> u32 {
    let mut f; // distance to boundary
    match direction {
        Direction::Right =>  {
            f = (100 - *curr) % 100;
        },
        Direction::Left => {
            f = *curr % 100;
        }
    }

    if f == 0 {
        // if calculated distance is 0, set to 100
        f = 100;
    }

    let mut tick_count = 0;

    if f <= n {
        tick_count = 1 + ((n - f) / 100);
    }

    *curr = match direction {
        Direction::Right => (*curr + n) % 100,
        Direction::Left  => (*curr - n + 10000) % 100, 
    };

    tick_count as u32
}

fn part_1(input_path: &str) -> u32 {
    let mut counter: u32 = 0;

    let contents = std::fs::read_to_string(input_path).expect("Unable to open file");
    let instructions = contents.split("\n");
    let mut curr: i32 = 50;

    for i in instructions {
        if i.is_empty() {
            // last line is empty, just break
            break
        }

        let instruction = i.to_string();
        let direction = instruction.chars().nth(0).unwrap();
        let units: i32 = instruction[1..].parse::<i32>().unwrap();

        let direction = match direction {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        };

        rotate(&mut curr, units, direction); 

        if curr == 0 {
            counter += 1;
        }
    }

    counter
}

fn part_2(input_path: &str) -> u32 {
    let mut counter: u32 = 0;

    let contents = std::fs::read_to_string(input_path).expect("Unable to open file");
    let instructions = contents.split("\n");
    let mut curr: i32 = 50;

    for i in instructions {
        if i.is_empty() {
            // last line is empty, just break
            break
        }

        let instruction = i.to_string();
        let direction = instruction.chars().nth(0).unwrap();
        let units: i32 = instruction[1..].parse::<i32>().unwrap();

        let direction = match direction {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        };

        counter += count_ticked(&mut curr, units, direction);
    }

    counter
}

pub fn solve(input_path: &str) {
    println!("== DAY 1 ==");
    println!("Part 1: {}", part_1(input_path));
    println!("Part 2: {}", part_2(input_path));
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn part_1_example() {
        let ans = part_1("input/day_one_example.txt");
        assert_eq!(ans, 3);
    }

    #[test]
    fn part_2_example() {
        let ans = part_2("input/day_one_example.txt");
        assert_eq!(ans, 6);
    }
}