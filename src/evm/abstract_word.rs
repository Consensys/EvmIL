use std::{fmt,ops};
use crate::evm::Word;
use crate::util::{w256,SortedVec,JoinInto,Bottom,Concretizable,Top};

#[derive(Clone,Debug,PartialOrd)]
pub struct AbstractWord {
    items: Option<SortedVec<w256>>
}

impl AbstractWord {
    pub fn new() -> Self {
        AbstractWord{items: Some(SortedVec::new())}
    }
}

impl From<w256> for AbstractWord {
    fn from(w: w256) -> Self {
        AbstractWord{items: Some(vec![w].into())}
    }
}

impl PartialEq for AbstractWord {
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}

impl ops::Add for AbstractWord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!()
    }
}

impl fmt::Display for AbstractWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.items)
    }
}

// ==========================================================
// Lattice
// ==========================================================

impl Top for AbstractWord {
    const TOP : Self = AbstractWord{items:None};
}

impl Bottom for AbstractWord {
    const BOTTOM : Self = AbstractWord{items:Some(SortedVec::new())};
}

impl JoinInto for AbstractWord {
    /// Merge another abstract value into this value.
    fn join_into(&mut self, other: &Self) -> bool {
        match (&self.items,&other.items) {
            (None,_) => false,
            (_,None) => {
                self.items = None;
                true
            }
            (Some(_),Some(ws)) => {
                self.items.as_mut().unwrap().insert_all(ws)
            }
        }
    }

}

impl Concretizable for AbstractWord {
    type Item = w256;

    fn constant(&self) -> w256 {
        let items = self.items.as_ref().unwrap();
        if items.len() == 1 {
            items[0]
        } else {
            panic!("not constant: {}", self);
        }
    }

    fn is_constant(&self) -> bool {
        match &self.items {
            Some(vs) => vs.len() == 1,
            _ => false
        }
    }
}

// ==========================================================
// Word
// ==========================================================

impl Word for AbstractWord {

}
