use std::fmt::Display;

#[derive(Clone, Copy)]
pub enum Node<T: Clone + Copy + Display> {
    Terminal(T),
    Intermediate,
}

pub struct GenericTree<T: Clone + Copy + Display> {
    degree: usize,
    length: usize,
    body: Vec<Option<Node<T>>>,
}

impl<T: Clone + Copy + Display> GenericTree<T> {
    pub fn new(data: &Vec<T>, degree: usize) -> Self {
        if data.len() == 0 {
            return GenericTree {
                body: Vec::new(),
                degree: degree,
                length: 0,
            };
        }

        let max_level = if data.len() == 1 {
            1
        } else {
            (data.len() as f64).log(degree as f64).ceil() as u32
        };
        let arr_size = degree.pow(max_level);
        let mut body: Vec<Option<Node<T>>> = vec![None; arr_size * degree];

        body[0] = Some(Node::Intermediate);

        GenericTree::build_recursive(
            data,
            (0, data.len() - 1),
            &mut body,
            0,
            arr_size / degree,
            degree,
        );

        GenericTree {
            degree: degree,
            length: data.len(),
            body: body,
        }
    }

    pub fn degree(&self) -> usize {
        self.degree
    }
    pub fn len(&self) -> usize {
        self.length
    }
    pub fn get(&self, index: usize) -> Option<Node<T>> {
        let max_level = (index as f64).log(self.degree as f64).ceil() as u32;
        let estimate_arr_size = self.degree.pow(max_level);
        if estimate_arr_size < self.len() {
            self.find_recursive(
                index,
                (0, self.len() - 1),
                0,
                self.body.len() / self.degree(),
            )
        } else {
            None
        }
    }

    fn find_recursive(
        &self,
        index: usize,
        range: (usize, usize),
        cursor: usize,
        window_size: usize,
    ) -> Option<Node<T>> {
        if range.0 == range.1 {
            return self.get(cursor);
        }

        if range.1 - range.0 + 1 < window_size {
            let diff = index - range.1;
            return self.get(cursor + diff);
        }

        let windows = (range.0..range.1)
            .step_by(window_size)
            .map(|v| (v, (v + window_size - 1).min(self.len() - 1)));

        for (i, (s, e)) in windows.enumerate() {
            if (s..e).contains(&i) {
                return self.find_recursive(
                    index,
                    (s, e),
                    cursor * self.degree + i,
                    window_size / self.degree,
                );
            }
        }

        None
    }
    fn build_recursive(
        data: &Vec<T>,
        range: (usize, usize),
        arr: &mut Vec<Option<Node<T>>>,
        cursor: usize,
        window_size: usize,
        degree: usize,
    ) -> () {
        // range.0 == range.1
        if window_size == 0 {
            arr[cursor] = if let Some(value) = data.get(range.0) {
                Some(Node::Terminal(*value))
            } else {
                None
            };
            return;
        }

        arr[cursor] = Some(Node::Intermediate);

        let windows = (range.0..range.1 + 1)
            .step_by(window_size)
            .map(|v| (v, (v + window_size - 1).min(data.len() - 1)))
            .filter(|(s, e)| range.0 <= *s && *e <= range.1);

        for (i, (s, e)) in windows.clone().enumerate() {
            println!("{} {} {}", i, s, e);
        }
        for (i, (s, e)) in windows.enumerate() {
            if e - s + 1 < window_size {
                for p in (s - s)..(e - s) {
                    arr[cursor + i + p] = Some(Node::Terminal(data[s + p]));
                }
            } else {
                GenericTree::build_recursive(
                    data,
                    (s, e),
                    arr,
                    cursor * degree + i + 1,
                    window_size / degree,
                    degree,
                )
            }
        }
    }

    pub fn print(&self) -> () {
        self.print_recursive(0, 0);
    }
    fn print_recursive(&self, now: usize, level: usize) -> () {
        match self.body.get(now) {
            Some(Some(Node::Intermediate)) => {
                for i in 1..self.degree {
                    self.print_recursive(now * self.degree + i, level + 1);
                }
            }
            Some(Some(Node::Terminal(value))) => {
                for _ in 0..level {
                    print!("--");
                }

                println!("{}", value);
            }
            Some(None) => (),
            None => (),
        }
    }
}
