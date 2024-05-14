fn shl(word: u64, bits: u64) -> Option<u64> {
    if bits <= 64 {
        if bits == 64 {
            Some(0)
        } else {
            Some(word << bits)
        }
    } else {
        None
    }
}

fn shls(word: i64, bits: u64) -> Option<i64> {
    if bits <= 64 {
        if bits == 64 {
            Some(0)
        } else {
            Some(word << bits)
        }
    } else {
        None
    }
}

fn shr(word: u64, bits: u64) -> Option<u64> {
    if bits <= 64 {
        if bits == 64 {
            Some(0)
        } else {
            Some(word >> bits)
        }
    } else {
        None
    }
}

// shift right arithmetic
fn sra(word: i64, mut bits: u64) -> Option<i64> {
    if bits <= 64 {
        if bits == 64 {
            bits = 63
        }
        Some(word >> bits)
    } else {
        None
    }
}

/// Determine whether a signed value fits in a field of `width` bits
///
/// # Arguments
///
/// * `n` - a signed integer value
/// * `width` - a bitfield width
///
/// # Returns
///
/// true or false
pub fn fitss(n: i64, width: u64) -> bool {
    if width >= 64 {
        true
    } else {
        sra(shls(n, 64 - width).unwrap(), 64 - width) == Some(n)
    }
}

/// Determine whether an usigned value fits in a field of `width` bits
///
/// # Arguments
///
/// * `n` - an usigned integer value
/// * `width` - a bitfield width
///
/// # Returns
///
/// true or false
pub fn fitsu(n: u64, width: u64) -> bool {
    width >= 64 || shr(n, width) == Some(0)
}

/// Retrieve a signed value from an unsigned `word`,
/// beginning at least significant bit `lsb`
/// and having `width` bits.
///
/// # Arguments
///
/// * `word` - the word from which to extract a value
/// * `width` - the number of bits in the field
/// * `lsb` - the least-significant bit of the field
///
/// # Returns
///
/// a signed value corresponding to the 2s complement representation
/// of the appropriate field of the `word`
/// or `None` if the field is impossible

pub fn gets(word: u64, width: u64, lsb: u64) -> Option<i64> {
    if width == 0 {
        Some(0)
    } else {
        let hi = lsb + width;
        if hi <= 64 {
            Some(sra(
                shls(word as i64, 64 - hi)?,
                ((64 - width) as i64).try_into().ok()?,
            )?)
        } else {
            None
        }
    }
}

/// Retrieve a signed value from an unsigned `word`,
/// beginning at least significant bit `lsb`
/// and having `width` bits.
///
/// # Arguments
///
/// * `word` - the word from which to extract a value
/// * `width` - the number of bits in the field
/// * `lsb` - the least-significant bit of the field
///
/// # Returns
///
/// a signed value corresponding to the 2s complement representation
/// of the appropriate field of the `word`
///
/// or None
/// if `lsb + width > 64`
///

pub fn getu(word: u64, width: u64, lsb: u64) -> Option<u64> {
    let hi = lsb + width;
    if hi <= 64 {
        Some(shr(shl(word, 64 - hi)?, 64 - width)?)
    } else {
        None
    }
}

/// Given an unsigned 64-bit `word`, and an unsigned `value`,
/// pack that `value` into `width` bits of the `word` starting at
/// least-significant bit `lsb`, if possible.
///
/// # Arguments
///
/// * `word` - an arbitrary unsigned 64-bit word
/// * `width` - a number of bits describing a field
/// * `lsb` - the-least significant bit of a field
/// * `value` - the unsigned value to store in the field
///
/// # Returns
///
/// an `Option<u64>` which contains the desired value at the appropriate field, if possible
/// If the value does not fit, returns `None`

pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    let hi = lsb + width;
    assert!(hi <= 64);
    if !fitsu(value, width) {
        None
    } else {
        Some(
            shl(shr(word, hi)?, hi)? // high part
             | shr(shl(word, 64 - lsb)?, 64 - lsb)? // low part
             | (value << lsb),
        ) // new value
    }
}

/// Given an unsigned 64-bit `word`, and a signed `value`,
/// pack that `value` into `width` bits of the `word` starting at
/// least-significant bit `lsb`, if possible.
///
/// # Arguments
///
/// * `word` - an arbitrary unsigned 64-bit word
/// * `width` - a number of bits describing a field
/// * `lsb` - the-least significant bit of a field
/// * `value` - the signed value to store in the field
///
/// # Returns
///
/// an `Option<u64>` which contains the desired value at the appropriate field, if possible
/// If the value does not fit, returns `None`
///

pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if !fitss(value, width) {
        None
    } else {
        Some(newu(word, width, lsb, getu(value as u64, width, 0)?)?)
    }
}
