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
    pub fn next(&self) -> Option<char> {
        self.content[self.idx..].chars().next()
    }

    /// Return the next char and move the char by one
    pub fn consume(&mut self) -> Option<char> {
        let mut indices = self.content[self.idx..].char_indices();
        let c = match indices.next() {
            Some((_, c)) => Some(c),
            _ => None,
        };
        let (next_idx, _) = indices.next().unwrap_or((1, ' '));
        self.idx += next_idx;
        c
    }

    /// Consume content while function is true and it's not end of file
    pub fn consume_while<F>(&mut self, func: F) -> String
        where F: Fn(char) -> bool
    {
        let mut result = String::new();
        while !self.is_eof() && func(self.next().unwrap()) {
            let c = match self.consume() {
                Some(c) => c,
                None => return result,
            };
            result.push(c);
        }
        result
    }

    /// Consume whitespaces
    pub fn consume_whitespace(&mut self) -> String {
        self.consume_while(|c| c.is_whitespace())
    }


    /// Consume until the next char is the specified char
    pub fn consume_until(&mut self, c: char) -> String {
        self.consume_while(|nc| nc != c)
    }

    /// Consume until the content match the specified str
    pub fn consume_until_str(&mut self, s: &str) -> String {
        let mut result = String::new();
        while !self.is_eof() && !self.starts_with(s) {
            let c = match self.consume() {
                Some(c) => c,
                None => return result,
            };
            result.push(c);
        }
        result
    }

    /// Consume until end of file
    pub fn consume_until_end(&mut self) -> String {
        let mut result = String::new();
        while !self.is_eof() {
            result.push(self.consume().unwrap());
        }
        result
    }

    /// Consume inside the same two char, specified in arg
    pub fn consume_inside(&mut self, c: char) -> String {
        if self.next() != Some(c) && self.content[self.idx + 1..].contains("\"") {
            return String::new();
        }
        self.consume();
        let result = self.consume_until(c);
        self.consume();
        result
    }
}
