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

use super::AssemblyError;

// ===================================================================
// Token
// ===================================================================

#[derive(Debug,PartialEq)]
pub enum Token<'a> {
    EOF, // End-Of-File (not EVM Object Format)
    Section(&'a str),
    Hex(&'a str),
    Identifier(&'a str),
    Label(&'a str)
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
            Token::Label(s) => s.len() + 1
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

    pub fn lookahead(&self) -> Result<Token<'a>,AssemblyError> {
        // Skip any whitespace
        let start = skip(&self.chars, self.index, |c| c.is_ascii_whitespace());
        // Sanity check for end-of-file
        if start >= self.chars.len() {
            Ok(Token::EOF)
        } else {
            // Determine what kind of token we have.
            match self.chars[start] {
                '.' => self.scan_section_header(start),
                '0'..='9' => self.scan_hex_literal(start),
                'a'..='z'|'A'..='Z'|'_' => self.scan_id_or_label(start),
                _ => Err(AssemblyError::UnexpectedCharacter(start))
            }
        }
    }

    pub fn next(&mut self) -> Result<Token<'a>,AssemblyError> {
        // Skip any whitespace
        self.index = skip(&self.chars, self.index, |c| c.is_ascii_whitespace());
        // Determine next token
        let tok = self.lookahead()?;
        // Account for next token
        self.index += tok.len();
        //
        Ok(tok)
    }

    fn scan_hex_literal(&self, start: usize) -> Result<Token<'a>,AssemblyError> {
        // Sanity check literal starts with "0x"
        if self.chars[start..].starts_with(&['0','x']) {
            // Scan all digits of this hex literal
            let end = skip(&self.chars,start + 2,|c| c.is_ascii_alphanumeric());
            // Construct token
            Ok(Token::Hex(&self.input[start..end]))
        } else {
            Err(AssemblyError::InvalidHexString(start))
        }
    }

    fn scan_id_or_label(&self, start: usize) -> Result<Token<'a>,AssemblyError> {
        // Scan all characters of this identifier or label
        let end = skip(&self.chars,start,|c| c.is_ascii_alphanumeric());
        // Distinguish label versus identifier.
        if end < self.chars.len() && self.chars[end] == ':' {
            Ok(Token::Label(&self.input[start..end]))
        } else {
            Ok(Token::Identifier(&self.input[start..end]))
        }
    }

    fn scan_section_header(&self, mut start: usize) -> Result<Token<'a>,AssemblyError> {
        // Move passed "."
        start = start + 1;
        // Scan all characters of this identifier or label
        let end = skip(&self.chars,start,|c| c.is_ascii_alphabetic());
        // Done
        Ok(Token::Section(&self.input[start..end]))
    }
}

/// Skip over any characters matching a given predicate.
fn skip<P>(input: &[char], index: usize, pred: P) -> usize
where P: Fn(char) -> bool {
    let mut i = index;
    // Continue matching
    while i < input.len() && pred(input[i]) {
        i = i + 1;
    }
    // Done
    i
}
