use std::fmt;
use crate::Term;
use crate::lexer::{Lexer,Token};
use crate::lex::{Span,SnapError};

// =========================================================================
// Error
// =========================================================================

#[derive(Clone,Debug,PartialEq)]
pub enum ErrorCode {
    UnexpectedToken,
    UnexpectedEof,
    ExpectedToken(Token),
    ExpectedTokenIn(Vec<Token>)
}

/// Identifies possible errors stemming from the parser.
#[derive(Debug)]
pub struct Error {
    pub span: Span<Token>,
    pub code: ErrorCode
}

impl Error {
    pub fn new(span: Span<Token>, code: ErrorCode) -> Error {
	Error{span,code}
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // temporary for now.
        write!(f,"{:?}",self)
    }
}

impl std::error::Error for Error { }

impl From<SnapError<Token>> for Error {
    fn from(p:(Token,Span<Token>)) -> Error {
        Error{span:p.1,code:ErrorCode::ExpectedToken(p.0)}
    }
}

pub type Result<T> = std::result::Result<T, Error>;

// =========================================================================
// Parser
// =========================================================================

pub struct Parser {
    /// Provides access to our token stream.
    lexer: Lexer
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self { lexer: Lexer::new(input) }
    }

    /// Parse a line of text into a term.
    pub fn parse(&mut self) -> Result<Term> {
        self.parse_stmt()
    }

    // =========================================================================
    // Statements
    // =========================================================================

    fn parse_stmt(&mut self) -> Result<Term> {
    	let lookahead = self.lexer.peek();
    	//
    	let stmt = match lookahead.kind {
    	    Token::Assert => self.parse_stmt_assert(),
            _ => {
                // Unknown statement
                Err(Error::new(lookahead,ErrorCode::UnexpectedToken))
            }
        };
        //
        stmt
    }

    pub fn parse_stmt_assert(&mut self) -> Result<Term> {
    	self.lexer.snap(Token::Assert)?;
    	let expr = self.parse_expr()?;
        Ok(Term::Assert(Box::new(expr)))
    }

    // =========================================================================
    // Expressions
    // =========================================================================

    pub fn parse_expr(&mut self) -> Result<Term> {
        // Skip whitespace
        self.skip_whitespace();
        //
    	let lookahead = self.lexer.peek();
    	//
    	let expr = match lookahead.kind {
    	    Token::Integer => self.parse_literal_int()?,
    	    Token::LeftBrace => self.parse_expr_bracketed()?,
    	    _ => {
    		return Err(Error::new(lookahead,ErrorCode::UnexpectedToken));
    	    }
    	};
        // Done
        Ok(expr)
    }

    pub fn parse_expr_bracketed(&mut self) -> Result<Term> {
    	self.lexer.snap(Token::LeftBrace)?;
    	let expr = self.parse_expr();
    	self.lexer.snap(Token::RightBrace)?;
        expr
    }

    pub fn parse_literal_int(&mut self) -> Result<Term> {
        let tok = self.lexer.snap(Token::Integer)?;
        let x = self.lexer.get_int(tok);
        // FIXME: this is not ideal :)
        let b1 : u8 = ((x >> 24) & 0xff) as u8;
        let b2 : u8 = ((x >> 16) & 0xff) as u8;
        let b3 : u8 = ((x >> 8) & 0xff) as u8;
        let b4 : u8 = (x & 0xff) as u8;
        let bytes = [b1, b2, b3, b4];
        //
        Ok(Term::Int(bytes.to_vec()))
    }

    // =========================================================================
    // Helpers
    // =========================================================================

    fn skip_whitespace(&mut self) {
        let lookahead = self.lexer.peek();
        //
        match lookahead.kind {
            Token::Gap => {
                self.lexer.snap(lookahead.kind).unwrap();
                self.skip_whitespace()
            }
            _ => {
                // Do nothing!
            }
        }
    }
}
