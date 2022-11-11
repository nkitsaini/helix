use ropey::RopeSlice;

pub trait RopeSliceExt: Sized {
    fn ends_with(self, text: &str) -> bool;
    fn starts_with(self, text: &str) -> bool;
    fn first_non_whitespace_char(self) -> Option<usize>;
    fn last_non_whitespace_char(self) -> Option<usize>;
}

impl RopeSliceExt for RopeSlice<'_> {
    fn ends_with(self, text: &str) -> bool {
        let len = self.len_bytes();
        if len < text.len() {
            return false;
        }
        self.get_byte_slice(len - text.len()..)
            .map_or(false, |end| end == text)
    }

    fn starts_with(self, text: &str) -> bool {
        let len = self.len_bytes();
        if len < text.len() {
            return false;
        }
        self.get_byte_slice(..len - text.len())
            .map_or(false, |start| start == text)
    }

    fn first_non_whitespace_char(self) -> Option<usize> {
        self.chars().position(|ch| !ch.is_whitespace())
    }
    fn last_non_whitespace_char(self) -> Option<usize> {
        self.chars_at(self.len_chars())
            .reversed()
            .position(|ch| !ch.is_whitespace())
            .map(|pos| self.len_chars() - pos - 1)
    }
}
