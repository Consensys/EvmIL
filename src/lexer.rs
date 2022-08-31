use crate::lex::{Span};
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
// Tokenizer
// ======================================================
pub struct CharTokenizer {
    // Rules go here
}

impl lex::Tokenizer for CharTokenizer {
    type Token = Token;

    fn scan(&self, offset: usize, input: &[char]) -> Span<Self::Token> {
        todo!("fix me");
    }
}

// ======================================================
// Lexer
// ======================================================

pub type Lexer = lex::Lexer<CharTokenizer>;

pub fn create(input: &str) -> Lexer {
    let tokenizer = CharTokenizer{};
    lex::Lexer::new(input, tokenizer)
}

// ======================================================
// Tests
// ======================================================

#[cfg(test)]
mod tests {
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
        assert!(peek_int(&mut l) == 1);
        assert!(next_int(&mut l) == 1);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_12() {
        let mut l = Lexer::new("1234");
        assert!(peek_int(&mut l) == 1234);
        assert!(next_int(&mut l) == 1234);
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_13() {
        let mut l = Lexer::new("1234 ");
        assert!(peek_int(&mut l) == 1234);
        assert!(next_int(&mut l) == 1234);
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
        assert!(peek_int(&mut l) == 1234);
        assert!(next_int(&mut l) == 1234);
        assert!(l.peek().kind == Token::Identifier);
        assert!(l.next().kind == Token::Identifier);
        assert!(l.peek().kind == Token::EOF);
    }

    #[test]
    fn test_16() {
        let mut l = Lexer::new("1234 12");
        assert!(peek_int(&mut l) == 1234);
        assert!(next_int(&mut l) == 1234);
        assert!(l.peek().kind == Token::Gap);
        assert!(l.next().kind == Token::Gap);
        assert!(peek_int(&mut l) == 12);
        assert!(peek_int(&mut l) == 12);
    }

    // Identifiers

    #[test]
    fn test_20() {
        let mut l = Lexer::new("abc");
        let t = l.next();
        assert!(t.kind == Token::Identifier);
        assert!(l.get_content(t) == "abc");
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
        assert!(l.get_content(t) == "abc");
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_22() {
        let mut l = Lexer::new("_abc");
        assert!(l.peek().kind == Token::Identifier);
        let t = l.next();
        assert!(t.kind == Token::Identifier);
        assert!(l.get_content(t) == "_abc");
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_23() {
        let mut l = Lexer::new("a_bD12233_");
        assert!(l.peek().kind == Token::Identifier);
        let t = l.next();
        assert!(t.kind == Token::Identifier);
        assert!(l.get_content(t) == "a_bD12233_");
        assert!(l.next().kind == Token::EOF);
    }

    #[test]
    fn test_24() {
        let mut l = Lexer::new("_abc cd");
        assert!(l.peek().kind == Token::Identifier);
        let t1 = l.next();
        assert!(t1.kind == Token::Identifier);
        assert!(l.get_content(t1) == "_abc");
        assert!(l.peek().kind == Token::Gap);
        assert!(l.next().kind == Token::Gap);
        assert!(l.peek().kind == Token::Identifier);
        let t2 = l.next();
        assert!(t2.kind == Token::Identifier);
        assert!(l.get_content(t2) == "cd");
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
    fn test_61() {
        let mut l = Lexer::new("12345(");
        let t1 = l.next();
        assert!(t1.kind == Token::Integer);
        assert!(l.get_int(t1) == 12345);
        let t2 = l.next();
        assert!(t2.kind == Token::LeftBrace);
        assert!(l.get_content(t2) == "(");
        assert!(l.next().kind == Token::EOF);
    }

    fn peek_int(l: &mut Lexer) -> i32 {
        let t = l.peek(); l.get_int(t)
    }

    fn next_int(l: &mut Lexer) -> i32 {
        let t = l.next(); l.get_int(t)
    }
}
