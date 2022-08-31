use crate::Term;

pub struct Parser<'a> {
    content: &'a str
}

impl<'a> Parser<'a> {
    pub fn new(content: &'a str) -> Self {
        Parser{content}
    }

    pub fn parse(&self) -> Result<Term,()> {
        todo!("GOT HERE");
    }
}
