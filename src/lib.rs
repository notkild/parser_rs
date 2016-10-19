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

    /// Check if the content starts with the specified str
    pub fn starts_with(&self, s: &str) -> bool {
        self.content[self.idx..].starts_with(s)
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

    /// Consume until the next char is the specified char
    pub fn consume_until(&mut self, c: char) -> String {
        self.consume_while(|nc| nc != c)
    }

    /// Consume until the content match the specified str
    pub fn consume_until_str(&mut self, s: &str) -> String {
        let mut result = String::new();
        while !self.is_eof() && !self.starts_with(s) {
            let c = self.consume();
            result.push(c);
        }
        result
    }
}
