fn main() {
    let input = std::fs::read_to_string("day03/data/input.txt").expect("failed to read file");
    let mut numbers: Vec<GridNumber> = vec![];
    let mut row = 0;
    let mut column: i32;
    let mut number_buf = String::new();

    for line in input.lines() {
        let mut column = 0;
        for char in line.chars() {
            if char.is_digit(10) {
                number_buf.push(char);
            } else if !number_buf.is_empty() {
                let number = number_buf.parse::<u32>().expect("failed to parse number");
                let length = number_buf.len() as u32;
                numbers.push(GridNumber { number: number, l: row, x: column-length, y: column-1, marked: false});
                number_buf.clear();
            }

            column += 1;
        }

        if !number_buf.is_empty() {
            let number = number_buf.parse::<u32>().expect("failed to parse number");
            let length = number_buf.len() as u32;
            numbers.push(GridNumber { number: number, l: row, x: column-length, y: column-1, marked: false});
            number_buf.clear();
        }

        row += 1;
    }

    let mut gear_ratio_sum = 0;
    let mut index = 0;
    row = 0;
    for line in input.lines() {
        column = 0;
        for char in line.chars() {
            if !char.is_digit(10) && (char != '.') {
                let mut match_count = 0;
                let mut gear_ratio = 1;

                for i in index..numbers.len() {
                    let num = numbers[i];
                    if num.l > row + 1 {
                        continue;
                    }
                    if num.l < row - 1 {
                        index = i + 1;
                        continue;
                    }

                    if row - 1 <= num.l && num.l <= row + 1 {
                        for offset in -1..=1 {
                            let test = ((column as i32) + offset) as u32;
                            if num.x <= test && test <= num.y {
                                numbers[i].marked = true;
                                match_count += 1;
                                gear_ratio *= num.number;
                                break;
                            }
                        }
                    }
                }

                if char == '*' && match_count == 2 {
                    gear_ratio_sum += gear_ratio;
                }
            }

            column += 1;
        }

        row += 1;
    }

    let sum: u32 = numbers.iter().filter(|n| n.marked).fold(0, |acc, n| acc + n.number);

    println!("sum : {sum}");
    println!("gear_ratio_sum : {gear_ratio_sum}");

}

#[derive(Clone, Copy, Debug)]
struct GridNumber {
    number: u32, // value of the parsed string
    l: u32,      // height/line in the grid
    x: u32,      // leftmost position
    y: u32,      // rightmost position
    marked: bool // marked or not
}