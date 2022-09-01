use std::ops::Range;
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
    EqualsEquals,
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
const IF : &'static [char] = &['i','f'];

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
        IF => Token::If,
        _ => { return Err(()); }
    };
    // Success!
    Ok(Span::new(t,r.region.into()))
}

/// Scan an identifier which starts with an alpabetic character, or an
/// underscore and subsequently contains zero or more alpha-number
/// characters or underscores.
fn scan_identifier(input: &[char]) -> Result {
    if input.len() > 0 && is_identifier_start(input[0]) {
        scan_whilst(input, Token::Identifier, is_identifier_middle)
    } else {
        Err(())
    }
}

/// Scan all single-character operators.
fn scan_single_operators(input: &[char]) -> Result {
    if input.len() == 0 {
        Err(())
    } else {
        let t = match input[0] {
            '<' => Token::LeftAngle,
            '(' => Token::LeftBrace,
            '-' => Token::Minus,
            '+' => Token::Plus,
            '>' => Token::RightAngle,
            ')' => Token::RightBrace,
            '*' => Token::Star,
            _ => { return Err(()); }
        };
        //
        Ok(Span::new(t,0..1))
    }
}

/// Scan all double-character operators.
fn scan_double_operators(input: &[char]) -> Result {
    if input.len() <= 1 {
        Err(())
    } else {
        let t = match (input[0], input[1]) {
            ('&','&') => Token::AmpersandAmpersand,
            ('|','|') => Token::BarBar,
            ('=','=') => Token::EqualsEquals,
            ('<','=') => Token::LeftAngleEquals,
            ('>','=') => Token::RightAngleEquals,
            ('!','=') => Token::ShreakEquals,
            _ => { return Err(()); }
        };
        //
        Ok(Span::new(t,0..2))
    }
}

/// Determine whether a given character is the start of an identifier.
fn is_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

/// Determine whether a given character can occur in the middle of an
/// identifier
fn is_identifier_middle(c: char) -> bool {
    c.is_digit(10) || is_identifier_start(c)
}

/// Scan a "gap" which is a sequence of zero or more tabs and spaces.
fn scan_gap(input: &[char]) -> Result {
    scan_whilst(input, Token::Gap, |c| c == ' ' || c == '\t')
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
    scan_double_operators,
    scan_single_operators,
    scan_keyword,
    scan_identifier,
    scan_integer,
    scan_gap,
    scan_newline,
    scan_eof
];

// ======================================================
// Lexer
// ======================================================

pub struct Lexer {
    /// Internal lexer used for the heavy lifting.
    lexer: lex::Lexer<TableTokenizer<char,Token>>
}

impl Lexer {
    /// Construct a `Lexer` from a given string slice.
    pub fn new(input: &str) -> Lexer {
        let tokenizer = TableTokenizer::new(RULES.to_vec());
        let chars = input.chars().collect();
        Lexer{lexer:lex::Lexer::new(chars, tokenizer)}
    }

    /// Turn an integer token into a `i32`.  Observe that this will
    /// panic if the underlying characters of the token don't parse.
    fn get_int(&self, t: Span<Token>) -> i32 {
        // Sanity check this makes sense.
        assert!(t.kind == Token::Integer);
        // Extract characters from token.
        let chars = self.lexer.get(t);
        // Convert to string
        let s: String = chars.into_iter().collect();
        // Parse to i32
        s.parse().unwrap()
    }

    fn get_str(&self, t: Span<Token>) -> String {
        // Extract characters from token.
        let chars = self.lexer.get(t);
        // Convert to string
        chars.into_iter().collect()
    }

    pub fn peek(&self) -> Span<Token> { self.lexer.peek() }

    pub fn next(&mut self) -> Span<Token> { self.lexer.next() }
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
        let mut l = Lexer::new("");
        assert!(l.peek().kind == Token::EOF);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_02() {
        let mut l = Lexer::new(" ");
        assert!(l.peek().kind == Token::Gap);
        assert!(l.next().kind == Token::Gap);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_03() {
        let mut l = Lexer::new("  ");
        assert!(l.peek().kind == Token::Gap);
        assert!(l.next().kind == Token::Gap);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_04() {
        let mut l = Lexer::new("\n");
        assert!(l.peek().kind == Token::NewLine);
        assert!(l.next().kind == Token::NewLine);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_05() {
        let mut l = Lexer::new(" \n");
        assert!(l.peek().kind == Token::Gap);
        assert!(l.next().kind == Token::Gap);
        assert!(l.peek().kind == Token::NewLine);
        assert!(l.next().kind == Token::NewLine);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_06() {
        let mut l = Lexer::new("\n ");
        assert!(l.peek().kind == Token::NewLine);
        assert!(l.next().kind == Token::NewLine);
        assert!(l.peek().kind == Token::Gap);
        assert!(l.next().kind == Token::Gap);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_07() {
        let mut l = Lexer::new("\t");
        assert!(l.peek().kind == Token::Gap);
        assert!(l.next().kind == Token::Gap);
        assert!(l.peek().kind == Token::EOF);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_08() {
        let mut l = Lexer::new("\t ");
        assert!(l.peek().kind == Token::Gap);
        assert!(l.next().kind == Token::Gap);
        assert!(l.peek().kind == Token::EOF);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_09() {
        let mut l = Lexer::new(" \t");
        assert!(l.peek().kind == Token::Gap);
        assert!(l.next().kind == Token::Gap);
        assert!(l.peek().kind == Token::EOF);
        assert!(l.next().kind == Token::EOF);
    }

    // Literals

    #[test]
    fn test_10() {
        let mut l = Lexer::new("1");
        assert!(l.peek().kind == Token::Integer);
        assert!(l.next().kind == Token::Integer);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_11() {
        let mut l = Lexer::new("  1");
        assert!(l.peek().kind == Token::Gap);
        assert!(l.next().kind == Token::Gap);
        assert!(l.get_int(l.peek()) == 1);
        assert!(l.next().kind == Token::Integer);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_12() {
        let mut l = Lexer::new("1234");
        assert!(l.get_int(l.peek()) == 1234);
        l.next();
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_13() {
        let mut l = Lexer::new("1234 ");
        assert!(l.get_int(l.peek()) == 1234);
        l.next();
        assert!(l.peek().kind == Token::Gap);
        assert!(l.next().kind == Token::Gap);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_14() {
        let mut l = Lexer::new("1234_");
        assert!(l.peek().kind == Token::Integer);
        assert!(l.next().kind == Token::Integer);
        assert!(l.peek().kind == Token::Identifier);
        assert!(l.next().kind == Token::Identifier);
        assert!(l.peek().kind == Token::EOF);
    }

    #[test]
    fn test_15() {
        let mut l = Lexer::new("1234X");
        assert!(l.get_int(l.peek()) == 1234);
        l.next();
        assert!(l.peek().kind == Token::Identifier);
        assert!(l.next().kind == Token::Identifier);
        assert!(l.peek().kind == Token::EOF);
    }

    #[test]
    fn test_16() {
        let mut l = Lexer::new("1234 12");
        assert!(l.get_int(l.peek()) == 1234);
        l.next();
        assert!(l.peek().kind == Token::Gap);
        assert!(l.next().kind == Token::Gap);
        assert!(l.get_int(l.peek()) == 12);
        assert!(l.get_int(l.peek()) == 12);
    }

    // Identifiers

    #[test]
    fn test_20() {
        let mut l = Lexer::new("abc");
        let t = l.next();
        assert!(t.kind == Token::Identifier);
        assert!(l.get_str(t) == "abc");
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_21() {
        let mut l = Lexer::new("  abc");
        assert!(l.peek().kind == Token::Gap);
        assert!(l.next().kind == Token::Gap);
        assert!(l.peek().kind == Token::Identifier);
        let t = l.next();
        assert!(t.kind == Token::Identifier);
        assert!(l.get_str(t) == "abc");
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_22() {
        let mut l = Lexer::new("_abc");
        assert!(l.peek().kind == Token::Identifier);
        let t = l.next();
        assert!(t.kind == Token::Identifier);
        assert!(l.get_str(t) == "_abc");
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_23() {
        let mut l = Lexer::new("a_bD12233_");
        assert!(l.peek().kind == Token::Identifier);
        let t = l.next();
        assert!(t.kind == Token::Identifier);
        assert!(l.get_str(t) == "a_bD12233_");
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_24() {
        let mut l = Lexer::new("_abc cd");
        assert!(l.peek().kind == Token::Identifier);
        let t1 = l.next();
        assert!(t1.kind == Token::Identifier);
        assert!(l.get_str(t1) == "_abc");
        assert!(l.peek().kind == Token::Gap);
        assert!(l.next().kind == Token::Gap);
        assert!(l.peek().kind == Token::Identifier);
        let t2 = l.next();
        assert!(t2.kind == Token::Identifier);
        assert!(l.get_str(t2) == "cd");
        assert!(l.next().kind == Token::EOF);
    }

    // Keywords

    #[test]
    fn test_30() {
        let mut l = Lexer::new("if");
        assert!(l.peek().kind == Token::If);
        assert!(l.next().kind == Token::If);
        assert!(l.next().kind == Token::EOF);
    }

    // Operators

    #[test]
    fn test_40() {
        let mut l = Lexer::new("(");
        assert!(l.peek().kind == Token::LeftBrace);
        assert!(l.next().kind == Token::LeftBrace);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_41() {
        let mut l = Lexer::new("((");
        assert!(l.peek().kind == Token::LeftBrace);
        assert!(l.next().kind == Token::LeftBrace);
        assert!(l.peek().kind == Token::LeftBrace);
        assert!(l.next().kind == Token::LeftBrace);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_42() {
        let mut l = Lexer::new(")");
        assert!(l.peek().kind == Token::RightBrace);
        assert!(l.next().kind == Token::RightBrace);
    }

    #[test]
    fn test_43() {
        let mut l = Lexer::new("))");
        assert!(l.peek().kind == Token::RightBrace);
        assert!(l.next().kind == Token::RightBrace);
        assert!(l.peek().kind == Token::RightBrace);
        assert!(l.next().kind == Token::RightBrace);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_44() {
        let mut l = Lexer::new("()");
        assert!(l.peek().kind == Token::LeftBrace);
        assert!(l.next().kind == Token::LeftBrace);
        assert!(l.peek().kind == Token::RightBrace);
        assert!(l.next().kind == Token::RightBrace);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_45() {
        let mut l = Lexer::new("<=");
        assert!(l.next().kind == Token::LeftAngleEquals);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_46() {
        let mut l = Lexer::new(">=");
        assert!(l.next().kind == Token::RightAngleEquals);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_47() {
        let mut l = Lexer::new("==");
        assert!(l.next().kind == Token::EqualsEquals);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_48() {
        let mut l = Lexer::new("!=");
        assert!(l.next().kind == Token::ShreakEquals);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_49() {
        let mut l = Lexer::new("&&");
        assert!(l.next().kind == Token::AmpersandAmpersand);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_50() {
        let mut l = Lexer::new("||");
        assert!(l.next().kind == Token::BarBar);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_61() {
        let mut l = Lexer::new("12345(");
        let t1 = l.next();
        assert!(t1.kind == Token::Integer);
        assert!(l.get_int(t1) == 12345);
        let t2 = l.next();
        assert!(t2.kind == Token::LeftBrace);
        assert!(l.get_str(t2) == "(");
        assert!(l.next().kind == Token::EOF);
    }
}
