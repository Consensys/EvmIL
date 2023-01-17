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

/// Identifies type which have a _maximum_ value (e.g. `u32`, etc).
pub trait Max {
    const MAX: Self;
}

/// Identifies type which have a _minimum_ value (e.g. `u32`, etc).
pub trait Min {
    const MIN: Self;
}

pub trait OverflowingAdd: Sized {
    fn overflowing_add(self, rhs: Self) -> (Self, bool);
}

pub trait OverflowingSub: Sized {
    fn overflowing_sub(self, rhs: Self) -> (Self, bool);
}

// ==================================================================
// U8
// ==================================================================

impl Max for u8 {
    const MAX: u8 = u8::MAX;
}

impl Min for u8 {
    const MIN: u8 = u8::MIN;
}

impl OverflowingAdd for u8 {
    fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }
}

impl OverflowingSub for u8 {
    fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }
}

// ==================================================================
// U16
// ==================================================================

impl Max for u16 {
    const MAX: u16 = u16::MAX;
}

impl Min for u16 {
    const MIN: u16 = u16::MIN;
}

impl OverflowingAdd for u16 {
    fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }
}

impl OverflowingSub for u16 {
    fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }
}

// ==================================================================
// U32
// ==================================================================

impl Max for u32 {
    const MAX: u32 = u32::MAX;
}

impl Min for u32 {
    const MIN: u32 = u32::MIN;
}

impl OverflowingAdd for u32 {
    fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }
}

impl OverflowingSub for u32 {
    fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }
}

// ==================================================================
// U64
// ==================================================================

impl Max for u64 {
    const MAX: u64 = u64::MAX;
}

impl Min for u64 {
    const MIN: u64 = u64::MIN;
}

impl OverflowingAdd for u64 {
    fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }
}

impl OverflowingSub for u64 {
    fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }
}

// ==================================================================
// U128
// ==================================================================

impl Max for u128 {
    const MAX: u128 = u128::MAX;
}

impl Min for u128 {
    const MIN: u128 = u128::MIN;
}

impl OverflowingAdd for u128 {
    fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }
}

impl OverflowingSub for u128 {
    fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }
}

// ==================================================================
// USIZE
// ==================================================================

impl Max for usize {
    const MAX: usize = usize::MAX;
}

impl Min for usize {
    const MIN: usize = usize::MIN;
}

impl OverflowingAdd for usize {
    fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        self.overflowing_add(rhs)
    }
}

impl OverflowingSub for usize {
    fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        self.overflowing_sub(rhs)
    }
}
