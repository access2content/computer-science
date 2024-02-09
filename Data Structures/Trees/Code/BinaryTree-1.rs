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

fn main() {
    // Create the Tree
    println!("Enter root: ");
    let root = create_tree();

    //  Print in Order
    println!("In Order: ");
    in_order(&root);

    println!("Pre Order: ");
    pre_order(&root);

    println!("Post Order: ");
    post_order(&root);

    println!("Height: {:?}", height(&root));
    println!("Size: {:?}", size(&root));
    println!("Max: {:?}", max(&root));
}
