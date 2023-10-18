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
use ruint::{Uint};
use crate::util;

/// Represents a `256` bit word.  This is very similar what a `u256`
/// would be, but where all operations employ modulo arithmetic.
#[allow(non_camel_case_types)]
pub type w256 = Uint<256,4>;

pub const W256_ZERO : w256 = w256::from_limbs([0,0,0,0]);
pub const W256_ONE : w256 = w256::from_limbs([1,0,0,0]);
pub const W256_TWO : w256  = w256::from_limbs([2,0,0,0]);
pub const W256_THREE : w256  = w256::from_limbs([3,0,0,0]);
pub const W256_THIRTYTWO : w256  = w256::from_limbs([32,0,0,0]);

// =====================================================================
// Min / Max
// =====================================================================

impl util::Max for w256 {
    const MAX: Self = w256::MAX;
}

impl util::Min for w256 {
    const MIN: Self = w256::MIN;
}
