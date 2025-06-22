use std::env;

use simple_tree::implementations::binary_unbalanced::NodeBinaryUnbalanced;
use simple_tree::node::Node;

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
