#[derive(Clone, Debug)]
pub struct BitSet {
    buf: Vec<bool>,
    size: usize,
}

impl BitSet {
    pub fn new(size: usize, value: u32) -> BitSet {
        let mut buf = vec![false; size];
        let mut value = value;
        for i in 0..size {
            if value % 2 == 1 {
                buf[i] = true;
            }
            value = value >> 1;
        }
        BitSet {
            buf: buf,
            size: size,
        }
    }

    pub fn as_integer(&self) -> u32 {
        let mut value = 0;
        for i in (0..self.size).rev() {
            value = value << 1;
            if self.buf[i] {
                value += 1;
            }
        }
        value
    }
}

impl std::ops::Index<usize> for BitSet {
    type Output = bool;
    fn index(&self, index: usize) -> &bool {
        &self.buf[index]
    }
}

fn main() {
    let bs = BitSet::new(16, 7);
    println!("{:?}", (bs[0], bs[1], bs[2], bs[3]));
    println!("{:?}", bs.as_integer());
}
