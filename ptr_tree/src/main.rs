use std::ptr;

fn main() {
    let mut root = Node::from_value(5f64);
    
    let mut left = Node::from_value(1f64);

    let mut right = Node::from_value(6f64);

    root.assign_left(&mut left);
    root.assign_right(&mut right);

    let bin_tree = BinaryTree::from_node(&mut root);
    bin_tree.print();
}

#[derive(Debug)]
struct Node {
    value: f64,
    left: *mut Node,
    right: *mut Node,
}

impl Node {
    fn from_value(value: f64) -> Node {
        Node {
            value,
            left: ptr::null_mut(),
            right: ptr::null_mut(),
        }
    }

    fn get_value(&self) -> f64 {
        self.value
    }

    fn assign_left(&mut self, node: &mut Node) {
        self.left = node;
    }

    fn assign_right(&mut self, node: &mut Node) {
        self.right = node;
    }

    fn print(&self, depth: usize) {
        println!("depth: {}, value: {}", depth, self.value);
        if self.left.is_null() {
            println!("Finish Left at depth: {}!", depth);
        } else {
            unsafe {
                println!("Left: ");
                (*(self.left)).print(depth + 1);
            }
        }
        if self.right.is_null() {
            println!("Finish Right at depth: {}!", depth);
        } else {
            unsafe {
                println!("Right: ");
                (*(self.right)).print(depth + 1);
            }
        }
    }
}

#[derive(Debug)]
struct BinaryTree {
    root: *mut Node,
}

impl BinaryTree {
    fn from_node(node: &mut Node) -> BinaryTree {
        BinaryTree {
            root: node,
        }
    }

    fn print(&self) {
        unsafe {
            (*(self.root)).print(0usize);
        }
    }
}
