/// Identifies type which have a _maximum_ value (e.g. `u32`, etc).
pub trait Max {
    const MAX : Self;
}

/// Identifies type which have a _minimum_ value (e.g. `u32`, etc).
pub trait Min {
    const MIN : Self;
}

// ==================================================================
// U8
// ==================================================================

impl Max for u8 {
    const MAX : u8 = u8::MAX;
}

impl Min for u8 {
    const MIN : u8 = u8::MIN;
}

// ==================================================================
// U16
// ==================================================================

impl Max for u16 {
    const MAX : u16 = u16::MAX;
}

impl Min for u16 {
    const MIN : u16 = u16::MIN;
}

// ==================================================================
// U32
// ==================================================================

impl Max for u32 {
    const MAX : u32 = u32::MAX;
}

impl Min for u32 {
    const MIN : u32 = u32::MIN;
}

// ==================================================================
// U64
// ==================================================================

impl Max for u64 {
    const MAX : u64 = u64::MAX;
}

impl Min for u64 {
    const MIN : u64 = u64::MIN;
}

// ==================================================================
// U128
// ==================================================================

impl Max for u128 {
    const MAX : u128 = u128::MAX;
}

impl Min for u128 {
    const MIN : u128 = u128::MIN;
}

// ==================================================================
// USIZE
// ==================================================================

impl Max for usize {
    const MAX : usize = usize::MAX;
}

impl Min for usize {
    const MIN : usize = usize::MIN;
}
