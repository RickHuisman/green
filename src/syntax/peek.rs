pub struct PeekWithNext<I: Iterator> {
    iter: I,
    items: (Option<I::Item>, Option<I::Item>),
}

impl<I: Iterator> PeekWithNext<I> {
    pub fn new(iter: I) -> Self {
        PeekWithNext {
            iter,
            items: (None, None),
        }
    }

    pub fn peek(&mut self) -> Option<&I::Item> {
        self.fill();
        self.items.0.as_ref()
    }

    pub fn peek_next(&mut self) -> Option<&I::Item> {
        self.fill();
        self.items.1.as_ref()
    }

    fn fill(&mut self) {
        if self.items.0.is_none() {
            self.items = (self.iter.next(), self.iter.next());
        }
    }
}

impl<I> Iterator for PeekWithNext<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.fill();
        let next = self.items.0.take();
        self.items.0 = self.items.1.take();
        self.items.1 = self.iter.next();
        next
    }
}
