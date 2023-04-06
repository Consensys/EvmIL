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

/// A simple alias to make things a bit clearer.  In essence, this
/// generates an encoding error from a given byte or word in the
/// stream (depending on the kind of error being generated).
type EncodingErrorFn<T,E> = fn(T)->E;

/// A utility for encoding structured data into bytes.
pub struct ByteEncoder {
    bytes: Vec<u8>
}

impl ByteEncoder {
    pub const fn new() -> Self {
        Self{bytes: Vec::new()}
    }

    /// Encode a single byte into this stream.
    pub fn encode_u8(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    /// Encode a 16bit word into this stream using a big endian
    /// representation.
    pub fn encode_u16(&mut self, word: u16) {
        self.bytes.extend(word.to_be_bytes())
    }

    pub fn encode_checked_u16<E>(&mut self, word: usize, ef: EncodingErrorFn<usize,E>) -> Result<(),E> {
        if word > (u16::MAX as usize) {
            Err(ef(word))
        } else {
            self.encode_u16(word as u16);
            Ok(())
        }
    }

    pub fn encode_bytes(&mut self, bytes: Vec<u8>) {
        self.bytes.extend(bytes);
    }

    pub fn to_vec(self) -> Vec<u8> {
        self.bytes
    }
}
