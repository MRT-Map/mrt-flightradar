pub struct BefAftWindowIterator<'a, T> {
    cursor: usize,
    list: &'a Vec<T>,
}

impl<'a, T> Iterator for BefAftWindowIterator<'a, T> {
    type Item = (Option<&'a T>, &'a T, Option<&'a T>);
    fn next(&mut self) -> Option<Self::Item> {
        self.cursor += 1;
        Some((
            self.list.get(self.cursor - 1),
            self.list.get(self.cursor)?,
            self.list.get(self.cursor + 1),
        ))
    }
}

impl<'a, T> BefAftWindowIterator<'a, T> {
    pub fn new(list: &'a Vec<T>) -> Self {
        Self { cursor: 0, list }
    }
}
