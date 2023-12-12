fn main() {
    let input = std::fs::read_to_string("day11/data/input.txt").expect("failed to read file");
    let image = Image::parse(&input);
    image.print();

    let sum_two = image.apsp_length_sum(2);
    println!("sum_two: {sum_two}");

    let sum_million = image.apsp_length_sum(1000000);
    println!("sum_million: {sum_million}");
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Data {
    Empty,
    Galaxy
}

impl Data {
    fn parse(c: char) -> Data {
        match c {
            '.' => Data::Empty,
            '#' => Data::Galaxy,
            _ => panic!("failed to parse Data")
        }
    }

    fn char(&self) -> char {
        match self {
            Data::Empty  => '.',
            Data::Galaxy => '#'
        }
    }
}

#[derive(Debug)]
struct Image {
    data: Vec<Data>,
    width: i64,
    height: i64
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn manhattan_distance(self, other: Point) -> i64 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }
}

impl Image {
    fn parse(s: &str) -> Image {
        let mut data = vec![];
        let mut height = 0;

        for line in s.lines() {
            height += 1;
            for c in line.chars() {
                data.push(Data::parse(c));
            }
        }

        let width = data.len() as i64 / height;
        Image{data, width, height}
    }

    fn get_data(&self, point: Point) -> Data {
        let index = (point.y * self.width + point.x) as usize;
        self.data[index]
    }

    fn _set_data(&mut self, point: Point, data: Data) {
        let index = (point.y * self.width + point.x) as usize;
        self.data[index] = data;
    }

    fn print(&self) {
        for y in 0..self.height {
            for x  in 0..self.width {
                let point = Point{x, y};
                let data = self.get_data(point);
                let c = data.char();
                print!("{c}");
            }
            println!();
        }
    }

    fn apsp_length_sum(&self, expansion: i64) -> i64 {
        let mut sum = 0;
        let mut galaxies = vec![];
        let mut empty_rows = vec![];
        let mut empty_columns = vec![];

        for y in 0..self.height {
            let is_empty_row = (0..self.width).all(|x| self.get_data(Point{x,y}) == Data::Empty);
            if is_empty_row {
                empty_rows.push(y);
            }
        }

        for x in 0..self.width {
            let is_empty_column = (0..self.height).all(|y| self.get_data(Point{x,y}) == Data::Empty);
            if is_empty_column {
                empty_columns.push(x);
            }
        }

        for y in 0..self.height {
            for x  in 0..self.width {
                let mut point = Point{x, y};
                let data = self.get_data(point);
                if data == Data::Galaxy {
                    let dx = empty_columns.iter().fold(0, |acc, column| acc + if x > *column {1} else {0});
                    let dy = empty_rows.iter().fold(0, |acc, row| acc + if y > *row {1} else {0});
                    point.x += dx * (expansion - 1);
                    point.y += dy * (expansion - 1);
                    galaxies.push(point);
                }
            }
        }

        let n = galaxies.len();
        for i in 0..n {
            for j in (i+1)..n {
                let a = galaxies[i];
                let b = galaxies[j];
                let distance = a.manhattan_distance(b);
                sum += distance;
            }
        }

        sum
    }
}
