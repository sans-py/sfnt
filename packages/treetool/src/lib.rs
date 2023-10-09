pub use crate::generic_tree::BuildGenericTree;
use crate::tree::Node;

pub mod generic_tree;
pub mod tree;

#[test]
fn test() {
    let mut v = vec![];
    for i in 1..10 {
        v.push(i);
        println!("{:?}", v);
        v.clone().build(2).print();
    }
}
