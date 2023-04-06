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
            val = val >> 8;
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
    for i in 0..bytes.len() {
        val = (val << 8) | (bytes[i] as u128);
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
        acc = acc + (d * base);
        if i > 0 {
            // NOTE: Following overflows on last iteration, so just
            // don't do it :)
            base = base * (radix as u128);
        }
    }
    // Done
    acc
}


/// A simple alias to make things a bit clearer.  In essence, this
/// generates a decoding error from a given byte or word in the stream
/// (depending on the kind of error being generated).
type DecodingErrorFn<T,E> = fn(T)->E;

/// Helper for pulling information out of an EOF formatted byte
/// stream.
pub struct ByteDecoder<'a> {
    bytes: &'a [u8],
    index: usize
}

impl<'a> ByteDecoder<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self{bytes,index:0}
    }

    /// Attempt to match a given `u8` byte in the bytestream at the
    /// present position.  If the match fails, an error is generating
    /// using the provided decoding error generator.
    pub fn match_u8<E:Default>(&mut self, n: u8, ef: DecodingErrorFn<u8,E>) -> Result<(),E> {
        let m = self.next_u8()?;
        if m == n { Ok(()) }
        else { Err(ef(m)) }
    }

    /// Attempt to match a given `u16` word in the bytestream at the
    /// present position assuming a _big endian_ representation.  If
    /// the match fails, an error is generating using the provided
    /// decoding error generator.
    pub fn match_u16<E:Default>(&mut self, n: u16, ef: DecodingErrorFn<u16,E>) -> Result<(),E> {
        let m = self.next_u16()?;
        if m == n { Ok(()) }
        else { Err(ef(m)) }
    }

    /// Attempt to match the _end of file_.  That is, we are expected
    /// at this point that all bytes in original stream have been
    /// consumed.  If not, then we have some trailing garbage in the
    /// original stream and, if so, an error is generating using the
    /// provided error.
    pub fn match_eof<E>(&mut self, err: E) -> Result<(),E> {
        if self.index == self.bytes.len() {
            Ok(())
        } else {
            Err(err)
        }
    }

    /// Read the next byte from the sequence, and move our position to
    /// the next byte in the sequence.  If no such byte is available
    /// (i.e. we have reached the end of the byte sequence), then an
    /// error is reported.
    pub fn next_u8<E:Default>(&mut self) -> Result<u8,E> {
        if self.index < self.bytes.len() {
            let next = self.bytes[self.index];
            self.index += 1;
            Ok(next)
        } else {
            Err(E::default())
        }
    }

    /// Read the next word from the sequence assuming a _big endian_
    /// representation, whilst moving our position to the next byte in
    /// the sequence.  If no such word is available (i.e. we have
    /// reached the end of the byte sequence), then an error is
    /// reported.
    pub fn next_u16<E:Default>(&mut self) -> Result<u16,E> {
        let msb = self.next_u8()?;
        let lsb = self.next_u8()?;
        Ok(u16::from_be_bytes([msb,lsb]))
    }

    /// Read the next `n` bytes from the sequence, whilst moving our
    /// position to the following byte.  If there are insufficient
    /// bytes remaining, then an error is reported.
    pub fn next_bytes<E:Default>(&mut self, length: usize) -> Result<&'a [u8],E> {
        let start = self.index;
        self.index += length;
        if self.index <= self.bytes.len() {
            Ok(&self.bytes[start..self.index])
        } else {
            Err(E::default())
        }
    }
}
