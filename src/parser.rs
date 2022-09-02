use std::fmt;
use crate::{BinOp,Term};
use crate::lexer::{Lexer,Token};
use crate::lex::{Span,SnapError};

/// Defines the set of tokens which are considered to identify logical
/// connectives (e.g. `&&`, `||`, etc).
pub const LOGICAL_CONNECTIVES : &'static [Token] = &[
    Token::AmpersandAmpersand,
    Token::BarBar
];

/// Defines the set of tokens which are considered to identify
/// arithmetic comparators (e.g. `<`, `<=`, `==`, etc).
pub const ARITHMETIC_COMPARATORS : &'static [Token] = &[
    Token::EqualsEquals,
    Token::ShreakEquals,
    Token::LeftAngle,
    Token::LeftAngleEquals,
    Token::RightAngle,
    Token::RightAngleEquals
];

/// Defines the set of tokens which are considered to identify
/// arithmetic operators (e.g. `+`, `-`, `*`, etc).
pub const ARITHMETIC_OPERATORS : &'static [Token] = &[
    Token::Minus,
    Token::Percent,
    Token::Plus,
    Token::RightSlash,
    Token::Star
];

pub const BINARY_CONNECTIVES : &'static [ &'static [Token] ] = &[
    &ARITHMETIC_OPERATORS,
    &ARITHMETIC_COMPARATORS,
    &LOGICAL_CONNECTIVES,
];

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
    fn from(p:SnapError<Token>) -> Error {
        match p {
            SnapError::Expected(t,s) => {
                Error{span:s,code:ErrorCode::ExpectedToken(t)}
            }
            SnapError::ExpectedIn(ts,s) => {
                Error{span:s,code:ErrorCode::ExpectedTokenIn(ts)}
            }
        }
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
        self.lexer.snap(Token::SemiColon)?;
        Ok(Term::Assert(Box::new(expr)))
    }

    // =========================================================================
    // Expressions
    // =========================================================================

    pub fn parse_expr(&mut self) -> Result<Term> {
        self.parse_expr_binary(3)
    }

    /// Parse a binary expression at a given _level_.  Higher levels
    /// indicate expressions which bind _less tightly_.  Furthermore,
    /// level `0` corresponds simply to parsing a unary expression.
    pub fn parse_expr_binary(&mut self, level: usize) -> Result<Term> {
        if level == 0 {
            self.parse_expr_term()
        } else {
            let tokens = BINARY_CONNECTIVES[level-1];
            // Parse level below
    	    let lhs = self.parse_expr_binary(level-1)?;
            // Skip remaining whitespace (on this line)
            self.skip_whitespace();
	    // Check whether logical connective follows
    	    let lookahead = self.lexer.snap_any(tokens);
            //
            match lookahead {
                Ok(s) => {
                    // FIXME: turn this into a loop?
	            let rhs = self.parse_expr_binary(level)?;
                    let bop = Self::binop_from_token(s.kind).unwrap();
	            Ok(Term::Binary(bop,Box::new(lhs),Box::new(rhs)))
                }
                Err(_) => {
                    Ok(lhs)
                }
            }
        }
    }

    pub fn parse_expr_term(&mut self) -> Result<Term> {
        // Skip whitespace
        self.skip_whitespace();
        //
    	let lookahead = self.lexer.peek();
    	//
    	let expr = match lookahead.kind {
    	    Token::Integer => self.parse_literal_int()?,
    	    Token::Hex => self.parse_literal_hex()?,
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
        // Extract characters making up literal
        let chars = self.lexer.get_str(tok);
        // Convert characters into digits
        let digits = chars.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        // All good!
        Ok(Term::Int(digits))
    }

    pub fn parse_literal_hex(&mut self) -> Result<Term> {
        let tok = self.lexer.snap(Token::Hex)?;
        // Extract characters making up literal
        let chars = &self.lexer.get_str(tok)[2..];
        // Convert characters into digits
        let digits = chars.chars().map(|c| c.to_digit(16).unwrap() as u8).collect();
        // All good!
        Ok(Term::Hex(digits))
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

    fn binop_from_token(token: Token) -> Option<BinOp> {
	let bop = match token {
            // // Equality
            Token::EqualsEquals => BinOp::Equals,
            Token::ShreakEquals => BinOp::NotEquals,
            // // Comparison
	    Token::LeftAngle => BinOp::LessThan,
            Token::LeftAngleEquals => BinOp::LessThanOrEquals,
            Token::RightAngle => BinOp::GreaterThan,
            Token::RightAngleEquals => BinOp::GreaterThanOrEquals,
            // Arithmetic
            Token::Minus => BinOp::Subtract,
	    Token::Percent => BinOp::Remainder,
	    Token::Plus => BinOp::Add,
            Token::RightSlash => BinOp::Divide,
            Token::Star => BinOp::Multiply,
            // // Logical
            Token::AmpersandAmpersand => BinOp::LogicalAnd,
            Token::BarBar => BinOp::LogicalOr,
            // No match
	    _ => { return None; }
	};
        Some(bop)
    }
}
