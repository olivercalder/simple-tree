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

impl<T> Node<T> for NodeBinaryUnbalanced<T>
where
    T: fmt::Display + Ord,
{
    fn new(value: T) -> Self {
        NodeBinaryUnbalanced {
            val: value,
            count: 1,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
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

    fn value(&self) -> &T {
        &self.val
    }

    fn children(&self) -> impl Iterator<Item = &Box<Self>> {
        let mut ch = Vec::new();
        if let Some(left) = &self.left {
            ch.push(left);
        }
        if let Some(right) = &self.right {
            ch.push(right);
        }
        ch.into_iter()
    }
}

impl<T> fmt::Display for NodeBinaryUnbalanced<T>
where
    T: fmt::Display + Ord,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Node::fmt(self, f)
    }
}
