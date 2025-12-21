use std::fmt::Display;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    pub beg: usize,

    pub len: usize,
}

impl Span {
    pub fn new(beg: usize, len: usize) -> Self {
        Self { beg, len }
    }

    pub fn beg(&self) -> usize {
        self.beg
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn add_assign(&mut self, other: Self) -> &mut Self {
        self.len += other.len + other.beg - (self.beg + self.len);
        self
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{beg: {}, len: {}}}", self.beg, self.len)
    }
}
