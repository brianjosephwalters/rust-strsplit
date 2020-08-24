//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

pub struct StrSplit {
    remainder: &str,
    delimiter: &str,
}

impl StrSplit {
    pub fn new(haystack: &str, delimiter: &str) -> Self {
        Self {
            remainder: haystack,
            delimiter, // don't need field:value because same name
        }
    }
}

impl Iterator for StrSplit {
    type Item = &str;
    fn next(&mut self) -> Optional<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        }
        // either remainder is empty -> None
        else if self.remainder.is_empty() {
            // TODO: Bug
            None
        }
        // remainder is not empty -> remainder
        else {
            let rest = self.remainder;
            self.remainder = &[];
            Some(rest)
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    for leter in StrSplit::new(haystack, " ");
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"].into_iter());
}