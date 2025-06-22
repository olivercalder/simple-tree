use std::fmt;

#[derive(Debug, Clone, Copy)]
enum SpecialStatus {
    None,
    Root,
    LastChild,
}

/// [Node] defines common methods for nodes in a displayable tree.
pub trait Node<T>
where
    T: fmt::Display,
{
    /// Returns the displayable value represented by this node.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_tree::implementations::NodeBinaryUnbalanced;
    /// use simple_tree::Node;
    ///
    /// let root = NodeBinaryUnbalanced::new("hello world!");
    ///
    /// assert_eq!(root.value(), &"hello world!");
    /// ```
    fn value(&self) -> &T;

    /// Returns an iterator over the children of this node. Children of those children must not be
    /// included.
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
    /// assert_eq!(children.next().unwrap().value(), &25);
    /// assert_eq!(children.next().unwrap().value(), &100);
    /// assert_eq!(children.next().is_none(), true);
    /// ```
    fn children(&self) -> impl Iterator<Item = &Self>;

    /// Returns the number of direct children of this node.
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
    /// assert_eq!(root.count_children(), 2);
    /// let mut children = root.children();
    ///
    /// let first = children.next().unwrap();
    /// assert_eq!(first.count_children(), 0);
    ///
    /// let second = children.next().unwrap();
    /// assert_eq!(second.count_children(), 1);
    /// ```
    fn count_children(&self) -> usize {
        self.children().count()
    }

    /// Returns the number of children of this node, and each of their children, recursively.
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
    /// assert_eq!(root.count_descendents(), 3);
    /// ```
    fn count_descendents(&self) -> usize {
        self.children().map(|c| c.count_descendents() + 1).sum()
    }

    /// Formats a tree rooted at the given node and writes the result to the given formatter.
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
    /// assert_eq!(format!("{}", root), "
    /// 7
    /// ├── 3
    /// │   ├── 2
    /// │   └── 5
    /// └── 13
    ///     ├── 11
    ///     └── 15");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut prefixes = vec!["\n"];
        self.print_tree(f, &mut prefixes, SpecialStatus::Root)
    }

    /// A helper method of [Self::fmt] which recursively prints a node and its children to the given
    /// formatter, given some existing prefixes and metadata about this node.
    ///
    /// This should never be called directly.
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
            SpecialStatus::None => ("├── ", "│   "),
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
}
