pub struct Parser {
    content: String,
    idx: usize,
}

impl Parser {
    pub fn new<S: Into<String>>(content: S) -> Parser {
        Parser {
            content: content.into(),
            idx: 0,
        }
    }

    /// Check if it's the end of file
    pub fn is_eof(&self) -> bool {
        self.idx >= self.content.len()
    }

    /// Return the next char without changing the index
    pub fn next(&self) -> char {
        self.content[self.idx..].chars().next().unwrap()
    }

    /// Return the next char and move the char by one
    pub fn consume(&mut self) -> char {
        let mut indices = self.content[self.idx..].char_indices();
        let (_, c) = indices.next().unwrap();
        let (next_idx, _) = indices.next().unwrap_or((1, ' '));
        self.idx += next_idx;
        c
    }

    /// Consume content while function is true and it's not end of file
    pub fn consume_while<F>(&mut self, func: F) -> String
        where F: Fn(char) -> bool
    {
        let mut result = String::new();
        while !self.is_eof() && func(self.next()) {
            let c = self.consume();
            result.push(c);
        }
        result
    }
}
