use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RichIndex {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub begin: RichIndex,
    pub end: RichIndex,
}

impl Span {
    pub fn notice(&mut self, c: char) {
        self.end.index += 1;
        self.end.column += 1;
        if c == '\n' {
            self.end.line += 1;
            self.end.column = 0;
        }
    }

    pub fn tp_end(&mut self) {
        self.begin = self.end;
    }

    pub fn into_rich<'a>(self, src: &'a str) -> RichSpan<'a> {
        RichSpan { span: self, src }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RichSpan<'a> {
    span: Span,
    src: &'a str,
}

impl<'a> Display for RichSpan<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Add support for multiple lines.
        assert_eq!(self.span.begin.line, self.span.end.line);
        let line = self.src.lines().nth(self.span.begin.line).unwrap();
        println!("{}", self.span.end.column - self.span.begin.column);
        let arrows = format!(
            "{}{}",
            " ".repeat(self.span.begin.column),
            "^".repeat(self.span.end.column - self.span.begin.column)
        );
        write!(f, "\n{}\n{}", line, arrows)
    }
}
