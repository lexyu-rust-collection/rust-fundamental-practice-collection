// Box Pointer to used when you need a large amount of data
// that stored on the heap and then you pass pointers to it
// on the stack
pub fn output() {
    let b_int1 = Box::new(10);
    println!("b_int1 = {}", b_int1);

    let node1 = TreeNode::new(1)
        .left(TreeNode::new(2))
        .right(TreeNode::new(3));
    println!("{:#?}", node1);
}

#[derive(Debug)]
struct TreeNode<T> {
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
    pub key: T,
}
impl<T> TreeNode<T> {
    pub fn new(key: T) -> Self {
        TreeNode {
            left: None,
            right: None,
            key,
        }
    }
    pub fn left(mut self, node: TreeNode<T>) -> Self {
        self.left = Some(Box::new(node));
        self
    }
    pub fn right(mut self, node: TreeNode<T>) -> Self {
        self.right = Some(Box::new(node));
        self
    }
}
