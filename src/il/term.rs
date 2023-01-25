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

// ============================================================================
// Terms
// ============================================================================

#[derive(Clone)]
pub enum Term {
    // Statements
    Assert(Box<Term>),
    Assignment(Box<Term>, Box<Term>),
    Goto(String),
    IfGoto(Box<Term>, String),
    Label(String),
    Succeed(Vec<Term>),
    Revert(Vec<Term>),
    Fail,
    Stop,
    // Expressions
    Binary(BinOp, Box<Term>, Box<Term>),
    ArrayAccess(Box<Term>, Box<Term>),
    MemoryAccess(Region),
    // Values
    Int(Vec<u8>),
    Hex(Vec<u8>),
}

// ============================================================================
// Binary Operators
// ============================================================================

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BinOp {
    // Arithmetic
    Add,
    Subtract,
    Divide,
    Multiply,
    Remainder,
    // Comparators
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
    // Logical
    LogicalAnd,
    LogicalOr,
}

// ============================================================================
// Memory Regions
// ============================================================================

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Region {
    Memory,
    Storage,
    CallData,
}
