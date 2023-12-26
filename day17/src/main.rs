use std::{vec, collections::BinaryHeap, cmp::Ordering};

fn main() {
    let input = std::fs::read_to_string("day17/data/example.txt").expect("failed to read file");
    let map = Map::parse(&input);
    map.print();

    let path = Path {
        points: vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
        ]
    };
    let path_heat_loss = map.path_heat_loss(&path);
    println!("path_heat_loss: {}", path_heat_loss);

    // TODO: Think of the Ansatz:
    // 1. If the path restriction would not be there, this would just be Djikstra
    // 2. Can path restriction also be incorporated into Djikstra?
    //    I am not 100% sure but the following might work (just test it!):
    //    While doing normal Djikstra keep track of the amount of times that we move in a straight line consecutively. (left/right: set 0, straight: increment)
    //    If counter >= 3: do not consider the straight edge.

}

struct Tile {
    position: Point,
    heat_loss: i64,
    marked: bool,
    distance: i64,
    parent: Option<Point>
}

impl Tile {
    fn new(x: i64, y: i64, heat_loss: i64) -> Tile {
        Tile { position: Point { x, y }, heat_loss, marked: false, distance: i64::MAX, parent: None }
    }
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> Ordering {
        // this generates the reverse ordering, so we can use the simple MaxHeap as a MinHeap!
        self.heat_loss.cmp(&other.heat_loss).reverse()
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        (self.position.x, self.position.y) == (other.position.x, other.position.y)
    }
}

impl Eq for Tile {}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn neighbourhood(&self) -> Vec<Point> {
        vec![
            Point { x: self.x-1, y: self.y   },
            Point { x: self.x+1, y: self.y   },
            Point { x: self.x,   y: self.y-1 },
            Point { x: self.x,   y: self.y+1 },
        ]
    }
}

struct Path {
    points: Vec<Point>
}

struct Map {
    w: i64,
    h: i64,
    tiles: Vec<Tile>
}

impl Map {
    fn parse(s: &str) -> Map {
        let mut map = Map { w: 0, h: 0, tiles: vec![] };

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let heat_loss = c as i64 - '0' as i64;
                let tile = Tile::new(x as i64, y as i64, heat_loss);
                map.tiles.push(tile);
            }
            map.h += 1;
        }
        map.w = map.tiles.len() as i64 / map.h;

        map
    }

    fn print(&self) {
        for y in 0..self.h {
            for x in 0..self.w {
                print!("{}", self.get(x, y).heat_loss);
            }
            println!();
        }
    }

    fn get(&self, x: i64, y: i64) -> &Tile {
        let index = y * self.w + x;
        &self.tiles[index as usize]
    }

    fn get_mut(&mut self, x: i64, y: i64) -> &mut Tile {
        let index = y * self.w + x;
        &mut self.tiles[index as usize]
    }

    fn path_heat_loss(&self, path: &Path) -> i64 {
        let mut sum = 0;

        for point in path.points.iter().skip(1) {
            let tile = self.get(point.x, point.y);
            sum += tile.heat_loss;
        }

        sum
    }

    fn find_optimal_path(&mut self, s: Point, t: Point) -> Path {
        let mut path = Path { points: vec![] };
        let mut boundary = BinaryHeap::new();
        self.get_mut(0, 0).marked = true;
        self.get_mut(1, 0).parent = Some(Point { x: 0, y: 0 });
        self.get_mut(0, 1).parent = Some(Point { x: 0, y: 0 });
        boundary.push(self.get(1, 0));
        boundary.push(self.get(0, 1));
        boundary.peek_mut()

        // Do a 3-Partition of the set of nodes:
        // 1. Known: We know the shortest path to these nodes.
        // 2. Boundary: Candidate nodes for further exploration of the unknown.
        // 3. Unknown: We have not seen these nodes yet.
        let mut consecutive_counter = 0;
        loop {
            // extend shortest path tree with one new node
            let next = boundary.pop().unwrap();
            let p = next.position;
            self.get_mut(p.x, p.y).marked = true;
            
            let neighbourhood = p.neighbourhood();
            for neighbour in neighbourhood {
                if !(0..self.w).contains(&neighbour.x) || !(0..self.h).contains(&neighbour.y) {
                    continue;
                }

                let tile = self.get(neighbour.x, neighbour.y);
                let distance = next.distance + tile.heat_loss;
                tile.distance = tile.distance.min(next.distance + tile.heat_loss);
                
                if !tile.marked {
                    tile.marked = true;
                    boundary.push(tile);
                }
            }

            // TODO: later also consider the consecutive counter. Right now ignore it.



            // TODO: search until t is found
            // TODO: impl modified dijkstra
            panic!("dijkstra not implemented");
        }

        path
    }

    fn find_optimal_heat_loss(&mut self) -> i64 {
        let w  = self.w;
        let h = self.h;
        let s = Point { x: 0, y: 0 };
        let t = Point { x: w-1, y: h-1 };
        let path = self.find_optimal_path(s, t);
        self.path_heat_loss(&path)
    }
}
