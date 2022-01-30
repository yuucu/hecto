use std::cmp;

pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let mut row = Self {
            string: String::from(slice),
            len: 0,
        };
        row.update_len();
        row
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        self.string.get(start..end).unwrap_or_default().to_string()
    }
    pub fn len(&self) -> usize {
        self.string.len()
    }
    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }
    fn update_len(&mut self) {
        self.len = self.string.len();
    }
}
