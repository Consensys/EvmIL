use std::ops::Range;

// =================================================================
// Errors
// =================================================================

/// Indicates that a particular kind of token was expected, but that
/// we actually found something else.
pub type SnapError<T> = (T,Span<T>);

/// The type of error which can be returned from `snap()`.
pub type SnapResult<T> = Result<Span<T>,SnapError<T>>;

// =================================================================
// Region
// =================================================================

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

    pub fn shift(&mut self, delta: usize) {
        self.start += delta;
        self.end += delta;
    }
}

/// Simple mechanism for constructing a `Region` from a `Range`.
impl From<Range<usize>> for Region {
    fn from(r: Range<usize>) -> Region {
       Region{start:r.start,end:r.end}
    }
}

impl Into<Range<usize>> for Region {
    fn into(self) -> Range<usize> { self.start .. self.end }
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

    /// Extract the underlying region covered by this span as a
    /// `Range`.  This is really just for convenience.
    pub fn range(&self) -> Range<usize> { self.start() .. self.end() }

    /// Shift the span to a different position in the underlying
    /// sequence.  The position is taken as a delta from the current
    /// position (e.g. `delta==1` means we shift one up the sequence).
    pub fn shift(&mut self, delta: usize) {
        self.region.shift(delta);
    }
}

// =================================================================
// Tokenizer
// =================================================================

/// Provides a generic description of something which splits items in
/// the input sequence up into tokens.
pub trait Tokenizer {
    /// Identifies items in the underlying sequence being tokenized.
    type Item;
    /// Identifies the token type produced by this tokenizer.
    type Token:Clone+Copy+PartialEq;
    /// Responsible for producing a token from a given position in the
    /// input.
    fn scan(&self, input: &[Self::Item]) -> Span<Self::Token>;
}

// =================================================================
// Lexer
// =================================================================

/// Provides machinery for splitting up an _underlying sequence_ of
/// items into a sequence of tokens, where each token can correspond
/// to one or more items in the underlying sequence.
pub struct Lexer<T:Tokenizer> {
    /// Underlying sequence being tokenised
    input: Vec<T::Item>,
    /// Current position in character sequence
    offset: usize,
    /// Responsible for dividing characters into tokens
    tokeniser: T
}

impl<T:Tokenizer> Lexer<T> {
    /// Construct a new lexer for a given string slice.
    pub fn new(input: Vec<T::Item>, tokeniser: T) -> Self {
        // Construct lexer
        return Self { input, offset: 0, tokeniser }
    }

    /// Get the slice which corresponds to a given span from the
    /// underlying sequence.
    pub fn get(&self, span: Span<T::Token>) -> &[T::Item] {
        &self.input[span.range()]
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

    /// Match a given token type in the current stream.  If the kind
    /// matches, then the token stream advances.  Otherwise, it
    /// remains at the same position and an error is returned.
    pub fn snap(&mut self, kind : T::Token) -> SnapResult<T::Token> {
	// Peek at the next token
	let lookahead = self.peek();
	// Check it!
	if lookahead.kind == kind {
	    // Accept it
	    self.next();
	    //
	    Ok(lookahead)
	} else {
	    // Reject
	    Err((kind,lookahead))
	}
    }

    /// Begin process of scanning a token based on its first
    /// character.  The actual work is offloaded to a helper based on
    /// this.
    fn scan(&self, start: usize) -> Span<T::Token> {
        // Scan next token
        let mut span = self.tokeniser.scan(&self.input[start..]);
        // Shift to correct position
        span.shift(start);
        // Done
        span
    }
}

// =================================================================
// Table Tokenizer
// =================================================================

/// Defines a very simple concept of a scanner which requires no
/// state.  Tokenizers can be built out of scanners, for example.
pub type Scanner<S,T> = fn(&[S])->Result<Span<T>,()>;

/// A tokenizer construct from one or more tokenizers which are tried
/// in order of appearance.
pub struct TableTokenizer<S,T>
where T: Copy+Clone+PartialEq {
    /// The table of tokenizers to use for scanning.
    table : Vec<Scanner<S,T>>
}

impl<S,T> TableTokenizer<S,T>
where T: Copy+Clone+PartialEq {
    /// Construct a new tokenizer from a given table.
    pub fn new(table: Vec<Scanner<S,T>>) -> Self {
        Self{table}
    }
}

impl<S,T> Tokenizer for TableTokenizer<S,T>
where T: Copy+Clone+PartialEq {
    type Item = S;
    type Token = T;

    fn scan(&self, input: &[Self::Item]) -> Span<Self::Token> {
        for s in &self.table {
            match s(input) {
                Ok(s) => { return s; }
                _ => {}
            }
        }
        panic!("PROBLEM");
    }
}
