pub struct BefAftWindowIterator<'a, T> {
    cursor: usize,
    started: bool,
    list: &'a Vec<T>,
}

impl<'a, T> Iterator for BefAftWindowIterator<'a, T> {
    type Item = (Option<&'a T>, &'a T, Option<&'a T>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.started {
            self.cursor += 1;
        } else {
            self.started = true;
        }
        Some((
            if self.cursor == 0 {
                None
            } else {
                self.list.get(self.cursor - 1)
            },
            self.list.get(self.cursor)?,
            self.list.get(self.cursor + 1),
        ))
    }
}

impl<'a, T> BefAftWindowIterator<'a, T> {
    #[must_use]
    pub const fn new(list: &'a Vec<T>) -> Self {
        Self {
            cursor: 0,
            started: false,
            list,
        }
    }
}
