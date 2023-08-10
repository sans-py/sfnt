pub struct GenericTree<T> {
    pub degree: usize,
    pub depth: usize,
    pub body: Vec<Option<Node<T>>>,
}

impl<T> GenericTree<T> {
    pub fn get_degree(&self) -> usize {
        self.degree
    }
    pub fn get_depth(&self) -> usize {
        self.depth
    }
    pub fn get_root(&self) -> (&Option<Node<T>>, usize) {
        if self.body.len() == 0 {
            (&None, 0)
        } else {
            (&self.body[0], 0)
        }
    }
    pub fn get_parent(&self, index: usize) -> (&Option<Node<T>>, usize) {
        let prev_depth = (index as f64).log(self.degree as f64).floor() as u32;
        let index_in_depth = index - self.degree.pow(prev_depth);
        let parent = prev_depth as usize + (index_in_depth as f64 / self.degree as f64) as usize;
        
        if (0..self.body.len()).contains(&parent) {
            (&self.body[parent], parent)
        } else {
            (&None, parent)
        }

    }
    pub fn get_prev_sibling(&self, index: usize) -> (&Option<Node<T>>, usize) {
        let prev = index - 1;
        if (0..self.body.len()).contains(&prev) {
            (&self.body[prev], prev)
        } else {
            (&None, prev)
        }
    }
    pub fn get_next_sibling(&self, index: usize) -> (&Option<Node<T>>, usize) {
        let next = index + 1;
        if (0..self.body.len()).contains(&next) {
            (&self.body[next], next)
        } else {
            (&None, next)
        }
    }
    pub fn get_first_child(&self, index: usize) -> (&Option<Node<T>>, usize) {
        let now_depth = (index as f64).log(self.degree as f64).ceil() as u32;
        let index_in_depth = index - self.degree.pow(now_depth - 1);
        let first_child = self.degree.pow(now_depth) + index_in_depth * self.degree;
        
        if (0..self.body.len()).contains(&first_child ) {
            (&self.body[first_child], first_child)
        } else {
            (&None, first_child)
        }
    }

}

pub struct Node<T> {
    pub value: T
}