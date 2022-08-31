use std::fmt::Write;

/// A simple trait allowing something to be converted into a hex
/// string.
pub trait ToHexString {
    fn to_hex_string(&self) -> String;
}

/// A default implementation for byte slices.
impl ToHexString for [u8] {
    fn to_hex_string(&self) -> String {
        let size = 2 + (2 * self.len());
        let mut hexstr = String::with_capacity(size);
        // Prepend "0x"
        write!(hexstr,"0x").unwrap();
        // Write each byte
        for b in self { write!(hexstr, "{:02x}", b).unwrap(); }
        // Done
        hexstr
    }
}
