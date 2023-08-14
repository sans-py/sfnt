use std::str;

#[derive(Hash, PartialEq, Eq)]
struct Tag {
    pub value: String
}

impl Tag {
    pub fn from(content: &[u8]) -> Self {
        Tag {
            value: str::from_utf8(content).unwrap().to_string()
        }
    }
}

pub trait Pad {
    fn pad(&self, size: u8) -> Self;
}

impl Pad for Vec<u8> {
    fn pad(&self, window_size: u8) -> Self {
        if window_size == 0 {
            self.to_vec()
        } else {
            let remainder = (self.len() as u8) % window_size;
            let mut vector = self.to_vec();

            for _ in 0..remainder {
                vector.push(0)
            }
            vector
        }
    }
}

pub trait ReadAsBinary {
    type Destination;
    fn read_as_binary(&self) -> Self::Destination;
}

/** TODO: 사용례에 따라 reverse를 붙여줘야 할수도 있음 */
impl ReadAsBinary for Vec<u8> {
    type Destination = i32;
    fn read_as_binary(&self) -> Self::Destination {
        self.iter().enumerate().map(|(index, value)|{
            if *value == b'1' {
                0x1 << index
            } else {
                0
            }
        }).sum()
    }
}

/** TODO: 사용례에 따라 큰 수부터 해야할수도 있음. */
impl ReadAsBinary for i32 {
    type Destination = Vec<u8>;
    fn read_as_binary(&self) -> Self::Destination {
        let mut v = Vec::new();
        
        for digits in 0..8 {
            if *self & (0x1 << digits) == 1 {
                v.push(b'1');
            } else {
                v.push(b'0');
            }
        }

        v
    }
}

pub trait CaselessSort {
    fn caseless_sort(&self) -> Self;
}
impl CaselessSort for Vec<String> {
    fn caseless_sort(&self) -> Self {
        let in_lowercase = self.iter().map(|s|{ s.to_lowercase() });
        let mut sorted = Vec::from_iter(in_lowercase);
        sorted.sort();
        sorted
    }
}