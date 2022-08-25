/// Insert separators in a number string.
fn insert_separators(num_str: String) -> String {
    let rev = num_str.chars().rev().collect::<Vec<_>>();
    let chunks = rev.chunks(3).collect::<Vec<_>>();
    chunks.join([','].as_slice()).iter().rev().collect()
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
