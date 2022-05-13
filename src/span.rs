use std::{
    borrow::BorrowMut, cell::RefCell, fmt::Display, marker::PhantomData, ops::Deref, ptr::NonNull,
    rc::Rc, sync::RwLock,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct FileIndex {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
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

    pub fn display<'a>(self, src: &'a str) -> DisplaySpan<'a> {
        DisplaySpan { span: self, src }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplaySpan<'a> {
    span: Span,
    src: &'a str,
}

impl<'a> Display for DisplaySpan<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Add support for multiple lines.
        assert_eq!(self.span.0.line, self.span.1.line);
        let line = self.src.lines().nth(self.span.0.line).unwrap();
        let line_number_prefix = format!("{} | ", self.span.0.line + 1);
        let arrows = format!(
            "{}{}",
            " ".repeat(self.span.0.column + line_number_prefix.len()),
            "^".repeat(self.span.1.column - self.span.0.column)
        );
        write!(f, "[{}:{}]\n{}{}\n{}", self.span.0.line + 1, self.span.0.line + 1, line_number_prefix, line, arrows)
    }
}

#[derive(Debug)]
pub struct SpanStack {
    internal: Rc<RwLock<Vec<*mut Span>>>,
}

impl SpanStack {
    pub fn new() -> SpanStack {
        SpanStack {
            internal: Rc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn push(&self, mut span: Span) -> SpanGuard {
        let frame = self.internal.read().unwrap().len();
        (*self.internal).write().unwrap().push(&mut span as *mut _);
        SpanGuard {
            stack: Rc::clone(&self.internal),
            inner: span,
            frame,
        }
    }

    pub fn internal_stack_mut(&mut self) -> std::sync::RwLockWriteGuard<Vec<*mut Span>> {
        self.internal.write().unwrap()
    }

    pub fn internal_stack(&self) -> std::sync::RwLockReadGuard<Vec<*mut Span>> {
        self.internal.read().unwrap()
    }
}

pub struct SpanGuard {
    stack: Rc<RwLock<Vec<*mut Span>>>,
    inner: Span,
    frame: usize,
}

impl Drop for SpanGuard {
    fn drop(&mut self) {
        debug_assert_eq!(self.stack.read().unwrap().len(), self.frame + 1, "Invalid drop order. Child spans need to be dropped before the parent because this is a stack.");
        (*self.stack).write().unwrap().pop();
    }
}

impl Deref for SpanGuard {
    type Target = Span;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
