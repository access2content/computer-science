use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct BinaryTree {
    pub value: i32,
    left: Option<Box<BinaryTree>>,
    right: Option<Box<BinaryTree>>,
}

impl BinaryTree {
    pub fn new(value: i32) -> Self {
        BinaryTree {
            value,
            left: None,
            right: None,
        }
    }
}

fn read() -> i32 {
    let mut line = String::new();

    std::io::stdin().read_line(&mut line).unwrap();

    let number: i32 = line.trim().parse().unwrap();
    return number;
}

fn create_tree() -> Option<Box<BinaryTree>> {
    //  Get input from user
    let value = read();
    if value == -1 {
        return None;
    }

    let mut root = Box::new(BinaryTree::new(value));
    println!("Enter value for left of: {:?}", value);
    root.left = create_tree();
    println!("Enter value for right of: {:?}", value);
    root.right = create_tree();

    Some(root)
}

fn in_order(root: &Option<Box<BinaryTree>>) {
    match root {
        None => {
            return;
        }
        Some(root) => {
            in_order(&root.left);
            print!("{:?}, ", root.value);
            in_order(&root.right);
        }
    }
}

fn pre_order(root: &Option<Box<BinaryTree>>) {
    match root {
        None => {
            return;
        }
        Some(root) => {
            print!("{:?}, ", root.value);
            pre_order(&root.left);
            pre_order(&root.right);
        }
    }
}

fn post_order(root: &Option<Box<BinaryTree>>) {
    match root {
        None => {
            return;
        }
        Some(root) => {
            post_order(&root.left);
            post_order(&root.right);
            print!("{:?}, ", root.value);
        }
    }
}

fn height(root: &Option<Box<BinaryTree>>) -> usize {
    match root {
        None => 0,
        Some(data) => std::cmp::max(height(&data.left), height(&data.right)) + 1,
    }
}

fn size(root: &Option<Box<BinaryTree>>) -> usize {
    match root {
        None => 0,
        Some(node) => size(&node.left) + size(&node.right) + 1,
    }
}

fn max(root: &Option<Box<BinaryTree>>) -> i32 {
    match root {
        None => i32::MIN,
        Some(node) => std::cmp::max(node.value, std::cmp::max(max(&node.left), max(&node.right))),
    }
}

fn print_level(root: &Option<Box<BinaryTree>>, level: usize) {
    match root {
        None => return,
        Some(node) => {
            if level == 1 {
                print!("{:?}, ", node.value);
            } else if level > 1 {
                print_level(&node.left, level - 1);
                print_level(&node.right, level - 1);
            }
        }
    }
}

fn level_order(root: &Option<Box<BinaryTree>>) {
    if root.is_none() {
        return;
    }

    for i in 1..height(&root) + 1 {
        print_level(&root, i);
        println!("");
    }
}

fn level_order_queue(root: &Option<Box<BinaryTree>>) {
    let mut queue = VecDeque::new();
    queue.push_back(root);
    queue.push_back(&None);

    while !queue.is_empty() {
        let data = queue.pop_front();

        if data.is_none() {
            break;
        }

        match data.unwrap() {
            None => {
                if queue.is_empty() {
                    break;
                }
                queue.push_back(&None);
                println!("");
            }

            Some(node) => {
                print!("{:?}, ", node.value);

                if !node.left.is_none() {
                    queue.push_back(&node.left);
                }

                if !node.right.is_none() {
                    queue.push_back(&node.right);
                }
            }
        }
    }
}

fn left_view(root: &Option<Box<BinaryTree>>, level: usize, output: &mut Vec<i32>) {
    match root {
        None => return,
        Some(node) => {
            match output[level] {
                -1 => {
                    output[level] = node.value;
                }
                _ => {}
            }
            left_view(&node.left, level + 1, output);
            left_view(&node.right, level + 1, output);
        }
    }
}

fn print_left(root: &Option<Box<BinaryTree>>) {
    let size: usize = height(root);
    let mut output: Vec<i32> = vec![-1; size];
    left_view(root, 0, &mut output);
    // Gets the left view and prints it?
    println!("Output: {:?}", output);
}

fn right_view(root: &Option<Box<BinaryTree>>, level: usize, output: &mut Vec<i32>) {
    match root {
        None => return,
        Some(node) => {
            output[level] = node.value;
            right_view(&node.left, level + 1, output);
            right_view(&node.right, level + 1, output);
        }
    }
}

fn print_right(root: &Option<Box<BinaryTree>>) {
    let size: usize = height(root);
    let mut output: Vec<i32> = vec![-1; size];
    right_view(root, 0, &mut output);
    // Gets the left view and prints it?
    println!("Output: {:?}", output);
}

fn top_view_dfs(
    root: &Option<Box<BinaryTree>>,
    distance: i32,
    level: usize,
    output: &mut HashMap<i32, (usize, i32)>,
) {
    // Map<distance, <value, height>>
    match root {
        None => return,
        Some(node) => {
            top_view_dfs(&node.left, distance - 1, level + 1, output);

            //  Check if something exists for the distance.
            match output.get(&distance) {
                // If not, insert it
                None => {
                    output.insert(distance, (level, node.value));
                }
                Some((node_level, _)) => {
                    //  Check if the level is smaller. If so, replace it
                    if level < *node_level {
                        output.insert(distance, (level, node.value));
                    }
                }
            };

            top_view_dfs(&node.right, distance + 1, level + 1, output);
        }
    }
}

fn print_top_view(root: &Option<Box<BinaryTree>>) {
    let mut map: HashMap<i32, (usize, i32)> = HashMap::new();
    top_view_dfs(&root, 0, 0, &mut map);

    let mut keys: Vec<i32> = map.keys().copied().collect();
    keys.sort();

    for key in keys {
        match map.get(&key) {
            None => {}
            Some(&(_, data)) => {
                print!("{data}, ");
            }
        }
    }
}

fn level_view(
    root: &Option<Box<BinaryTree>>,
    level: usize,
    distance: i32,
    output: &mut HashMap<i32, (usize, i32)>,
) {
    match root {
        None => return,
        Some(node) => {
            if level == 1 {
                match output.get(&distance) {
                    None => {
                        output.insert(distance, (level, node.value));
                    }
                    Some((level_node, _)) => {
                        if level < *level_node {
                            output.insert(distance, (level, node.value));
                        }
                    }
                }
            } else {
                level_view(&node.left, level - 1, distance - 1, output);
                level_view(&node.right, level - 1, distance + 1, output);
            }
        }
    }
}

fn top_view_bfs(root: &Option<Box<BinaryTree>>) {
    let height = height(&root);
    let mut output: HashMap<i32, (usize, i32)> = HashMap::new();

    for i in 1..height + 1 {
        level_view(&root, i, 0, &mut output);
    }

    //  Print it
    let mut keys: Vec<i32> = output.keys().copied().collect();
    keys.sort();

    for key in keys {
        match output.get(&key) {
            None => {}
            Some(&(_, data)) => {
                print!("{data}, ");
            }
        }
    }
}

fn main() {
    // Create the Tree
    println!("Enter root: ");
    let root = create_tree();

    //  Print in Order
    println!("*****");
    println!("In Order: ");
    in_order(&root);

    println!("");
    println!("");
    println!("Pre Order: ");
    pre_order(&root);

    println!("");
    println!("");
    println!("Post Order: ");
    post_order(&root);

    println!("");
    println!("");
    println!("Height: {:?}", height(&root));
    println!("Size: {:?}", size(&root));
    println!("Max: {:?}", max(&root));

    println!("");
    println!("Level Order:");
    level_order(&root);

    println!("");
    println!("Level Order Queue:");
    level_order_queue(&root);

    println!("");
    println!("Left View:");
    print_left(&root);

    println!("");
    println!("Right View:");
    print_right(&root);

    println!("");
    println!("Top View DFS:");
    print_top_view(&root);

    println!("");
    println!("Top View BFS:");
    top_view_bfs(&root);
}
