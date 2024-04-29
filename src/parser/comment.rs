use std::hash::Hash;
use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Comment<'a> {
    start: usize,
    end: usize,
    value: &'a str,
}

impl<'a> Comment<'a> {
    pub fn new(value: &'a str, start: usize, end: usize) -> Self {
        Self { value, start, end }
    }
}

impl<'a> std::fmt::Display for Comment<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'a> Deref for Comment<'a> {
    type Target = Comment<'a>;

    fn deref(&self) -> &Self::Target {
        self
    }
}
