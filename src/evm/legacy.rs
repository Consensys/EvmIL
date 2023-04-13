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
use crate::evm::{Bytecode};

pub fn from_bytes(bytes: &[u8]) -> Bytecode {
    todo!()
}

/// Convert this bytecode contract into a byte sequence correctly
/// formatted for legacy code.
pub fn to_bytes(bytecode: &Bytecode) -> Vec<u8> {
    let mut bytes = Vec::new();
    //
    for s in bytecode { s.encode(&mut bytes); }
    // Done
    bytes
}
