use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::node::Node;

#[derive(Debug)]
enum SpecialFile {
    None,
    Root,
    Symlink(PathBuf),
}

#[derive(Debug)]
pub struct DirTree {
    path: PathBuf,
    special: SpecialFile,
    children: Vec<DirTree>,
}

impl DirTree {
    /// Reads the directory tree starting at the given path and constructs a new [DirTree] and
    /// returns it.
    ///
    /// # Errors
    ///
    /// This function will error in the following cases, but is not limited to just these cases:
    ///
    /// - The provided `path` doesn't exist.
    /// - The process lacks permission to view the contents.
    /// - The provided `path` is a directory and the process lacks permission to stat any file
    ///   within that directory or any subdirectory of it.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::env;
    /// use std::fs;
    /// use std::path::{MAIN_SEPARATOR, Path};
    /// use simple_tree::implementations::DirTree;
    /// use simple_tree::Node;
    ///
    /// let tmpdir = env::temp_dir().join("test-dir-tree-new");
    /// fs::create_dir(&tmpdir).unwrap();
    ///
    /// let foo_path = tmpdir.join("foo");
    /// fs::create_dir(&foo_path).unwrap();
    ///
    /// let bar_path = foo_path.join("bar");
    /// let baz_path = foo_path.join("baz");
    /// fs::create_dir(&bar_path).unwrap();
    /// fs::create_dir(&baz_path).unwrap();
    ///
    /// let a_path = bar_path.join("a");
    /// let b_path = bar_path.join("b");
    /// let c_path = bar_path.join("c");
    /// fs::create_dir(&a_path).unwrap();
    /// #[cfg(unix)] {
    ///     std::os::unix::fs::symlink(&tmpdir, &b_path).unwrap();
    /// }
    /// #[cfg(windows)] {
    ///     std::os::windows::fs::symlink_dir(&tmpdir, &b_path).unwrap();
    /// }
    /// fs::write(&c_path, "stuff").unwrap();
    ///
    /// let mut root = DirTree::new(&foo_path).unwrap();
    ///
    /// // The filesystem is not read again after `DirTree::new()` returns.
    /// // Clean up the directory we created for this test.
    /// fs::remove_dir_all(&tmpdir).unwrap();
    ///
    /// assert_eq!(format!("{}", root), format!("
    /// {}{}foo
    /// ├── bar
    /// │   ├── a
    /// │   ├── b -> {}
    /// │   └── c
    /// └── baz",tmpdir.display(), MAIN_SEPARATOR, tmpdir.display())
    /// );
    /// ```
    pub fn new<P>(path: P) -> io::Result<Self>
    where
        PathBuf: From<P>,
    {
        let path = PathBuf::from(path);
        Self::new_internal(path, true)
    }

    fn new_internal(path: PathBuf, is_root: bool) -> io::Result<Self> {
        let metadata = path.symlink_metadata()?;
        let special = match (is_root, metadata.is_symlink()) {
            (true, _) => SpecialFile::Root,
            (_, true) => SpecialFile::Symlink(path.read_link()?),
            _ => SpecialFile::None,
        };
        let mut children = Vec::new();
        // Only traverse symlinks if the path is the root of the dir tree.
        if metadata.is_dir() || is_root && metadata.is_symlink() && path.metadata()?.is_dir() {
            for entry in fs::read_dir(&path)? {
                children.push(Self::new_internal(entry?.path(), false)?);
            }
        }
        children.sort_by(|a, b| a.path.cmp(&b.path));
        Ok(DirTree {
            path,
            special,
            children,
        })
    }
}

impl Node for DirTree {
    /// Returns a lossy string representation the path associated with this node.
    ///
    /// # Examples
    ///
    /// The example is derived from that of [std::ffi::OsStr::to_string_lossy].
    ///
    /// ```
    /// // Note, due to differences in how Unix and Windows represent strings,
    /// // we are forced to complicate this example, setting up example `OsStr`s
    /// // with different source data and via different platform extensions.
    ///
    /// #[cfg(unix)] {
    ///     use std::env;
    ///     use std::ffi::OsStr;
    ///     use std::fs;
    ///     use std::os::unix::ffi::OsStrExt;
    ///     use std::path::MAIN_SEPARATOR;
    ///     use simple_tree::implementations::DirTree;
    ///     use simple_tree::Node;
    ///
    ///     let tmpdir = env::temp_dir().join("test-dir-tree-value");
    ///     fs::create_dir(&tmpdir).unwrap();
    ///
    ///     // Here, the values 0x66 and 0x6f correspond to 'f' and 'o'
    ///     // respectively. The value 0x80 is a lone continuation byte, invalid
    ///     // in a UTF-8 sequence.
    ///     let source = [0x66, 0x6f, 0x80, 0x6f];
    ///     let os_str = OsStr::from_bytes(&source[..]);
    ///
    ///     let path = tmpdir.join(os_str);
    ///     fs::create_dir(&path).unwrap();
    ///
    ///     let mut root = DirTree::new(&path).unwrap();
    ///
    ///     // Clean up the directory we created for this test
    ///     fs::remove_dir_all(&tmpdir).unwrap();
    ///
    ///     assert_eq!(
    ///         format!("{}", root.value()),
    ///         format!(
    ///             "{}{}{}",
    ///             tmpdir.display(),
    ///             MAIN_SEPARATOR,
    ///             "fo�o"
    ///         )
    ///     );
    /// }
    /// #[cfg(windows)] {
    ///     use std::env;
    ///     use std::ffi::OsString;
    ///     use std::fs;
    ///     use std::os::windows::prelude::*;
    ///     use std::path::MAIN_SEPARATOR;
    ///     use simple_tree::implementations::DirTree;
    ///     use simple_tree::Node;
    ///
    ///     let tmpdir = env::temp_dir().join("test-dir-tree-value");
    ///     fs::create_dir(&tmpdir).unwrap();
    ///
    ///     // Here the values 0x0066 and 0x006f correspond to 'f' and 'o'
    ///     // respectively. The value 0xD800 is a lone surrogate half, invalid
    ///     // in a UTF-16 sequence.
    ///     let source = [0x0066, 0x006f, 0xD800, 0x006f];
    ///     let os_string = OsString::from_wide(&source[..]);
    ///     let os_str = os_string.as_os_str();
    ///
    ///     let path = tmpdir.join(os_str);
    ///     fs::create_dir(&path).unwrap();
    ///
    ///     let mut root = DirTree::new(&path).unwrap();
    ///
    ///     // Clean up the directory we created for this test
    ///     fs::remove_dir_all(&tmpdir).unwrap();
    ///
    ///     assert_eq!(
    ///         format!("{}", root.value()),
    ///         format!(
    ///             "{}{}{}",
    ///             parent.display(),
    ///             MAIN_SEPARATOR,
    ///             "fo�o"
    ///         )
    ///     );
    /// }
    /// ```
    fn value(&self) -> impl fmt::Display {
        match &self.special {
            SpecialFile::None => self
                .path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            SpecialFile::Root => self.path.as_os_str().to_string_lossy().into_owned(),
            SpecialFile::Symlink(target) => {
                format!(
                    "{} -> {}",
                    self.path.file_name().unwrap().display(),
                    target.as_os_str().display()
                )
            }
        }
    }

    /// If `self` is associated with a directory, returns an iterator over the path entries in that
    /// directory. If `self` is not associated with a directory (e.g. because it is associated with a file instead), or the directory is empty, the iterator is empty.
    ///
    /// The directory tree is computed during a previous call to [DirTree::new], and no further
    /// reads of the filesystem are made during this method call.
    ///
    /// Except for the root of the [DirTree], symbolic links are not traversed when constructing
    /// the tree, so entries associated with symbolic links will return an empty iterator.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::env;
    /// use std::fs;
    /// use std::path::{MAIN_SEPARATOR, Path};
    /// use simple_tree::implementations::DirTree;
    /// use simple_tree::Node;
    ///
    /// let tmpdir = env::temp_dir().join("test-dir-tree-children");
    /// fs::create_dir(&tmpdir).unwrap();
    ///
    /// let foo_path = tmpdir.join("foo");
    /// fs::create_dir(&foo_path).unwrap();
    ///
    /// let bar_path = foo_path.join("bar");
    /// let baz_path = foo_path.join("baz");
    /// fs::write(&bar_path, "stuff").unwrap();
    /// fs::create_dir(&baz_path).unwrap();
    ///
    /// let qux_path = baz_path.join("qux");
    /// fs::create_dir(&qux_path).unwrap();
    ///
    /// let mut root = DirTree::new(&foo_path).unwrap();
    ///
    /// // The filesystem is not read again after `DirTree::new()` returns.
    /// // Clean up the directory we created for this test.
    /// fs::remove_dir_all(&tmpdir).unwrap();
    ///
    /// let mut children = root.children();
    /// assert_eq!(format!("{}", children.next().unwrap().value()), "bar");
    /// assert_eq!(format!("{}", children.next().unwrap().value()), "baz");
    /// assert!(children.next().is_none());
    /// ```
    fn children(&self) -> impl Iterator<Item = &Self> {
        self.children.iter()
    }
}

impl fmt::Display for DirTree {
    /// Format using the default [Node::fmt] implementation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Node::fmt(self, f)
    }
}
