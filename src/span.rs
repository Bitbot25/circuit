#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct FileIndex {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Copy, Debug, Default, Clone, PartialEq, Eq)]
pub struct Span(pub FileIndex, pub FileIndex);

impl Span {
    pub fn notice(&mut self, c: char) {
        self.1.index += 1;
        self.1.column += 1;
        if c == '\n' {
            self.1.line += 1;
            self.1.column = 0;
        }
    }

    pub fn extend(&mut self, other: &Span) {
        self.1 = other.1;
    }

    pub fn blip(&mut self) {
        self.0 = self.1;
    }

    /*pub fn display<'a>(self, src: &'a str) -> DisplaySpan<'a> {
        DisplaySpan { span: self, src }
    }*/
}

// FIXME: Move this to a new ParseError.

/*#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplaySpan<'a> {
    span: Span,
    src: &'a str,
}

impl<'a> Display for DisplaySpan<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Add support for multiple lines.
        assert_eq!(self.span.0.line, self.span.1.line);
        let line = self
            .src
            .lines()
            .nth(self.span.0.line)
            .unwrap_or_else(|| panic!("Line number {} doesn't exist.", self.span.0.line));
        let line_number_prefix = format!("{} | ", self.span.0.line + 1);
        let arrows = format!(
            "{}{}",
            " ".repeat(self.span.0.column + line_number_prefix.len()),
            "^".repeat(self.span.1.column - self.span.0.column)
        );
        write!(
            f,
            "[{}:{}]\n{}{}\n{}",
            self.span.0.line + 1,
            self.span.0.line + 1,
            line_number_prefix,
            line,
            arrows
        )
    }
}*/
