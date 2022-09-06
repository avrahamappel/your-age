#[derive(Debug)]
struct IntersperseChunks<I: Iterator> {
    inner: I,
    buffer: Option<I::Item>,
    separator: I::Item,
    count: usize,
    chunk_length: usize,
}

impl<I: Iterator> IntersperseChunks<I> {
    fn new(inner: I, separator: I::Item, chunk_length: usize) -> Self {
        Self {
            inner,
            buffer: None,
            separator,
            chunk_length,
            count: 0,
        }
    }
}

impl<I> Iterator for IntersperseChunks<I>
where
    <I as Iterator>::Item: Clone + std::fmt::Debug,
    I: Iterator + std::fmt::Debug,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = &self.buffer {
            let item = item.clone();
            self.buffer = None;
            self.count += 1;
            return Some(item);
        }

        let item = self.inner.next();

        if self.count == self.chunk_length && item.is_some() {
            self.count = 0;
            self.buffer = item;
            return Some(self.separator.clone());
        }

        self.count += 1;
        item
    }
}

trait ToIntersperseChunks
where
    Self: Sized + Iterator,
{
    fn intersperse_chunks(self, item: Self::Item, chunk_length: usize) -> IntersperseChunks<Self> {
        IntersperseChunks::new(self, item, chunk_length)
    }
}

impl<I> ToIntersperseChunks for I where I: Iterator {}

/// Insert separators in a number string.
fn insert_separators(num_str: String) -> String {
    num_str
        .chars()
        .rev()
        .intersperse_chunks(',', 3)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}

pub trait WithSeparators {
    fn with_separators(self) -> String
    where
        Self: Sized;
}

impl WithSeparators for String {
    fn with_separators(self) -> Self {
        insert_separators(self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn insert_separators() {
        let strings = [
            ("30", "30"),
            ("374", "374"),
            ("11232", "11,232"),
            ("269568", "269,568"),
            ("16174080", "16,174,080"),
            ("970444800", "970,444,800"),
        ];

        for (input, output) in strings {
            assert_eq!(output, super::insert_separators(input.into()));
        }
    }
}
