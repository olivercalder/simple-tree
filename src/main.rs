use std::env;

use simple_tree::DirTree;

fn main() {
    let mut was_arg = false;
    for root in env::args().skip(1) {
        was_arg = true;

        let tree = DirTree::new(root).unwrap();
        println!("{}", tree);
    }

    if !was_arg {
        let tree = DirTree::new(".").unwrap();
        println!("{}", tree);
    }
}
