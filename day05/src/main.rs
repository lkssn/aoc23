use std::vec;

fn main() {
    let input = std::fs::read_to_string("day05/data/input.txt").expect("failed to read file");
    let mut lines = input.lines();
    let start_values = lines.next().unwrap().split(':').skip(1).next().unwrap().split_ascii_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut almanac = Almanac::new(start_values);

    for line in lines {
        if line.is_empty() {
            continue;
        }

        if line.contains(':') {
            almanac.maps.push(Map::new());
            continue;
        }

        let nums = line.split_ascii_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let range = Range{
            dest_range_start: nums[0],
            source_range_start: nums[1],
            range_length: nums[2],
        };
        almanac.maps.last_mut().unwrap().ranges.push(range);
    }

    let mut dest_min = almanac.source_to_dest(almanac.start_values[0]);

    for source in &almanac.start_values {
        let dest = almanac.source_to_dest(*source);
        if dest < dest_min {
            dest_min = dest;
        }
    }

    println!("Part One: {dest_min}");

    let mut outputs = vec![];
    for input in &almanac.start_ranges {
        outputs.append(&mut almanac.map(input));
    }

    outputs.sort_by(|a,b| a.start.cmp(&b.start));
    let dest_min_ranges = outputs[0].start;
    println!("Part Two: {dest_min_ranges}");
}

#[derive(Debug)]
struct Almanac {
    start_values: Vec<i64>,
    start_ranges: Vec<SimpleRange>,
    maps: Vec<Map>
}

impl Almanac {
    fn new(start_values: Vec<i64>) -> Almanac {
        let mut almanac = Almanac {
            start_values,
            start_ranges: vec![],
            maps: vec![]
        };

        let mut i = 0;
        while i < almanac.start_values.len() {
            almanac.start_ranges.push(SimpleRange { 
                start: almanac.start_values[i],
                length: almanac.start_values[i+1]
            });
            i += 2;
        }

        return almanac;
    }

    fn source_to_dest(&self, mut source: i64) -> i64 {
        for map in &self.maps {
            source = map.source_to_dest(source);
        }
        source
    }

    fn map(&self, range: &SimpleRange) -> Vec<SimpleRange> {
        let mut inputs = vec![*range];
        let mut outputs = vec![];

        for map in &self.maps {
            inputs.append(&mut outputs);
            for input in &inputs {
                outputs.append(&mut map.map(&input));
            }
            inputs.clear();
        }

        outputs
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>
}

impl Map {
    fn new() -> Map {
        Map {
            ranges: vec![]
        }
    }
}

impl Map {
    // Map a range of sources to multiple destination ranges.
    fn map(&self, sources: &SimpleRange) -> Vec<SimpleRange> {
        let mut intersections = Vec::<SimpleRange>::new();
        let mut destinations = Vec::<SimpleRange>::new();

        for range in &self.ranges {
            if let Some(intersection) = range.intersection(sources) {
                let translated = range.translate(&intersection);
                intersections.push(intersection);
                destinations.push(translated);
            }
        }

        let mut unmatcheds = sources.minus_disjoint_subsets(intersections);
        destinations.append(&mut unmatcheds);
        destinations
    }

    fn source_to_dest(&self, source: i64) -> i64 {
        for range in &self.ranges {
            if let Some(dest) = range.source_to_dest(source) {
                return dest;
            }
        }
        return source;
    }
}

#[derive(Debug)]
struct Range {
    source_range_start: i64,
    dest_range_start: i64,
    range_length: i64
}

impl Range {
    // Calculate intersection with the matching range.
    fn intersection(&self, sources: &SimpleRange) -> Option<SimpleRange> {
        let a = sources.start;
        let b = sources.start + sources.length - 1;
        let c = self.source_range_start;
        let d = self.source_range_start + self.range_length - 1;

        let x = a.max(c);
        let y = b.min(d);
        let l = y - x + 1;

        if l <= 0 {
            return None;
        }

        let intersect = SimpleRange {
            start: x,
            length: l
        };
        Some(intersect)
    }

    // Translate a range of sources, as if every source would be a match.
    fn translate(&self, sources: &SimpleRange) -> SimpleRange {
        SimpleRange {
            start: sources.start + self.dest_range_start - self.source_range_start,
            length: sources.length
        }
    }

    // Map a single point
    fn source_to_dest(&self, source: i64) -> Option<i64> {
        if self.source_range_start <= source && source < self.source_range_start + self.range_length - 1 {
            return Some(source - self.source_range_start + self.dest_range_start);
        }

        return None;
    }
}

#[derive(Debug, Clone, Copy)]
struct SimpleRange {
    start: i64,
    length: i64
}

impl SimpleRange {
    // idea: collect every endpoint and take the inverted ranges
    fn minus_disjoint_subsets(&self, mut ranges: Vec<SimpleRange>) -> Vec<SimpleRange> {
        let mut points = vec![];
        let mut results = vec![];
        ranges.sort_by(|a,b| a.start.cmp(&b.start));

        points.push(self.start);
        for range in &ranges {
            points.push(range.start - 1);
            points.push(range.start + range.length);
        }
        points.push(self.start + self.length - 1);

        let mut i = 0;
        while i < points.len() {
            let l = points[i+1] - points[i] + 1;
            if l >= 1 {
                let result = SimpleRange{
                    start: points[i],
                    length: l
                };
                results.push(result);
            }

            i += 2;
        }

        results
    }
}
