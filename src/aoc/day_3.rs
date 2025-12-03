use std::fs;

fn part_1(input_path: &str) -> u32 {
    let mut total = 0;

    let contents = match fs::read_to_string(input_path) {
        Ok(contents) => contents,
        Err(e) => panic!("Unable to read file: {}", e),
    };

    let rows = contents.split("\n");

    for row in rows {
        if row.is_empty() {
            break
        }

        let mut left = 0;
    
        
        let row = row.replace("\n", "");
        let row: Vec<u32> = row.chars().map(|n| n.to_digit(10).unwrap()).collect();

        let (max_idx, _) = row.iter().enumerate().max_by_key(|&(_, val)| val).unwrap();

        let mut right = max_idx;

        let mut prev_max = 0;

        while left < right {
            if row[left] * 10 + row[right] > prev_max {
                prev_max = row[left] * 10 + row[right];
            }
            left += 1;
        }

        while right < row.len() - 1 {
            right += 1;

            if row[left] * 10 + row[right] > prev_max {
                prev_max = row[left] * 10 + row[right];
            }
        }
        

        total += prev_max;
    }

    

    total
}

fn part_2(input_path: &str) -> i64 {

    let mut total = 0;

    let contents = match fs::read_to_string(input_path) {
        Ok(contents) => contents,
        Err(e) => panic!("Unable to read file: {}", e),
    };

    let rows = contents.split("\n");

    for row in rows {
        if row.is_empty() {
            break
        }

        let row = row.replace("\n", "");
        let mut value: Vec<u32> = row.chars().map(|n| n.to_digit(10).unwrap()).collect();


        let mut i = 0;
        
        loop {
            // do comparison, if smaller pop
            // do until length 12 or index at end
            if i == value.len() - 1  || value.len() == 12 {
                break
            }

            if value[i] < value[i+1] {
                value.remove(i);
                i = 0; // inefficient but eh
                continue
            };

            i += 1;

        };

        // make sure to remove potential hanging nums after 12th position
        let value: String = value.split_at(12).0.into_iter().map(|x| x.to_string()).collect(); 
        let value = value.parse::<i64>().unwrap();
    
        total += value;
    }

    total
}

pub fn solve(input_path: &str) {
    println!("== DAY 3 ==");
    println!("Part 1: {}", part_1(input_path));
    println!("Part 2: {}", part_2(input_path));
    println!("");
}

mod test {
    use super::*;

    #[test]
    fn part_1_example() {
        let ans = part_1("input/day_3_example.txt");
        assert_eq!(ans, 357);
    }

    #[test]
    fn part_2_example() {
        let ans = part_2("input/day_3_example.txt");
        assert_eq!(ans, 3121910778619);
    }
}