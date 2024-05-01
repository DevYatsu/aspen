use std::hash::Hash;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Comment<'s> {
    pub start: usize,
    pub end: usize,
    pub value: &'s str,
}

impl<'s> Comment<'s> {
    pub fn new(value: &'s str, start: usize, end: usize) -> Self {
        Self { value, start, end }
    }
}

impl<'s> std::fmt::Display for Comment<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'s> Deref for Comment<'s> {
    type Target = Comment<'s>;

    fn deref(&self) -> &Self::Target {
        self
    }
}
