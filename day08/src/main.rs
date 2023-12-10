fn main() {
    let input = std::fs::read_to_string("day08/data/input.txt").expect("failed to read file");
    let mut lines = input.lines();

    let mut directions = vec![];
    let mut nodes = vec![];

    let path_str = lines.next().unwrap();
    for c in path_str.chars() {
        let dir = match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("failed to parse path")
        };
        directions.push(dir);
    }
    let path_length = directions.len();

    for line in lines.skip(1) {
        let current_str = &line[0..3];
        let left_str = &line[7..10];
        let right_str = &line[12..15];
        let node = Node::parse(current_str, left_str, right_str);
        nodes.push(node);
    }
    nodes.sort_by(|x, y| x.current.cmp(&y.current));

    let first_node_id = parse_node_id("AAA");
    let last_node_id = parse_node_id("ZZZ");
    if let Ok(first_node_index) = nodes.binary_search_by(|n| n.current.cmp(&first_node_id)) {
        let mut iter_node = &nodes[first_node_index];
        let mut pos = 0;
        let mut steps = 0;
        while iter_node.current != last_node_id {
            let direction = &directions[pos];
            let next_id = match direction {
                Direction::Left => iter_node.left,
                Direction::Right => iter_node.right
            };

            iter_node = &nodes[nodes.binary_search_by(|n| n.current.cmp(&next_id)).unwrap()];
            pos = (pos + 1) % path_length;
            steps += 1;
        }
        println!("steps: {steps}");
    }

    let mut iter_nodes = vec![];
    for node in &nodes {
        if node.node_type == NodeType::First {
            iter_nodes.push(node);
        }
    }
    let mut ghost_steps = vec![];
    ghost_steps.resize(iter_nodes.len(), 0);

    // calculate for each ghost: number of steps until the first finish node
    // then calculate the least common multiple (lcm).
    // This might not work in the general case, but in this graph it works.
    for i in 0..iter_nodes.len() {
        let mut pos = 0;
        let mut iter_node = iter_nodes[i];
        let ghost_step = &mut ghost_steps[i];
        while iter_node.node_type != NodeType::Last {
            let direction = &directions[pos];
            let next_id = match direction {
                Direction::Left => iter_node.left,
                Direction::Right => iter_node.right
            };

            iter_node = &nodes[nodes.binary_search_by(|n| n.current.cmp(&next_id)).unwrap()];
            pos = (pos + 1) % path_length;
            *ghost_step += 1;
        }
    }

    // lcm algorithm: use num-integer crate
    let lcm = ghost_steps.iter().fold(1 as u64, |acc, x| num_integer::lcm(acc, *x));
    println!("lcm: {lcm}");
}

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

#[derive(PartialEq,Debug,Clone,Copy)]
enum NodeType {
    First,
    Last,
    Normal
}

#[derive(Debug,Clone,Copy)]
struct Node {
    current: u64,
    left: u64,
    right: u64,
    node_type: NodeType
}

impl Node {
    fn parse(current_str: &str, left_str: &str, right_str: &str) -> Node {
        let mut nums = [0;3];

        for (i,  s) in [current_str, left_str, right_str].iter().enumerate() {
            nums[i] = parse_node_id(s);
        }

        let node_type = match current_str.chars().skip(2).next().unwrap() {
            'A' => NodeType::First,
            'Z' => NodeType::Last,
            _ => NodeType::Normal
        };

        Node {
            current: nums[0],
            left: nums[1],
            right: nums[2],
            node_type
        }
    }
}

fn parse_node_id(s: &str) -> u64 {
    let l = s.len();
    if l != 3 {
        panic!("failed to parse node id");
    }

    let base = 10 + 26 + 26; // 0-9, a-z, A-Z
    let mut id = 0;
    let mut factor = 1;
    for c in s.chars().rev() {
        id += (c as u64 - '0' as u64) * factor;
        factor *= base;
    }

    id
}
