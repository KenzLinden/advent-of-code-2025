use std::{collections::{HashMap}, fs};


fn is_two_repeating_pattern(s: String) -> bool {
    let chars: Vec<char> = s.chars().collect();

    let mut left = 0;
    let mut right = s.len()/2;

    while left <= chars.len()/2 {
        if right >= chars.len() {
            return true;
        }

        if chars[left] != chars[right] {
            return false;
        }

        right += 1;
        left += 1;
    }

    false
}

fn is_entirely_repeating_pattern(s: String) -> bool {
    let len = s.len();
    
    for i in 1..=len / 2 {
        if len % i == 0 {
            let pattern = &s[0..i]; 

            let mut valid = true;
            for j in (i..len).step_by(i) {
                if &s[j..j + i] != pattern {
                    valid = false;
                    break;
                }
            }

            if valid {
                return true; 
            }
        }
    }

    false
}



fn part_1(input_path: &str) -> i64 {
    let mut total: i64 = 0;
    let contents = fs::read_to_string(input_path).unwrap().replace("\n",  "");
    let ranges = contents.split(",");

    for range in ranges {
        let (min, max) = range.split_once("-").unwrap(); 
        let min: i64 = match min.parse::<i64>() {
            Ok(num) => num,
            Err(e) => panic!("Error parsing '{}': {}", min, e)
        };
        let max: i64 = match max.parse::<i64>() {
            Ok(num) => num,
            Err(e) => panic!("Error parsing '{}': {}", max, e)
        };

        for i in min..=max {
            let i_str = i.to_string();
            let pattern_repeats = is_two_repeating_pattern(i_str);
            if pattern_repeats {
                total += i as i64; 
            }
        }
    }

    total
}

fn part_2(input_path: &str) -> i64 {
    let mut total: i64 = 0;
    let contents = fs::read_to_string(input_path).unwrap().replace("\n", "");
    let ranges = contents.split(",");

    for range in ranges {
        let (min, max) = range.split_once("-").unwrap();

        let min: i64 = match min.parse::<i64>() {
            Ok(num) => num,
            Err(e) => panic!("Error parsing '{}': {}", min, e)
        };
        let max: i64 = match max.parse::<i64>() {
            Ok(num) => num,
            Err(e) => panic!("Error parsing '{}': {}", max, e)
        };

        for i in min..=max {
            let i_str = i.to_string();
            let pattern_repeats = is_entirely_repeating_pattern(i_str);
            if pattern_repeats {
                total += i as i64; 
            }
        }
    };

    total
}



pub fn solve (input_path: &str) {
    println!("== DAY 2 ==");
    println!("Part 1: {}", part_1(input_path));
    println!("Part 2: {}", part_2(input_path));
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn part_1_example() {
        let ans = part_1("input/day_two_example.txt");
        assert_eq!(ans, 1227775554);
    }

    #[test]
    fn part_2_example() {
        let ans = part_2("input/day_two_example.txt");
        assert_eq!(ans, 4174379265);
    }
}