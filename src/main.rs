// #![allow(unused_variables)]
struct Node {
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(data: i32) -> Node {
        Node {
            data,
            left: None,
            right: None,
        }
    }

    /// Insert a new node in the tree
    fn insert(&mut self, data: i32) {
        if self.data == data {
            return;
        }

        // Add a new node to the left branch
        if data < self.data {
            match &mut self.left {
                None => self.left = Some(Box::new(Node::new(data))),
                Some(child_node) => child_node.insert(data),
            }
        // Add a new node to the right branch
        } else {
            match &mut self.right {
                None => self.right = Some(Box::new(Node::new(data))),
                Some(child_node) => child_node.insert(data),
            }
        }
    }

    /// shows each node starting from the left branch following
    /// the root and ending at the right branch
    fn print_inorder(&self) {
        if let Some(child_node) = &self.left {
            child_node.print_inorder();
        }
        
        print!(" {} ", self.data);

        if let Some(child_node) = &self.right {
            child_node.print_inorder();
        }
    }

    fn search(&self, target: i32) -> Option<i32> {
        if target == self.data {
            Some(target)
        }
        //found
        else if target < self.data {
            self.left.as_ref()?.search(target)
        } else if target > self.data {
            self.right.as_ref()?.search(target)
        } else {
            None
        } //not found
    }

    // fn delete(&mut self, value: i32) -> Result<(), String> {
    //     match Node::search(self, value) {
    //         Some(_) => {}
    //         None => return Err("Node does not exist".to_string()),
    //     }

    //     Ok(())
    // }

    // fn height(&self) -> u32 {
    //     match self {
    //         Node {left: None, right: None, ..} => return 1,
    //         Node {left: Some(_), right: None, ..} => let 
    //     }
    // }
}

fn main() {
    /*
             13
          /     \
         11     17
        /      /  \
       5     16  20
        \
         7
    */
    let mut root = Node::new(13);
    root.insert(11);
    root.insert(5);
    root.insert(7);
    root.insert(17);
    root.insert(16);
    root.insert(20);
    println!("{:?}", Node::search(&root, 7));
    root.print_inorder();
}
