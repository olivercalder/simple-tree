use std::cmp::Ordering;
use std::fmt;

use crate::node::Node;

#[derive(Debug)]
pub struct NodeBinaryUnbalanced<T>
where
    T: fmt::Display + Ord,
{
    val: T,
    count: usize,
    left: Option<Box<NodeBinaryUnbalanced<T>>>,
    right: Option<Box<NodeBinaryUnbalanced<T>>>,
}

impl<T> NodeBinaryUnbalanced<T>
where
    T: fmt::Display + Ord,
{
    /// Returns a new node with the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_tree::implementations::NodeBinaryUnbalanced;
    /// use simple_tree::Node;
    ///
    /// let root = NodeBinaryUnbalanced::new("foo");
    ///
    /// assert_eq!(format!("{}", root.value()), "foo");
    /// assert_eq!(root.count_children(), 0);
    /// ```
    pub fn new(value: T) -> Self {
        NodeBinaryUnbalanced {
            val: value,
            count: 1,
            left: None,
            right: None,
        }
    }

    /// Creates a new node with the given value and adds it to the tree under self.
    /// If the new value is less than the value of self, inserts the new node as the left child of
    /// self if none exists, otherwise calls `insert(value)` on the existing left child.
    ///
    /// If the new value is greater than the value of self, inserts the new node as the right child
    /// of self if none exists, otherwise calls `insert(value)` on the existing right child.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_tree::implementations::NodeBinaryUnbalanced;
    /// use simple_tree::Node;
    ///
    /// let mut root = NodeBinaryUnbalanced::new(7);
    /// root.insert(3);
    /// root.insert(5);
    /// root.insert(13);
    /// root.insert(2);
    /// root.insert(11);
    /// root.insert(15);
    ///
    /// assert_eq!(format!("{}", root),
    /// "7
    /// ├── 3
    /// │   ├── 2
    /// │   └── 5
    /// └── 13
    ///     ├── 11
    ///     └── 15");
    /// ```
    pub fn insert(&mut self, value: T) {
        match value.cmp(&self.val) {
            Ordering::Less => {
                if let Some(left) = &mut self.left {
                    left.insert(value);
                } else {
                    self.left = Some(Box::new(Self::new(value)));
                }
            }
            Ordering::Equal => self.count += 1,
            Ordering::Greater => {
                if let Some(right) = &mut self.right {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(Self::new(value)));
                }
            }
        }
    }
}

impl<T> Node for NodeBinaryUnbalanced<T>
where
    T: fmt::Display + Ord,
{
    /// Returns the value stored in [self].
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_tree::implementations::NodeBinaryUnbalanced;
    /// use simple_tree::Node;
    ///
    /// let root = NodeBinaryUnbalanced::new("hello");
    ///
    /// assert_eq!(format!("{}", root.value()), "hello");
    /// assert_eq!(root.children().count(), 0);
    /// ```
    fn value(&self) -> impl fmt::Display {
        &self.val
    }

    /// Returns an iterator which yields the left child, if present, and then the right child, if
    /// present.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_tree::implementations::NodeBinaryUnbalanced;
    /// use simple_tree::Node;
    ///
    /// let mut root = NodeBinaryUnbalanced::new(50);
    /// root.insert(25);
    /// root.insert(100);
    /// root.insert(75);
    ///
    /// let mut children = root.children();
    /// assert_eq!(format!("{}", children.next().unwrap().value()), "25");
    /// assert_eq!(format!("{}", children.next().unwrap().value()), "100");
    /// assert!(children.next().is_none());
    /// ```
    fn children(&self) -> impl Iterator<Item = &Self> {
        let mut ch = Vec::new();
        if let Some(left) = &self.left {
            ch.push(left.as_ref());
        }
        if let Some(right) = &self.right {
            ch.push(right.as_ref());
        }
        ch.into_iter()
    }
}

impl<T> fmt::Display for NodeBinaryUnbalanced<T>
where
    T: fmt::Display + Ord,
{
    /// Format using the default [Node::fmt] implementation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Node::fmt(self, f)
    }
}
