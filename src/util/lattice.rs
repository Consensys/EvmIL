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


// ===================================================================
// Bottom
// ===================================================================

/// Defines an abstract value which has a specific TOP value in the
/// lattice.
pub trait Top {
    const TOP : Self;
}

pub trait IsTop {
    /// Check whether `self` is the special top value or not.
    fn is_top(&self) -> bool;
}

impl<T:PartialEq+Top> IsTop for T {
    fn is_top(&self) -> bool {
        self == &T::TOP
    }
}

// ===================================================================
// Bottom
// ===================================================================

/// Defines an abstract value which has a specific BOTTOM value in the
/// lattice.
pub trait Bottom {
    const BOTTOM : Self;
}

pub trait IsBottom {
    /// Check whether `self` is the bottom value or not.
    fn is_bottom(&self) -> bool;
}

impl<T:PartialEq+Bottom> IsBottom for T {
    fn is_bottom(&self) -> bool {
        self == &T::BOTTOM
    }
}

// ===================================================================
// Constant
// ===================================================================

pub trait Constant {
    type Item;

    /// Determine whether this abstract value is a constant or not.
    fn is_constant(&self) -> bool;

    /// Extract constant value.
    fn constant(&self) -> Self::Item;
}
