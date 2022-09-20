/// A simple implementation of a binary tree in Rust.
///
/// A binary tree is a data structure that is made up of nodes,
/// each of which is linked to two other nodes, that is why it is
/// called a binary tree.
///
/// This binary tree has the property that the nodes that are to the
/// right of another, it is because they are greater than the parent,
/// if they are to the left it is because they are less.
struct Node {
    data: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    /// Create a new binary tree such that the value of the main node is data.
    fn new(data: i32) -> Node {
        Node {
            data,
            left: None,
            right: None,
        }
    }

    fn new_from_vector(vector: &Vec<i32>) -> Node {
        let mut root = Node::new(vector[0]);

        for i in &vector[1..vector.len()] {
            root.insert(*i);
        }

        root
    }

    /// Insert a new node in the treeInserts a new node depending on the
    /// value of the parent, if the value of the new node is less than
    /// the parent the node will be entered on the left, if it is greater
    /// it will be entered on the right.
    ///
    /// # Example
    ///
    /// We will create the following binary tree.
    ///
    ///```
    ///      13
    ///    /    \
    ///   11    17
    ///  /     /  \
    /// 5     16  20
    ///  \
    ///   7
    /// ```
    ///
    /// The corresponding code is:
    ///
    /// ```rust
    /// let mut root = Node::new(13);
    /// root.insert(11);
    /// root.insert(5);
    /// root.insert(7);
    /// root.insert(17);
    /// root.insert(16);
    /// root.insert(20);
    /// ```
    fn insert(&mut self, data: i32) {
        // If the new node has a value that already exists then it is
        // ignored
        if self.data == data {
            return;
        }

        // If the value is less than the parent, it is added to the left
        if data < self.data {
            match &mut self.left {
                None => self.left = Some(Box::new(Node::new(data))),
                Some(child_node) => child_node.insert(data),
            }

        //If the value is greater than the parent, it is added to the right
        } else {
            match &mut self.right {
                None => self.right = Some(Box::new(Node::new(data))),
                Some(child_node) => child_node.insert(data),
            }
        }
    }

    /// Get the ordered list of all data from each of the nodes
    fn sort_elements(&self) -> Vec<i32> {
        let mut nodevector: Vec<i32> = Vec::new();

        let leftvector = if let Some(child_node) = &self.left {
            child_node.sort_elements()
        } else {
            Vec::<i32>::new()
        };

        nodevector.push(self.data);

        let rightvector = if let Some(child_node) = &self.right {
            child_node.sort_elements()
        } else {
            Vec::<i32>::new()
        };

        nodevector.extend(rightvector);

        [&leftvector[..], &nodevector[..]].concat()
    }

    /// Search for a node, if the node exists it returns its value
    fn search(&self, target: i32) -> Option<i32> {
        // If the value is equal to the current node, it is returned
        if target == self.data {
            Some(target)
        }
        // If the value is different from the parent, the nodes on the
        // left are searched if the value is less than the parent, or on
        // the right if it is greater.
        else if target < self.data {
            self.left.as_ref()?.search(target)
        } else if target > self.data {
            self.right.as_ref()?.search(target)
        } else {
            None // If the value is not found at any node
        }
    }
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
    let root = Node::new_from_vector(&vec![13, 11, 17, 5, 16, 20, 7]);
    println!("{:?}", root.search(7));
    println!("{:?}", root.sort_elements());
}
