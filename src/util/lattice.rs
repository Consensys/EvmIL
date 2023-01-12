/// An abstract value can be used to represent one or more concrete values.
pub trait JoinInto {
    /// Merge another abstract value into this value.
    fn join_into(&mut self, other: &Self);
}

pub trait Join {
    fn join(&self,other:&Self) -> Self;
}

impl<T:JoinInto+Clone> Join for T {
    fn join(&self,other:&Self) -> Self {
	let mut tmp = self.clone();
	tmp.join_into(other);
	tmp
    }
}
