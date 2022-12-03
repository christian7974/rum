/// Returns true iff the unsigned value `n` fits into `width` signed bits.
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    let upper_bound = (1_i64 << width as i64 - 1) - 1; 
    let lower_bound = -(1_i64 << width as i64 - 1);

    return n <= upper_bound as i64 && n >= lower_bound;
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    return n as u32 <= (1_u32 << width as u32) - 1;
}
/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
// pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    // let new_word = (word << 64 - width - lsb) >> 64 - width;
    // return new_word;
// }
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
    // isolates the right part and left part of the word excluding the part you want to get
    (word << (64 - width - lsb)) as i64 >> (64 - width)
}
/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
        // isolates the right part and left part of the word excluding the part you want to get

    (word << (64 - width - lsb)) >> (64 - width)
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    // left shift the bits by lsb, or the words so that
    // the proper bits were in the correct spots
    if fitsu(value, width){
        Some(value << lsb | word)
    } else {
        None
    }
}
/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
///
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if fitss(value, width){
        // would shift the bits by the lsb and then or the word (same thing as newu)
        Some(((value & (!(-1_i64 << width)) as i64) as u64) << lsb | word)
    } else {
        None
    }
}


#[cfg(test)]
mod tests {
    use crate::bitpack::{fitss, fitsu, getu, gets, newu, news};

    #[test]
    fn fitsu_works() {
        assert_eq!(fitsu(2, 3), true);
        assert_eq!(fitsu(4, 3), false);
    }

    #[test]
    fn fitss_works() {
        assert_eq!(fitss(3, 3), true);
        assert_eq!(fitss(-3, 3), false);
        assert_eq!(fitss(4, 3), false);
        assert_eq!(fitss(-1, 3), true);
    }

    #[test]
    fn getu_works() {
        assert_eq!(getu(0b1101, 2, 1), 0b10);
        assert_eq!(getu(0b1101, 4, 0), 0b1101);
        assert_eq!(getu(0b1101, 1, 3), 0b1);
    }

    #[test]
    fn gets_works() {
        assert_eq!(gets(0b1101, 2, 1), -2);
        assert_eq!(gets(0b1101, 4, 0), -3);
        assert_eq!(gets(0b0101, 2, 0), 1);
    }

    #[test]
    fn newu_works() {
        assert_eq!(newu(0b0000, 2, 1, 1), Some(0b0110));
        assert_eq!(newu(0b01001, 2, 1, 3), Some(0b1111));
    }

    #[test]
    fn news_works() {
        assert_eq!(news(0b0000,4,0,-3), Some(0b1101));
        assert_eq!(news(0b0000,3,1,-1), Some(0b1010));
        assert_eq!(news(0b0000,2,2,1), Some(0b0100));
        assert_eq!(news(0b0000,2,0,0), Some(0b0000));
    }
}
