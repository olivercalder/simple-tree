use std::fmt;
use std::fs;
pub use std::path::PathBuf;

use crate::node::Node;

impl Node for PathBuf {
    fn value(&self) -> impl fmt::Display {
        self.file_name().unwrap().display()
    }

    fn children(&self) -> impl Iterator<Item = &Self> {
        if !self.is_dir() {
            return vec![].into_iter();
        }
        fs::read_dir(self).unwrap().map(|e| e.unwrap().path())
    }
}
