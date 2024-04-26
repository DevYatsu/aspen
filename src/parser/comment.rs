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
