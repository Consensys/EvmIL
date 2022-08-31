use crate::Term;
use crate::lexer;
use crate::lexer::{Lexer,Token};

pub struct Parser {
    /// Provides access to our token stream.
    lexer: Lexer
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Self { lexer: lexer::create(input) }
    }

    /// Parse a line of text into a term.
    pub fn parse(&mut self) -> Result<Term,()> {
        self.parse_stmt()
    }

    // =========================================================================
    // Statements
    // =========================================================================

    fn parse_stmt(&mut self) -> Result<Term,()> {
    	let lookahead = self.lexer.peek();
    	//
    	let stmt = match lookahead.kind {
    	    Token::Assert => self.parse_stmt_assert(),
            _ => {
                // Unknown statement
                Err(())
            }
        };
        //
        stmt
    }

    pub fn parse_stmt_assert(&mut self) -> Result<Term,()> {
        println!("parse_stmt_assert()");
    	let tok = self.snap(Token::Assert)?;
    	let expr = self.parse_expr()?;
        todo!("implement parse_stmt_assert()");
    }

    // =========================================================================
    // Expressions
    // =========================================================================

    pub fn parse_expr(&mut self) -> Result<Term,()> {
        // Skip whitespace
        self.skip_whitespace();
        //
    	let lookahead = self.lexer.peek();
    	//
    	let expr = match lookahead.kind {
    	    Token::Integer => self.parse_literal_int()?,
    	    Token::LeftBrace => self.parse_expr_bracketed()?,
    	    _ => {
    		return Err(());
    	    }
    	};
        // Done
        Ok(expr)
    }

    pub fn parse_expr_bracketed(&mut self) -> Result<Term,()> {
    	self.snap(Token::LeftBrace)?;
    	let expr = self.parse_expr();
    	self.snap(Token::RightBrace)?;
        expr
    }

    pub fn parse_literal_int(&mut self) -> Result<Term,()> {
        let tok = self.snap(Token::Integer)?;
        //let val = self.lexer.get_int(tok);
        todo!("implement parse_literal_int()");
    }

    // =========================================================================
    // Helpers
    // =========================================================================

    /// Match a given token type in the current stream.  If the kind
    /// matches, then the token stream advances.  Otherwise, it
    /// remains at the same position and an error is returned.
    fn snap(&mut self, kind : Token) -> Result<Token,()> {
	// Peek at the next token
	let lookahead = self.lexer.peek();
	// Check it!
	if lookahead.kind == kind {
	    // Accept it
	    self.lexer.next();
	    //
	    Ok(lookahead.kind)
	} else {
	    // Reject
	    Err(())
	}
    }

    fn skip_whitespace(&mut self) -> Result<(),()> {
        let lookahead = self.lexer.peek();
        //
        match lookahead.kind {
            Token::EOF => {
                Ok(())
            }
            Token::Gap => {
                self.snap(lookahead.kind)?;
                self.skip_whitespace()
            }
            _ => {
                // Do nothing!
                Ok(())
            }
        }
    }
}
