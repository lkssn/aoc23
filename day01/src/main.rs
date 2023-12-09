use std::collections::HashMap;

fn filter_digits(s: &str) -> Vec<u32> {
    s.chars().filter(|c| c.is_digit(10)).map(|c| c.to_digit(10).unwrap()).collect()
}

fn calc_calibration_line(line: &str) -> Option<u32> {
    let digits = filter_digits(line);
    if digits.len() == 0 {
        return None;
    }

    let first = digits.first().unwrap();
    let last = digits.last().unwrap();
    Some(first * 10 + last)
}

fn calc_calibration_document(s: &str) -> u32 {
    s.lines().map(|line| calc_calibration_line(line).unwrap_or(0)).sum()
}

fn calc_calibration_line_v2(line: &str) -> Option<u32> {
    let lookup  = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),

        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut first_index = None;
    let mut first_value = None;
    let mut last_index = None;
    let mut last_value = None;

    for pattern in lookup.keys() {
        if let Some(index) = line.find(pattern) {
            match first_index {
                None => {
                    first_index = Some(index);
                    first_value = Some(*lookup.get(pattern).unwrap());
                },
                Some(ind) => {
                    if index < ind {
                        first_index = Some(index);
                        first_value = Some(*lookup.get(pattern).unwrap());
                    }
                }
            }
        }

        if let Some(index) = line.rfind(pattern) {
            match last_index {
                None => {
                    last_index = Some(index);
                    last_value = Some(*lookup.get(pattern).unwrap());
                },
                Some(ind) => {
                    if index > ind {
                        last_index = Some(index);
                        last_value = Some(*lookup.get(pattern).unwrap());
                    }
                }
            }
        }
    }

    if first_value.is_none() || last_value.is_none() {
        return None;
    }

    let first = first_value.unwrap();
    let last = last_value.unwrap();
    Some(first * 10 + last)
}

fn calc_calibration_document_v2(s: &str) -> u32 {
    s.lines().map(|line| calc_calibration_line_v2(line).unwrap_or(0)).sum()
}

fn main() {
    let input_v1 = std::fs::read_to_string("day01/data/input.txt");
    if input_v1.is_err() {
        println!("failed to read input_v1");
        return;
    }
    let input_v1 = input_v1.unwrap();

    let result_v1 = calc_calibration_document(input_v1.as_str());
    println!("result_v1: {result_v1}");

    let input_v2 = std::fs::read_to_string("day01/data/input.txt");
    if input_v2.is_err() {
        println!("failed to read input_v2");
        return;
    }
    let input_v2 = input_v2.unwrap();

    let result_v2 = calc_calibration_document_v2(input_v2.as_str());
    println!("result_v2: {result_v2}");
}
