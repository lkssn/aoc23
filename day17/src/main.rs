use std::iter::zip;

fn main() {
    let input = std::fs::read_to_string("day17/data/example.txt").expect("failed to read file");
    let heat_matrix = HeatMatrix::parse(&input);
    let s = Point::new(0, 0);
    let t: Point = Point::new(heat_matrix.w-1, heat_matrix.h-1);

    let dijkstra_tree = heat_matrix.dijkstra(s, t);
    let dijkstra_distance = dijkstra_tree.get(t).distance;

    let optimal_feasible_path = heat_matrix.find_optimal_feasible_path();
    let optimal_feasible_distance = heat_matrix.path_distance(&optimal_feasible_path);

    dijkstra_tree.print();
    println!("dijkstra_distance: {}", dijkstra_distance);

    heat_matrix.print_path(optimal_feasible_path);
    println!("optimal_feasible_distance: {}", optimal_feasible_distance);
}

#[derive(Debug, Clone, Copy)]
struct DijkstraData {
    marked: bool,
    distance: i64,
    parent: Option<Point>
}

impl Default for DijkstraData {
    fn default() -> DijkstraData {
        DijkstraData {
            marked: false,
            distance: i64::MAX,
            parent: None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn print(&self) {
        let c = match self {
            Self::North => '^',
            Self::East =>  '>',
            Self::South => 'v',
            Self::West =>  '<',
        };
        print!("{}", c);
    }
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    fn neighbourhood(&self) -> Vec<Point> {
        vec![
            Point { x: self.x,   y: self.y-1 },
            Point { x: self.x+1, y: self.y   },
            Point { x: self.x,   y: self.y+1 },
            Point { x: self.x-1, y: self.y   },
        ]
    }

    fn delta(&self, next: &Self) -> Point {
        Point::new(next.x - self.x, next.y - self.y) 
    }

    fn get_direction(&self, next: &Self) -> Direction {
        let d = self.x.abs_diff(next.x) + self.y.abs_diff(next.y);
        if d != 1 {
            panic!("get_direction: not neighbours {:?}, {:?}", self, next);
        }

        if self.y > next.y {
            return Direction::North;
        }

        if self.y < next.y {
            return Direction::South;
        }

        if self.x < next.x {
            return Direction::East;
        }

        return Direction::West;
    }
}

struct Matrix<T: Clone + Default> {
    w: i64,
    h: i64,
    values: Vec<T>
}

type HeatMatrix = Matrix<i64>;
type DijkstraMatrix = Matrix<DijkstraData>;

impl<T: Clone + Default> Matrix<T> {
    fn new() -> Matrix<T> {
        Matrix::<T> {
            w: 0,
            h: 0,
            values: vec![]
        }
    }

    fn new_sized(w: i64, h: i64) -> Matrix<T> {
        Matrix::<T> {
            w,
            h,
            values: vec![T::default(); (w * h) as usize]
        }
    }

    fn get(&self, p: Point) -> &T {
        let index = p.y * self.w + p.x;
        &self.values[index as usize]
    }

    fn get_mut(&mut self, p: Point) -> &mut T {
        let index = p.y * self.w + p.x;
        &mut self.values[index as usize]
    }
}

impl HeatMatrix {
    fn parse(s: &str) -> HeatMatrix {
        let mut matrix = HeatMatrix::new();

        for line in s.lines() {
            for c in line.chars() {
                let heat_loss = c as i64 - '0' as i64;
                matrix.values.push(heat_loss);
            }
            matrix.h += 1;
        }
        matrix.w = matrix.values.len() as i64 / matrix.h;

        matrix
    }

    fn print_path(&self, mut path: Vec<Point>) {
        path.sort();

        for y in 0..self.h {
            for x in 0..self.w {
                let p = Point::new(x, y);

                if let Ok(index) = path.binary_search(&p) {
                    let next_index = index + 1;
                    if next_index < path.len() {
                        let next = path[next_index];
                        let dir = p.get_direction(&next);
                        dir.print();
                        continue;
                    }
                }

                print!(".");
            }
            println!();
        }
    }

    // Compute on the HeatMatrix graph G the partial shortest-path-tree
    // with source s and target t using Dijkstra's algorithm.
    fn dijkstra(&self, s: Point, t: Point) -> DijkstraMatrix {
        self.dijkstra_mod(vec![], s, t)
    }

    // Compute on the HeatMatrix graph G the partial shortest-path-tree
    // with source s and target t using Dijkstra's algorithm
    // but instead use the Graph G' = G - p + {s, t}.
    fn dijkstra_mod(&self, p: Vec<Point>, s: Point, t: Point) -> DijkstraMatrix {
        let mut tree = DijkstraMatrix::new_sized(self.w, self.h);
        let mut boundary = vec![];
        tree.get_mut(s).distance = 0;
        tree.get_mut(s).marked = true;
        boundary.push(s);

        // Solve the single source shortest path problem with Dijkstra's Algorithm.
        // Do a 3-Partition of the set of nodes and iterate:
        // 1. Known: We know the shortest path to these nodes. -> Known = {}
        // 2. Boundary: Candidate nodes for further exploration of the unknown. -> Boundary = {s}
        // 3. Unknown: We have not seen these nodes yet. -> Unknown = V

        while !boundary.is_empty() {
            // extend shortest path tree with one new node
            let next = tree.remove_minimal_point(&mut boundary);
            if next == t {
                // just compute the partial tree until we find t
                break;
            }
            let next_d = tree.get(next).distance;

            // update the boundary
            for neighbour in next.neighbourhood() {
                // check rectangle-bounds for potential neighbours
                if !(0..self.w).contains(&neighbour.x) || !(0..self.h).contains(&neighbour.y) {
                    continue;
                }

                // skip p but not {s,t}
                if neighbour != s && neighbour != t {
                    if p.contains(&neighbour) {
                        continue;
                    }
                }

                let data = tree.get_mut(neighbour);
                let new_distance = next_d + self.get(neighbour);

                if new_distance < data.distance {
                    data.parent = Some(next);
                    data.distance = new_distance;
                }

                if !data.marked {
                    data.marked = true;
                    boundary.push(neighbour);
                }
            }
        }

        tree
    }

    // Find any feasible path from the upper-left corner to the lower-right corner.
    fn find_any_feasible_path(&self) -> Vec<Point> {
        let s = Point::new(0, 0);
        let t = Point::new(self.w-1, self.h-1);
        let mut path = vec![];
        let mut vertical = true;
        let mut current = s;

        // zig zag along the diagonal
        // turns at every node -> path is feasible
        // diagonal path always works -> is a path from s to t
        while current != t {
            path.push(current);
            if vertical {
                current.y += 1;
            } else {
                current.x += 1;
            }
            vertical = !vertical;
        }
        path.push(t);

        path
    }

    // Find a set of paths that could be an improvement.
    fn find_augmented_paths(&self, path: &Vec<Point>) -> Vec<Vec<Point>> {
        let l = path.len();
        let mut augmentations = vec![];

        let mut candidate_nodes = path.clone();
        candidate_nodes.remove(0); // remove s
        candidate_nodes.pop(); // remove t

        // candidate_nodes = p - {s,t}
        for index in 1..=(l-2) {
            let s = path[index-1];
            let t = path[index+1];
            let tree = self.dijkstra_mod(path.clone(), s, t);
            let epsilon = tree.find_path(s, t);

            // check if t is reachable from s
            if let Some(mut epsilon) = epsilon {
                let mut aug = vec![];
                aug.append(&mut Vec::from(&path[..(index-1)]));
                aug.append(&mut epsilon);
                aug.append(&mut Vec::from(&path[(index+2)..]));
                augmentations.push(aug);
            }
        }

        augmentations
    }

    // Check if the path is feasible:
    // Delta of consecutive points can be the same max 4 times.
    fn is_feasible_path(&self, path: &Vec<Point>) -> bool {
        let mut delta = path[0].delta(&path[1]);
        let mut counter = 1;

        for (current, next) in zip(path.iter().skip(1), path.iter().skip(2)) {
            let new_delta = current.delta(&next);

            if delta == new_delta {
                counter += 1;
                if counter >= 5 {
                    return false;
                }
            } else {
                delta = new_delta;
                counter = 1;
            }
        }

        true
    }

    // Calculate the path distance as the sum of heat losses on the edges(!).
    // This always skips the heat loss on the first node.
    fn path_distance(&self, path: &Vec<Point>) -> i64 {
        path.iter().skip(1).fold(0 as i64, |acc, x| acc + self.get(*x))
    }

    // Find among all feasible paths from the upper-left corner to the lower-right corner
    // the shortest path.
    fn find_optimal_feasible_path(&self) -> Vec<Point> {
        // Optimization Procedure:
        // 1. Find any feasible path.
        // 2. Iteratively improve the path.
        // 3. Loop until no improvement found.
        let mut path = self.find_any_feasible_path();

        loop {
            // generate a set of possible improved paths
            let augmented_paths = self.find_augmented_paths(&path);

            // find the optimal feasible path among the generated paths and the current path
            let mut min_path = path.clone();
            let mut min_distance = self.path_distance(&path);

            for aug in augmented_paths {
                if !self.is_feasible_path(&aug) {
                    continue;
                }

                let distance = self.path_distance(&aug);
                if distance < min_distance {
                    min_path = aug.clone();
                    min_distance = distance;
                }
            }

            // check if we are done
            if min_path == path {
                break
            }

            path = min_path;
            println!("find_optimal_feasible_path: distance = {}, path.len() = {:?}", min_distance, path.len());
        }

        path
    }
}

impl DijkstraMatrix {
    fn remove_minimal_point(&self, v: &mut Vec<Point>) -> Point {
        if v.is_empty() {
            panic!("remove_minimal_point: empty");
        }

        let mut min_index = 0;

        for (i, p) in v.iter().enumerate() {
            let d = self.get(*p).distance;
            let min_d = self.get(v[min_index]).distance;

            if d < min_d {
                min_index = i;
            }
        }

        v.remove(min_index)
    }

    fn print(&self) {
        for y in 0..self.h {
            for x in 0..self.w {
                let p = Point::new(x, y);
                let data = self.get(p);

                if let Some(parent) = data.parent {
                    let direction = p.get_direction(&parent);
                    direction.print();
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn find_path(&self, s: Point, t: Point) -> Option<Vec<Point>> {
        let mut path = vec![];
        let mut iter = t;

        loop {
            path.push(iter);

            if let Some(parent) = self.get(iter).parent {
                iter = parent;
            } else {
                break;
            }
        }

        if iter != s {
            return None;
        }

        path.reverse();
        Some(path)
    }
}
