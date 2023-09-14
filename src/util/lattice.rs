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

/// An abstract value can be used to represent one or more concrete values.
pub trait JoinInto<Rhs:?Sized = Self> {
    /// Merge another abstract value into this value.
    fn join_into(&mut self, other: &Rhs) -> bool;
}

pub trait Join {
    fn join(&self, other: &Self) -> Self;
}

impl<T: JoinInto + Clone> Join for T {
    fn join(&self, other: &Self) -> Self {
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
    const TOP: Self;
}

pub trait IsTop {
    /// Check whether `self` is the special top value or not.
    fn is_top(&self) -> bool;
}

impl<T: PartialEq + Top> IsTop for T {
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
    const BOTTOM: Self;
}

pub trait IsBottom {
    /// Check whether `self` is the bottom value or not.
    fn is_bottom(&self) -> bool;
}

impl<T: PartialEq + Bottom> IsBottom for T {
    fn is_bottom(&self) -> bool {
        self == &T::BOTTOM
    }
}

// ===================================================================
// Constant
// ===================================================================

/// A trait which allows (when possible) an abstract value to be extracted into
/// a concrete value.  This makes sense only when that abstract value represents
/// a single concrete value.
pub trait Concretizable {
    type Item;

    /// Determine whether this abstract value is a constant or not.
    fn is_constant(&self) -> bool;

    /// Extract constant value.
    fn constant(&self) -> Self::Item;
}

// ===================================================================
// JoinSemiLattice
// ===================================================================

/// A partially ordered set which has a join for any two elements, and includes
/// a _bottom_ (i.e. least) element which is below every other element.  For
/// example, the integer sets meets these requirements where the join operation
/// is set union, and bottom is the empty set.
pub trait JoinSemiLattice: JoinInto + Bottom {}

/// Default implementation
impl<T: JoinInto + Bottom> JoinSemiLattice for T {}

// ===================================================================
// JoinLattice
// ===================================================================

/// A partially ordered set which has a join for any two elements, and includes
/// both a _bottom_ (i.e. least) and _top_ (i.e. most) elements.  These are
/// respectively below (above) every other element.
pub trait JoinLattice: JoinSemiLattice + Top {}

/// Default implementation
impl<T: JoinInto + Bottom + Top> JoinLattice for T {}
