use std::cmp::Ordering;
use std::env;
use std::fmt;

/// # Simple Tree
///
/// Simple tree is a simple, growable, printable tree.

#[derive(Debug, Clone, Copy)]
enum SpecialStatus {
    None,
    Root,
    LastChild,
}

/// Node defines common methods for different node implementations.
trait Node<T>
where
    T: fmt::Display + Ord,
{
    fn new(value: T) -> Self;
    fn insert(&mut self, value: T);
    fn value(&self) -> &T;
    fn children(&self) -> impl Iterator<Item = &Box<Self>>;

    fn count_children(&self) -> usize {
        self.children().count()
    }

    fn count_descendents(&self) -> usize {
        self.children().map(|c| c.count_descendents() + 1).sum()
    }

    fn print_tree(
        &self,
        f: &mut fmt::Formatter<'_>,
        prefixes: &mut Vec<&str>,
        status: SpecialStatus,
    ) -> fmt::Result {
        for pref in prefixes.iter() {
            write!(f, "{}", pref)?;
        }
        let (s_prefix, c_prefix) = match status {
            SpecialStatus::None => ("├── ", "│   "),
            SpecialStatus::Root => ("", ""),
            SpecialStatus::LastChild => ("└── ", "    "),
        };
        write!(f, "{}{}", s_prefix, self.value())?;
        prefixes.push(c_prefix);
        let count = self.count_children();
        for (i, c) in self.children().enumerate() {
            let c_status = if i == count - 1 {
                SpecialStatus::LastChild
            } else {
                SpecialStatus::None
            };
            c.print_tree(f, prefixes, c_status)?;
        }
        let _ = prefixes.pop();
        Ok(())
    }

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut prefixes = vec!["\n"];
        self.print_tree(f, &mut prefixes, SpecialStatus::Root)
    }
}

#[derive(Debug)]
struct NodeBinaryUnbalanced<T>
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

fn main() {
    let mut nums = env::args()
        .skip(1)
        .map(|n| n.parse::<i32>().expect("Arguments must be integers"));
    let Some(first) = nums.next() else { return };
    let mut root = NodeBinaryUnbalanced::new(first);
    for num in nums {
        root.insert(num)
    }

    println!("{}", root);
    println!(
        "Number of descendents from root: {}",
        root.count_descendents()
    );
}

#[cfg(test)]
mod tests {}
