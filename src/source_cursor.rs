#[derive(Debug, Clone)]
pub struct SourceCodeCursor {
    contents: Vec<char>,
    index: usize,
    curr_line: usize,
    curr_col: usize,
}

impl SourceCodeCursor {
    pub fn new(contents: String) -> Self {
        SourceCodeCursor {
            contents: contents.chars().collect(),
            index: 0,
            curr_line: 1,
            curr_col: 0,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.contents.get(self.index).copied()
    }
    pub fn peek_nth(&self, n: usize) -> Option<char> {
        // peek_nth(1) is equivalent to peek()
        self.contents.get(self.index + n - 1).copied()
    }

    pub fn begins_with(&self, s: &str) -> bool {
        let mut index: usize = 1;
        for char in s.chars() {
            if self.peek_nth(index) != Some(char) {
                return false;
            }
            index += 1;
        }
        return true;
    }

    pub fn next(&mut self) -> Option<char> {
        self.index += 1;
        let result = self.contents.get(self.index - 1);
        match result {
            None => None,
            Some(char) => {
                if char == &'\n' {
                    self.curr_line += 1;
                    self.curr_col = 0;
                } else {
                    self.curr_col += 1;
                }

                Some(*char)
            }
        }
    }
}
