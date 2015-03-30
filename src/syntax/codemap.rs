//! Maps spans to source code

use std::mem;
use std::ops::Index;

/// Byte position
pub type BytePos = usize;

/// Source code
pub struct Source(str);

impl Source {
    /// Treats the input string as source code
    pub fn new(input: &str) -> &Source {
        unsafe {
            mem::transmute(input)
        }
    }

    /// Views the source code as a string slice
    pub fn as_str(&self) -> &str {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl Index<Span> for Source {
    type Output = str;

    fn index(&self, span: Span) -> &str {
        &self.0[span.lo..span.hi]
    }
}

/// Source code span
#[derive(Copy, Debug)]
pub struct Span {
    /// Start of the span
    pub lo: BytePos,
    /// End of the span
    pub hi: BytePos,
}

impl Span {
    /// Creates a new span from `lo` to `hi`
    pub fn new(lo: BytePos, hi: BytePos) -> Span {
        Span {
            lo: lo,
            hi: hi,
        }
    }
}

impl Span {
    /// The "dummy" span, should never be used for indexing
    pub fn dummy() -> Span {
        Span {
            lo: 0,
            hi: 0,
        }
    }
}

/// A spanned node
#[derive(Copy, Debug)]
pub struct Spanned<T> {
    /// The node
    pub node: T,
    /// The span
    pub span: Span,
}

impl<A> Spanned<A> {
    /// Creates a spanned node
    pub fn new(lo: BytePos, hi: BytePos, node: A) -> Spanned<A> {
        Spanned {
            span: Span::new(lo, hi),
            node: node,
        }
    }

    /// Maps the spanned node while retaining the span
    pub fn map<F>(self, f: F) -> Spanned<F::Output> where F: Fn<(A,)> {
        Spanned {
            span: self.span,
            node: f(self.node)
        }
    }
}
