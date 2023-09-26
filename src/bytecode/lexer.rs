// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::ParseError;

// ===================================================================
// Token
// ===================================================================

#[derive(Debug,PartialEq)]
pub enum Token<'a> {
    EOF, // End-Of-File (not EVM Object Format)
    Section(&'a str),
    Hex(&'a str),
    Identifier(&'a str),
    Label(&'a str),
    Loc(&'a str) // stack (or memory) location
}

impl<'a> Token<'a> {
    // Return the "length" of a token.  That is, the number of
    // characters it represents.
    pub fn len(&self) -> usize {
        match self {
            Token::EOF => 0,
            Token::Section(s) => s.len() + 1,
            Token::Hex(s) => s.len(),
            Token::Identifier(s) => s.len(),
            Token::Label(s) => s.len() + 1,
            Token::Loc(s) => s.len() + 7
        }
    }
}

// ===================================================================
// Lexer
// ===================================================================

/// A very simple lexer
pub struct Lexer<'a> {
    input: &'a str,
    chars: Vec<char>,
    index: usize
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        // FIXME: this could be made more efficient by using an
        // iterator instead of allocating a new vector.
        let chars : Vec<char> = input.chars().collect();
        //
        Self{input, chars, index: 0}
    }

    pub fn lookahead(&self) -> Result<Token<'a>,ParseError> {
        // Skip any whitespace
        let start = self.skip_whitespace(self.index);
        // Sanity check for end-of-file
        if start >= self.chars.len() {
            Ok(Token::EOF)
        } else {
            // Determine what kind of token we have.
            match self.chars[start] {
                '.' => self.scan_section_header(start),
                '0'..='9' => self.scan_hex_literal(start),
                'a'..='z'|'A'..='Z'|'_' => self.scan_id_or_label(start),
                _ => Err(ParseError::UnexpectedCharacter(start))
            }
        }
    }

    pub fn next(&mut self) -> Result<Token<'a>,ParseError> {
        // Skip any whitespace
        self.index = self.skip_whitespace(self.index);
        // Determine next token
        let tok = self.lookahead()?;
        // Account for next token
        self.index += tok.len();
        //
        Ok(tok)
    }

    fn scan_hex_literal(&self, start: usize) -> Result<Token<'a>,ParseError> {
        // Sanity check literal starts with "0x"
        if self.chars[start..].starts_with(&['0','x']) {
            // Scan all digits of this hex literal
            let end = skip(&self.chars,start + 2,|c| c.is_ascii_alphanumeric());
            // Construct token
            Ok(Token::Hex(&self.input[start..end]))
        } else {
            Err(ParseError::InvalidHexString(start))
        }
    }

    fn scan_id_or_label(&self, start: usize) -> Result<Token<'a>,ParseError> {
        // Scan all characters of this identifier or label
        let end = skip(&self.chars,start,|c| c == '_' || c.is_ascii_alphanumeric());
        // Extract slice
        let id = &self.input[start..end];
        // Distinguish label versus identifier.
        if end < self.chars.len() && self.chars[end] == ':' {
            Ok(Token::Label(id))
        } else if id == "stack" && end < self.chars.len() && self.chars[end] == '[' {
            let start = end+1;
            let end = skip(&self.chars,start,|c| c.is_ascii_alphanumeric());
            Ok(Token::Loc(&self.input[start..end]))
        } else {
            Ok(Token::Identifier(id))
        }
    }

    fn scan_section_header(&self, mut start: usize) -> Result<Token<'a>,ParseError> {
        // Move passed "."
        start += 1;
        // Scan all characters of this identifier or label
        let end = skip(&self.chars,start,|c| c.is_ascii_alphabetic());
        // Done
        Ok(Token::Section(&self.input[start..end]))
    }

    fn skip_whitespace(&self, mut index: usize) -> usize {
        index = skip(&self.chars, index, |c| c.is_ascii_whitespace());
        // Check for a comment
        if self.chars[index..].starts_with(&[';']) {
            // Skip to newline
            index = skip(&self.chars, index, |c| c != '\n');
            // Recursive call to handle trainling whitespace
            self.skip_whitespace(index)
        } else {
            index
        }
    }
}

/// Skip over any characters matching a given predicate.
fn skip<P>(input: &[char], index: usize, pred: P) -> usize
where P: Fn(char) -> bool {
    let mut i = index;
    // Continue matching
    while i < input.len() && pred(input[i]) {
        i += 1;
    }
    // Done
    i
}
