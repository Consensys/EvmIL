use std::ops::Range;

/// Basically the same as `std::ops::Range`, but implements `Copy`.
/// Note, like `Range`, this is _half open_.  That means `start`
/// identifies the first index in the region, whilst `end` is one past
/// the last index.
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Region {
    pub start: usize,
    pub end: usize
}

impl Region {
    pub fn new(start: usize, end: usize) -> Self {
        Self {start,end}
    }
    /// Determine the number of items this region covers.
    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

/// Simple mechanism for constructing a `Region` from a `Range`.
impl From<Range<usize>> for Region {
    fn from(r: Range<usize>) -> Region {
       Region{start:r.start,end:r.end}
    }
}

// =================================================================
// Token
// =================================================================

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct Span<T>
where T:Clone+Copy+PartialEq {
    /// Type of the token
    pub kind : T,
    /// Identifies the (half open) region in the sequence.
    pub region: Region
}

impl<T> Span<T>
where T:Clone+Copy+PartialEq {

    pub fn new(kind: T, range: Range<usize>) -> Self {
        Self { kind, region: Region::from(range) }
    }

    /// Get first index of this token.
    pub fn start(&self) -> usize {
	self.region.start
    }

    /// Get end of this token (that is one past its last character).
    pub fn end(&self) -> usize {
	self.region.end
    }

    /// Get the length (in chars) of this token.
    pub fn len(&self) -> usize {
        self.region.end - self.region.start
    }
}

// =================================================================
// Rule
// =================================================================

pub trait Tokenizer {
    /// Identifies the token type produced by this tokenizer.
    type Token:Clone+Copy+PartialEq;
    /// Responsible for producing a token from a given position in the
    /// input.
    fn scan(&self, offset: usize, input: &[char]) -> Span<Self::Token>;
}

// =================================================================
// Lexer
// =================================================================

/// Provides machinery for splitting up a string slice into a sequence
/// of tokens.
pub struct Lexer<T:Tokenizer> {
    /// Character sequence being tokenised
    chars: Vec<char>,
    /// Current position in character sequence
    offset: usize,
    /// Responsible for dividing characters into tokens
    tokeniser: T
}

/// An acceptor determines whether or not a character is part of a
/// given token.
type Acceptor = fn(char)->bool;

/// An acceptor determines whether or not a pair of characters is matched.
type Acceptor2 = fn(char,char)->bool;

impl<T:Tokenizer> Lexer<T> {
    /// Construct a new lexer for a given string slice.
    pub fn new(input: &str, tokeniser: T) -> Self {
        // Extract character sequence
        let chars = input.chars().collect();
        // Construct lexer
        return Self { chars, offset: 0, tokeniser }
    }

    /// Check whether the lexer is at the end of file.
    pub fn is_eof(&self) -> bool {
        self.offset >= self.chars.len()
    }

    /// Peek at the next token in the sequence, or none if we have
    /// reached the end.
    pub fn peek(&self) -> Span<T::Token> { self.scan(self.offset) }

    /// Get the next token in the sequence, or none if we have reached
    /// the end.
    pub fn next(&mut self) -> Span<T::Token> {
        let t = self.scan(self.offset);
        self.offset = t.end();
        t
    }

    /// Begin process of scanning a token based on its first
    /// character.  The actual work is offloaded to a helper based on
    /// this.
    fn scan(&self, start: usize) -> Span<T::Token> {
        let ch = self.chars[start];
        todo!("got here");
    }
}
