fn main() {
    let input = std::fs::read_to_string("day17/data/example.txt").expect("failed to read file");
    let heat_matrix = HeatMatrix::parse(&input);

    let path = heat_matrix.find_path().expect("unreachable");
    let distance = heat_matrix.calc_path_weight(&path);

    heat_matrix.print_path(&path);
    println!("distance: {distance}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum DijkstraState {
    Known,
    Boundary,
    Unknown
}

#[derive(Debug, Clone)]
struct DijkstraData {
    state: DijkstraState,
    distance: i64,
    prev_path: Option<Vec<Point>>
}

impl Default for DijkstraData {
    fn default() -> DijkstraData {
        DijkstraData {
            state: DijkstraState::Unknown,
            distance: i64::MAX,
            prev_path: None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    fn turn_clock_wise(&self) -> Point {
        let p = self;
        let q = Point::new(-p.y, p.x);
        q
    }

    fn reversed(&self) -> Point {
        let p = self;
        let q = Point::new(-p.x, -p.y);
        q
    }
}

#[derive(PartialEq, Clone, Debug)]
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

    fn print_path(&self, path: &Vec<Point>) {
        let mut path = path.clone();
        path.sort();

        for y in 0..self.h {
            for x in 0..self.w {
                let p = Point::new(x, y);

                if path.binary_search(&p).is_ok() {
                    print!("x");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    // Path is in the bounds.
    fn check_path_valid(&self, path: &Vec<Point>) -> bool {
        path.iter().all(|p| self.check_point_valid(p))
    }

    // Point is in the bounds.
    fn check_point_valid(&self, p: &Point) ->  bool {
        (0..self.w).contains(&p.x) && (0..self.h).contains(&p.y)
    }

    // Generate all the next valid straight paths:
    // 1. Do not go into the direction of prev_path or the reverse direction of it.
    // 2. For every valid direction go 1,2 or 3 steps into that direction.
    fn generate_next_paths(&self, p: Point, prev_path: &Option<Vec<Point>>) -> Vec<Vec<Point>> {
        let mut paths = vec![];
        if let Some(prev_path) = prev_path {
            let prev = prev_path[1];
            let mut d = p;
            d.x -= prev.x;
            d.y -= prev.y;

            for ortho in [d.turn_clock_wise(), d.turn_clock_wise().reversed()] {
                paths.append(&mut self.generate_short_paths(p, ortho));
            }
        } else {
            for d in [
                Point::new(1, 0),
                Point::new(-1, 0),
                Point::new(0, 1),
                Point::new(0, -1)
            ] {
                paths.append(&mut self.generate_short_paths(p, d));
            }
        }

        paths
    }

    // Generate all valid paths from p in the direction
    // of the normalized vector d (east/west/north/south),
    // with 1 to 3 steps.
    fn generate_short_paths(&self, p: Point, d: Point) -> Vec<Vec<Point>> {
        let mut paths: Vec<Vec<Point>> = vec![];
        let mut path = vec![p];
        let mut q: Point = p;

        for _ in 0..3 {
            q.x += d.x;
            q.y += d.y;

            if !self.check_point_valid(&q) {
                break;
            }

            path.push(q);
            paths.push(path.clone());
        }

        paths
    }

    fn calc_path_weight(&self, path: &Vec<Point>) -> i64 {
        // saturating add: treat i64::MAX as infinity
        path.iter().skip(1).fold(0 as i64, |acc, x| acc.saturating_add(*self.get(*x)))
    }

    // Compute on the HeatMatrix graph G the shortest-path-tree
    // with source s using Dijkstra's algorithm where nodes are neighbouring,
    // if a single chess move suffices.
    fn dijkstra_chess(&self, s: Point) -> DijkstraMatrix {
        let mut tree = DijkstraMatrix::new_sized(self.w, self.h);
        let mut boundary = vec![];
        tree.get_mut(s).distance = 0;
        tree.get_mut(s).state = DijkstraState::Boundary;
        boundary.push(s);

        // Solve the single source shortest path problem with Dijkstra's Algorithm.
        // Do a 3-Partition of the set of nodes and iterate:
        // 1. Known: We know the shortest path to these nodes. -> Known = {}
        // 2. Boundary: Candidate nodes for further exploration of the unknown. -> Boundary = {s}
        // 3. Unknown: We have not seen these nodes yet. -> Unknown = V \ {s}

        while !boundary.is_empty() {
            // extend shortest path tree with one new node
            let current = tree.remove_minimal_point(&mut boundary);
            tree.get_mut(current).state = DijkstraState::Known;

            // update the boundary
            for path in self.generate_next_paths(current, &tree.get(current).prev_path) {
                let neighbour = *path.last().expect("chess path is empty");
                let old_distance = tree.get(neighbour).distance;
                let new_distance = tree.get(current).distance.saturating_add(self.calc_path_weight(&path));

                println!("path: {path:?}");
                println!("relax: {current:?}->{neighbour:?}");

                if new_distance < old_distance {
                    let neighbour_data = tree.get_mut(neighbour);
                    let mut prev_path = path;
                    // Always want to store the reverse paths!
                    prev_path.reverse();
                    neighbour_data.prev_path = Some(prev_path);
                    neighbour_data.distance = new_distance;
                }

                if tree.get(neighbour).state == DijkstraState::Unknown {
                    tree.get_mut(neighbour).state = DijkstraState::Boundary;
                    boundary.push(neighbour);
                }
            }
        }

        tree
    }

    // Compute the shortest path tree.
    fn find_tree(&self, s: Point, t: Point) -> DijkstraMatrix {
        let base_tree = self.dijkstra_chess(s);
        let mut final_tree = base_tree.clone();

        // TODO: this does not work correctly right now!
        // Somehow the modification generates infeasible paths.

        // Modification to base tree:
        // Last move to t does not necessarily have to end in a turn.

        // for vertical in [true, false] {
        //     for steps in 1..=3 {
        //         let delta = if vertical {Point::new(0, -1)} else {Point::new(-1, 0)};
        //         let mut path = vec![];
        //         let mut iter = t;

        //         path.push(iter);
        //         for _ in 0..steps {
        //             iter.x += delta.x;
        //             iter.y += delta.y;
        //             path.push(iter);
        //         }

        //         // check out of bounds
        //         if !self.check_path_valid(&path) {
        //             continue;
        //         }

        //         // check no loops
        //         if let Some(prev_path) = &base_tree.get(iter).prev_path {
        //             if prev_path.contains(&path[path.len()-2]) {
        //                 continue;
        //             }
        //         }

        //         let old_distance  = final_tree.get(t).distance;
        //         let new_distance = base_tree.get(iter).distance.saturating_add(self.calc_path_weight(&path));
        //         let data = final_tree.get_mut(t);

        //         if new_distance < old_distance {
        //             data.distance = new_distance;
        //             data.prev_path = Some(path);
        //         }
        //     }
        // }

        final_tree
    }

    fn find_path(&self) -> Option<Vec<Point>> {
        let s = Point{x: 0, y: 0};
        let t = Point{x: self.w-1, y: self.h-1};
        let tree = self.find_tree(s, t);

        let mut path = vec![t];
        let mut current = t;

        while let Some(prev_path) = &tree.get(current).prev_path {
            for p in prev_path.iter().skip(1) {
                path.push(*p);
            }

            current = *path.last().expect("path empty");
        }

        if current != s {
            return None;
        }

        path.reverse();

        Some(path)
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
}
