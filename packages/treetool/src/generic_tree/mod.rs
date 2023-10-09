use std::fmt::Display;

use crate::tree;

pub trait BuildGenericTree<T: Clone + Copy + Display> {
    fn build(&self, degree: usize) -> tree::GenericTree<T>;
}

impl<T: Clone + Copy + Display> BuildGenericTree<T> for Vec<T> {
    fn build(&self, degree: usize) -> tree::GenericTree<T> {
        tree::GenericTree::new(self, degree)
    }
}
