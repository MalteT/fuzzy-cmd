pub mod node;

use std::ops::{Deref, DerefMut};

use self::node::Node;

/// Reference to a [Node].
/// Encapsulates a reference to a `Node` for ease of use. See [fuzzy_cmd] for examples.
pub struct RefNode<'a> {
    node: &'a mut Node,
}

impl<'a> RefNode<'a> {
    /// Set this node's function.
    /// # Example
    /// ```wont_compile
    /// use fuzzy_cmd::{Node, RefNode};
    ///
    /// let mut node = Node::new();
    /// let mut rn = RefNode::from(&mut node);
    /// rn.call(|| {
    ///    println!("I was called!");
    /// })
    /// ```
    pub fn call<F: FnMut() + 'static>(self, f: F) {
        self.node.call(f);
    }
}

impl<'a> From<&'a mut Node> for RefNode<'a> {
    fn from(node: &'a mut Node) -> Self {
        RefNode { node }
    }
}

impl<'a> Deref for RefNode<'a> {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        self.node
    }
}

impl<'a> DerefMut for RefNode<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.node
    }
}
