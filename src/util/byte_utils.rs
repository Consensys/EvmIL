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

/// Convert a 128bit value into the smallest possible byte sequence
/// (in big endian order).
pub fn to_be_bytes(mut val: u128) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    // Convert digits in a given radix into bytes (in little endian)
    if val == 0 {
        bytes.push(0);
    } else {
        while val != 0 {
            bytes.push((val % 256) as u8);
            val >>= 8;
        }
    }
    // Convert from little endian to big endian format.
    bytes.reverse();
    //
    bytes
}

/// Convert a sequence of bytes in big endian form into a 128bit
/// value.
pub fn from_be_bytes(bytes: &[u8]) -> u128 {
    let mut val = 0;
    //
    for b in bytes {
        val = (val << 8) | (*b as u128);
    }
    //
    val
}

/// Convert a sequence of digits into a u128.
pub fn from_be_digits(digits: &[u8], radix: u32) -> u128 {
    let mut acc: u128 = 0;
    let mut base: u128 = 1;
    //
    for i in (0..digits.len()).rev() {
        let d = digits[i] as u128;
        // NOTE: this could overflow.
        acc += d * base;
        if i > 0 {
            // NOTE: Following overflows on last iteration, so just
            // don't do it :)
            base *= radix as u128;
        }
    }
    // Done
    acc
}
