use crate::tree;

pub trait BuildGenericTree<T> {
    fn build(&self, degree: usize) -> tree::GenericTree<T>;
}

impl<T: Clone> BuildGenericTree<T> for Vec<T> {
    fn build(&self, degree: usize) -> tree::GenericTree<T> {
        let depth = (self.len() as f64).log(degree as f64).ceil() as usize;
        let mut body: Vec<Option<tree::Node<T>>> = self.iter().map(|v| { Some(tree::Node{ value: v.clone() }) }).collect();
        body.resize_with(degree.pow(depth as u32), || { None });

        tree::GenericTree {
            degree: degree,
            depth: depth,
            body: body,
        }
    }
}
