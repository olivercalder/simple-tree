use std::fmt;

#[derive(Debug, Clone, Copy)]
enum SpecialStatus {
    None,
    Root,
    LastChild,
}

/// Node defines common methods for different node implementations.
pub trait Node<T>
where
    T: fmt::Display,
{
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
