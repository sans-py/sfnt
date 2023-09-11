use generic_tree::BuildGenericTree;

mod generic_tree;
mod tree;

#[test]
fn test() {
    let mut v = vec![];
    for i in 1..10 {
        v.push(i);
        v.clone().build(2).print();
    }

    assert_eq!(1, 1);
}
