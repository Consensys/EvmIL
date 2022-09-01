use crate::lex::{Scanner,Span,TableTokenizer};
use crate::lex;

// =================================================================
// Token
// =================================================================
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Token {
    AmpersandAmpersand,
    Assert,
    BarBar,
    EOF,
    EqualEqual,
    Gap,
    If,
    Identifier,
    Integer,
    LeftAngle,
    LeftAngleEquals,
    LeftBrace,
    Minus,
    NewLine,
    Plus,
    RightAngle,
    RightAngleEquals,
    RightBrace,
    ShreakEquals,
    Star,
}

// ======================================================
// Rules
// ======================================================

const ASSERT : &'static [char] = &['a','s','s','e','r','t'];

/// Handy type alias for the result type used for all of the lexical
/// rules.
type Result = std::result::Result<Span<Token>,()>;

/// Scan an integer.
fn scan_integer(input: &[char]) -> Result {
    scan_whilst(input, Token::Integer, |c| c.is_digit(10))
}

/// Scan a keyword, which is simple identifier matching a predefined
/// pattern.
fn scan_keyword(input: &[char]) -> Result {
    // Extract keyword identifier (if applicable)
    let r = scan_whilst(input, Token::Gap, |c| c.is_ascii_alphabetic())?;
    // Attempt to match it
    let t = match &input[r.range()] {
        ASSERT => Token::Assert,
        _ => { return Err(()); }
    };
    // Success!
    Ok(Span::new(t,r.region.into()))
}

/// Scan a "gap" which is a sequence of zero or more tabs and spaces.
fn scan_gap(input: &[char]) -> Result {
    scan_whilst(input, Token::Gap, |c| c == ' ' || c == 't')
}

fn scan_newline(input: &[char]) -> Result {
    scan_one(input,Token::NewLine,'\n')
}

/// If there is nothing left to scan, then we've reached the
/// End-Of-File.
fn scan_eof(input: &[char]) -> Result {
    if input.len() == 0 {
        Ok(Span::new(Token::EOF,0..0))
    } else {
        Err(())
    }
}

/// Helper which scans an item matching a given predicate.  If no
/// characters match, then it fails.
fn scan_whilst<P>(input: &[char], t: Token, pred: P) -> Result
where P: Fn(char) -> bool {
    let mut i = 0;
    // Continue whilst predicate matches
    while i < input.len() && pred(input[i]) { i = i + 1; }
    // Check what happened
    if i == 0 {
        // Nothing matched
        Err(())
    } else {
        // Something matched
        Ok(Span::new(t, 0..i))
    }
}

fn scan_one(input: &[char], t: Token, c: char) -> Result {
    if input.len() > 0 && input[0] == c {
        Ok(Span::new(t, 0..1))
    } else {
        Err(())
    }
}

/// The set of rules used for lexing.
static RULES : &'static [Scanner<char,Token>] = &[
    scan_keyword,
    scan_integer,
    scan_gap,
    scan_newline,
    scan_eof
];

// ======================================================
// Lexer
// ======================================================

pub type Lexer = lex::Lexer<TableTokenizer<char,Token>>;

/// Construct a `Lexer` from a given string slice.
pub fn create(input: &str) -> Lexer {
    let tokenizer = TableTokenizer::new(RULES.to_vec());
    let chars = input.chars().collect();
    lex::Lexer::new(chars, tokenizer)
}

// ======================================================
// Tests
// ======================================================

#[cfg(test)]
mod tests {
    use crate::lexer;
    use crate::lexer::{Lexer,Token};

    #[test]
    fn test_01() {
        let mut l = lexer::create("");
        assert!(l.peek().kind == Token::EOF);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_02() {
        let mut l = lexer::create(" ");
        assert!(l.peek().kind == Token::Gap);
        assert!(l.next().kind == Token::Gap);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_03() {
        let mut l = lexer::create("  ");
        assert!(l.peek().kind == Token::Gap);
        assert!(l.next().kind == Token::Gap);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_04() {
        let mut l = lexer::create("\n");
        assert!(l.peek().kind == Token::NewLine);
        assert!(l.next().kind == Token::NewLine);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_05() {
        let mut l = lexer::create(" \n");
        assert!(l.peek().kind == Token::Gap);
        assert!(l.next().kind == Token::Gap);
        assert!(l.peek().kind == Token::NewLine);
        assert!(l.next().kind == Token::NewLine);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_06() {
        let mut l = lexer::create("\n ");
        assert!(l.peek().kind == Token::NewLine);
        assert!(l.next().kind == Token::NewLine);
        assert!(l.peek().kind == Token::Gap);
        assert!(l.next().kind == Token::Gap);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_07() {
        let mut l = lexer::create("\t");
        assert!(l.peek().kind == Token::Gap);
        assert!(l.next().kind == Token::Gap);
        assert!(l.peek().kind == Token::EOF);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_08() {
        let mut l = lexer::create("\t ");
        assert!(l.peek().kind == Token::Gap);
        assert!(l.next().kind == Token::Gap);
        assert!(l.peek().kind == Token::EOF);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_09() {
        let mut l = lexer::create(" \t");
        assert!(l.peek().kind == Token::Gap);
        assert!(l.next().kind == Token::Gap);
        assert!(l.peek().kind == Token::EOF);
        assert!(l.next().kind == Token::EOF);
    }

    // // Literals

    // #[test]
    // fn test_10() {
    //     let mut l = lexer::create("1");
    //     assert!(l.peek().kind == Token::Integer);
    //     assert!(l.next().kind == Token::Integer);
    //     assert!(l.next().kind == Token::EOF);
    // }

    // #[test]
    // fn test_11() {
    //     let mut l = lexer::create("  1");
    //     assert!(l.peek().kind == Token::Gap);
    //     assert!(l.next().kind == Token::Gap);
    //     assert!(peek_int(&mut l) == 1);
    //     assert!(next_int(&mut l) == 1);
    //     assert!(l.next().kind == Token::EOF);
    // }

    // #[test]
    // fn test_12() {
    //     let mut l = lexer::create("1234");
    //     assert!(peek_int(&mut l) == 1234);
    //     assert!(next_int(&mut l) == 1234);
    //     assert!(l.next().kind == Token::EOF);
    // }

    // #[test]
    // fn test_13() {
    //     let mut l = lexer::create("1234 ");
    //     assert!(peek_int(&mut l) == 1234);
    //     assert!(next_int(&mut l) == 1234);
    //     assert!(l.peek().kind == Token::Gap);
    //     assert!(l.next().kind == Token::Gap);
    //     assert!(l.next().kind == Token::EOF);
    // }

    // #[test]
    // fn test_14() {
    //     let mut l = lexer::create("1234_");
    //     assert!(l.peek().kind == Token::Integer);
    //     assert!(l.next().kind == Token::Integer);
    //     assert!(l.peek().kind == Token::Identifier);
    //     assert!(l.next().kind == Token::Identifier);
    //     assert!(l.peek().kind == Token::EOF);
    // }

    // #[test]
    // fn test_15() {
    //     let mut l = lexer::create("1234X");
    //     assert!(peek_int(&mut l) == 1234);
    //     assert!(next_int(&mut l) == 1234);
    //     assert!(l.peek().kind == Token::Identifier);
    //     assert!(l.next().kind == Token::Identifier);
    //     assert!(l.peek().kind == Token::EOF);
    // }

    // #[test]
    // fn test_16() {
    //     let mut l = lexer::create("1234 12");
    //     assert!(peek_int(&mut l) == 1234);
    //     assert!(next_int(&mut l) == 1234);
    //     assert!(l.peek().kind == Token::Gap);
    //     assert!(l.next().kind == Token::Gap);
    //     assert!(peek_int(&mut l) == 12);
    //     assert!(peek_int(&mut l) == 12);
    // }

    // // Identifiers

    // #[test]
    // fn test_20() {
    //     let mut l = lexer::create("abc");
    //     let t = l.next();
    //     assert!(t.kind == Token::Identifier);
    //     assert!(l.get_content(t) == "abc");
    //     assert!(l.next().kind == Token::EOF);
    // }

    // #[test]
    // fn test_21() {
    //     let mut l = lexer::create("  abc");
    //     assert!(l.peek().kind == Token::Gap);
    //     assert!(l.next().kind == Token::Gap);
    //     assert!(l.peek().kind == Token::Identifier);
    //     let t = l.next();
    //     assert!(t.kind == Token::Identifier);
    //     assert!(l.get_content(t) == "abc");
    //     assert!(l.next().kind == Token::EOF);
    // }

    // #[test]
    // fn test_22() {
    //     let mut l = lexer::create("_abc");
    //     assert!(l.peek().kind == Token::Identifier);
    //     let t = l.next();
    //     assert!(t.kind == Token::Identifier);
    //     assert!(l.get_content(t) == "_abc");
    //     assert!(l.next().kind == Token::EOF);
    // }

    // #[test]
    // fn test_23() {
    //     let mut l = lexer::create("a_bD12233_");
    //     assert!(l.peek().kind == Token::Identifier);
    //     let t = l.next();
    //     assert!(t.kind == Token::Identifier);
    //     assert!(l.get_content(t) == "a_bD12233_");
    //     assert!(l.next().kind == Token::EOF);
    // }

    // #[test]
    // fn test_24() {
    //     let mut l = lexer::create("_abc cd");
    //     assert!(l.peek().kind == Token::Identifier);
    //     let t1 = l.next();
    //     assert!(t1.kind == Token::Identifier);
    //     assert!(l.get_content(t1) == "_abc");
    //     assert!(l.peek().kind == Token::Gap);
    //     assert!(l.next().kind == Token::Gap);
    //     assert!(l.peek().kind == Token::Identifier);
    //     let t2 = l.next();
    //     assert!(t2.kind == Token::Identifier);
    //     assert!(l.get_content(t2) == "cd");
    //     assert!(l.next().kind == Token::EOF);
    // }

    // // Keywords

    // #[test]
    // fn test_30() {
    //     let mut l = lexer::create("if");
    //     assert!(l.peek().kind == Token::If);
    //     assert!(l.next().kind == Token::If);
    //     assert!(l.next().kind == Token::EOF);
    // }

    // // Operators

    // #[test]
    // fn test_40() {
    //     let mut l = lexer::create("(");
    //     assert!(l.peek().kind == Token::LeftBrace);
    //     assert!(l.next().kind == Token::LeftBrace);
    //     assert!(l.next().kind == Token::EOF);
    // }

    // #[test]
    // fn test_41() {
    //     let mut l = lexer::create("((");
    //     assert!(l.peek().kind == Token::LeftBrace);
    //     assert!(l.next().kind == Token::LeftBrace);
    //     assert!(l.peek().kind == Token::LeftBrace);
    //     assert!(l.next().kind == Token::LeftBrace);
    //     assert!(l.next().kind == Token::EOF);
    // }

    // #[test]
    // fn test_42() {
    //     let mut l = lexer::create(")");
    //     assert!(l.peek().kind == Token::RightBrace);
    //     assert!(l.next().kind == Token::RightBrace);
    // }

    // #[test]
    // fn test_43() {
    //     let mut l = lexer::create("))");
    //     assert!(l.peek().kind == Token::RightBrace);
    //     assert!(l.next().kind == Token::RightBrace);
    //     assert!(l.peek().kind == Token::RightBrace);
    //     assert!(l.next().kind == Token::RightBrace);
    //     assert!(l.next().kind == Token::EOF);
    // }

    // #[test]
    // fn test_44() {
    //     let mut l = lexer::create("()");
    //     assert!(l.peek().kind == Token::LeftBrace);
    //     assert!(l.next().kind == Token::LeftBrace);
    //     assert!(l.peek().kind == Token::RightBrace);
    //     assert!(l.next().kind == Token::RightBrace);
    //     assert!(l.next().kind == Token::EOF);
    // }


    // #[test]
    // fn test_61() {
    //     let mut l = lexer::create("12345(");
    //     let t1 = l.next();
    //     assert!(t1.kind == Token::Integer);
    //     assert!(l.get_int(t1) == 12345);
    //     let t2 = l.next();
    //     assert!(t2.kind == Token::LeftBrace);
    //     assert!(l.get_content(t2) == "(");
    //     assert!(l.next().kind == Token::EOF);
    // }

    // fn peek_int(l: &mut Lexer) -> i32 {
    //     let t = l.peek(); l.get_int(t)
    // }

    // fn next_int(l: &mut Lexer) -> i32 {
    //     let t = l.next(); l.get_int(t)
    // }
}
