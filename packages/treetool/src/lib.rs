pub use crate::generic_tree::BuildGenericTree;

pub mod generic_tree;
pub mod tree;

#[test]
fn test() {
    // let mut v = vec![];
    // for i in 1..10 {
    //     v.push(i);
    //     v.clone().build(2).print();

    let t = vec![1, 2, 3].build(2);
    t.print();
    assert_eq!(1, 1);
}
