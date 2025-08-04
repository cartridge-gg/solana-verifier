mod primitive_conversions;

use core::ops::{Add, Neg};
use core::str::FromStr;

use num_bigint::BigUint;
use size_of::SizeOf;

use lambdaworks_math::{
    field::{
        element::FieldElement, fields::fft_friendly::stark_252_prime_field::Stark252PrimeField,
    },
    traits::ByteConversion,
    unsigned_integer::element::UnsignedInteger,
};

/// Definition of the Field Element type.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Felt(pub(crate) FieldElement<Stark252PrimeField>);

impl SizeOf for Felt {
    fn size_of_children(&self, _context: &mut size_of::Context) {}
}

/// A non-zero [Felt].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NonZeroFelt(FieldElement<Stark252PrimeField>);

impl NonZeroFelt {
    /// Create a [NonZeroFelt] as a constant.
    /// # Safety
    /// If the value is zero will panic.
    pub const fn from_raw(value: [u64; 4]) -> Self {
        assert!(
            value[0] != 0 || value[1] != 0 || value[2] != 0 || value[3] != 0,
            "Felt is zero"
        );
        let value = Felt::from_raw(value);
        Self(value.0)
    }

    /// [Felt] constant that's equal to 1.
    pub const ONE: Self = Self::from_felt_unchecked(Felt(
        FieldElement::<Stark252PrimeField>::from_hex_unchecked("1"),
    ));

    /// [Felt] constant that's equal to 2.
    pub const TWO: Self = Self::from_felt_unchecked(Felt(
        FieldElement::<Stark252PrimeField>::from_hex_unchecked("2"),
    ));

    /// [Felt] constant that's equal to 3.
    pub const THREE: Self = Self::from_felt_unchecked(Felt(
        FieldElement::<Stark252PrimeField>::from_hex_unchecked("3"),
    ));

    /// Maximum value of [Felt]. Equals to 2^251 + 17 * 2^192.
    pub const MAX: Self =
        Self::from_felt_unchecked(Felt(FieldElement::<Stark252PrimeField>::const_from_raw(
            UnsignedInteger::from_limbs([544, 0, 0, 32]),
        )));

    /// Create a [NonZeroFelt] without checking it. If the [Felt] is indeed [Felt::ZERO]
    /// this can lead to undefined behaviour and big security issue.
    /// You should always use the [TryFrom] implementation
    pub const fn from_felt_unchecked(value: Felt) -> Self {
        Self(value.0)
    }
}

#[derive(Debug)]
pub struct FeltIsZeroError;

#[derive(Debug)]
pub struct FromStrError;

impl Felt {
    /// [Felt] constant that's equal to 0.
    pub const ZERO: Self = Self(FieldElement::<Stark252PrimeField>::from_hex_unchecked("0"));

    /// [Felt] constant that's equal to 1.
    pub const ONE: Self = Self(FieldElement::<Stark252PrimeField>::from_hex_unchecked("1"));

    /// [Felt] constant that's equal to 2.
    pub const TWO: Self = Self(FieldElement::<Stark252PrimeField>::from_hex_unchecked("2"));

    /// [Felt] constant that's equal to 3.
    pub const THREE: Self = Self(FieldElement::<Stark252PrimeField>::from_hex_unchecked("3"));

    /// Maximum value of [Felt]. Equals to 2^251 + 17 * 2^192.
    pub const MAX: Self = Self(FieldElement::<Stark252PrimeField>::const_from_raw(
        UnsignedInteger::from_limbs([544, 0, 0, 32]),
    ));

    /// 2 ** 251
    pub const ELEMENT_UPPER_BOUND: Felt = Felt::from_raw([
        576459263475450960,
        18446744073709255680,
        160989183,
        18446743986131435553,
    ]);

    /// Creates a new [Felt] from the raw internal representation.
    /// See [UnsignedInteger] to understand how it works under the hood.
    pub const fn from_raw(val: [u64; 4]) -> Self {
        Self(FieldElement::<Stark252PrimeField>::const_from_raw(
            UnsignedInteger::from_limbs(val),
        ))
    }

    pub const fn from_hex_unchecked(val: &str) -> Self {
        Self(FieldElement::<Stark252PrimeField>::from_hex_unchecked(val))
    }

    /// Creates a new [Felt] from its big-endian representation in a [u8; 32] array.
    /// This is as performant as [from_bytes_le](Felt::from_bytes_le).
    pub fn from_bytes_be(bytes: &[u8; 32]) -> Self {
        FieldElement::from_bytes_be(bytes)
            .map(Self)
            .expect("from_bytes_be shouldn't fail for these many bytes")
    }

    /// Creates a new [Felt] from its little-endian representation in a [u8; 32] array.
    /// This is as performant as [from_bytes_le](Felt::from_bytes_be).
    pub fn from_bytes_le(bytes: &[u8; 32]) -> Self {
        FieldElement::from_bytes_le(bytes)
            .map(Self)
            .expect("from_bytes_le shouldn't fail for these many bytes")
    }

    /// Creates a new [Felt] from its big-endian representation in a [u8] slice.
    /// This is as performant as [from_bytes_le](Felt::from_bytes_le_slice).
    /// All bytes in the slice are consumed, as if first creating a big integer
    /// from them, but the conversion is performed in constant space on the stack.
    pub fn from_bytes_be_slice(bytes: &[u8]) -> Self {
        // NB: lambdaworks ignores the remaining bytes when len > 32, so we loop
        // multiplying by BASE, effectively decomposing in base 2^256 to build
        // digits with a length of 32 bytes. This is analogous to splitting the
        // number `xyz` as `x * 10^2 + y * 10^1 + z * 10^0`.
        const BASE: Felt = Felt(FieldElement::<Stark252PrimeField>::const_from_raw(
            UnsignedInteger::from_limbs([
                576413109808302096,
                18446744073700081664,
                5151653887,
                18446741271209837569,
            ]),
        ));
        // Sanity check; gets removed in release builds.
        debug_assert_eq!(BASE, Felt::TWO.pow(256u32));

        let mut factor = Self::ONE;
        let mut res = Self::ZERO;
        let chunks = bytes.rchunks_exact(32);
        let remainder = chunks.remainder();

        for chunk in chunks {
            let digit =
                Self::from_bytes_be(&chunk.try_into().expect("conversion to same-sized array"));
            res += digit * factor;
            factor *= BASE;
        }

        if remainder.is_empty() {
            return res;
        }

        let mut remainder = remainder.iter().rev().cloned();
        let buf: [u8; 32] = core::array::from_fn(move |_| remainder.next().unwrap_or_default());
        let digit = Self::from_bytes_le(&buf);
        res += digit * factor;

        res
    }

    /// Creates a new [Felt] from its little-endian representation in a [u8] slice.
    /// This is as performant as [from_bytes_be](Felt::from_bytes_be_slice).
    /// All bytes in the slice are consumed, as if first creating a big integer
    /// from them, but the conversion is performed in constant space on the stack.
    pub fn from_bytes_le_slice(bytes: &[u8]) -> Self {
        // NB: lambdaworks ignores the remaining bytes when len > 32, so we loop
        // multiplying by BASE, effectively decomposing in base 2^256 to build
        // digits with a length of 32 bytes. This is analogous to splitting the
        // number `xyz` as `x * 10^2 + y * 10^1 + z * 10^0`.
        const BASE: Felt = Felt(FieldElement::<Stark252PrimeField>::const_from_raw(
            UnsignedInteger::from_limbs([
                576413109808302096,
                18446744073700081664,
                5151653887,
                18446741271209837569,
            ]),
        ));
        // Sanity check; gets removed in release builds.
        debug_assert_eq!(BASE, Felt::TWO.pow(256u32));

        let mut factor = Self::ONE;
        let mut res = Self::ZERO;
        let chunks = bytes.chunks_exact(32);
        let remainder = chunks.remainder();

        for chunk in chunks {
            let digit =
                Self::from_bytes_le(&chunk.try_into().expect("conversion to same-sized array"));
            res += digit * factor;
            factor *= BASE;
        }

        if remainder.is_empty() {
            return res;
        }

        let mut remainder = remainder.iter().cloned();
        let buf: [u8; 32] = core::array::from_fn(move |_| remainder.next().unwrap_or_default());
        let digit = Self::from_bytes_le(&buf);
        res += digit * factor;

        res
    }

    /// Converts to big-endian byte representation in a [u8] array.
    /// This is as performant as [to_bytes_le](Felt::to_bytes_le)
    pub fn to_bytes_be(&self) -> [u8; 32] {
        self.0.to_bytes_be()
    }

    /// Converts to little-endian byte representation in a [u8] array.
    /// This is as performant as [to_bytes_be](Felt::to_bytes_be)
    pub fn to_bytes_le(&self) -> [u8; 32] {
        self.0.to_bytes_le()
    }

    /// Converts to little-endian bit representation.
    pub fn to_bits_le(&self) -> [bool; 256] {
        self.0.to_bits_le()
    }

    /// Converts to big-endian bit representation.
    pub fn to_bits_be(&self) -> [bool; 256] {
        let mut bits = self.0.to_bits_le();
        bits.reverse();
        bits
    }

    /// Finite field division.
    pub fn field_div(&self, rhs: &NonZeroFelt) -> Self {
        Self((self.0 / rhs.0).unwrap())
    }

    /// Truncated quotient between `self` and `rhs`.
    pub fn floor_div(&self, rhs: &NonZeroFelt) -> Self {
        Self(FieldElement::from(
            &(self.0.representative().div_rem(&rhs.0.representative())).0,
        ))
    }

    /// Quotient and remainder between `self` and `rhs`.
    pub fn div_rem(&self, rhs: &NonZeroFelt) -> (Self, Self) {
        let (q, r) = self.0.representative().div_rem(&rhs.0.representative());
        (Self(FieldElement::from(&q)), Self(FieldElement::from(&r)))
    }

    /// Multiplicative inverse inside field.
    pub fn inverse(&self) -> Option<Self> {
        self.0.inv().map(Self).ok()
    }

    /// Finds the square root. There may be 2 roots for each square, and the lower one is returned.
    pub fn sqrt(&self) -> Option<Self> {
        let (root_1, root_2) = self.0.sqrt()?;
        Some(Self(core::cmp::min(root_1, root_2)))
    }

    /// Raises `self` to the power of 2.
    pub fn square(&self) -> Self {
        Self(self.0.square())
    }

    /// Doubles the point `self`
    pub fn double(&self) -> Self {
        Self(self.0.double())
    }

    /// Raises `self` to the power of `exponent`.
    pub fn pow(&self, exponent: impl Into<u128>) -> Self {
        Self(self.0.pow(exponent.into()))
    }

    /// Raises `self` to the power of `exponent`.
    pub fn pow_felt(&self, exponent: &Felt) -> Self {
        Self(self.0.pow(exponent.0.representative()))
    }

    /// Remainder of dividing `self` by `n` as integers.
    pub fn mod_floor(&self, n: &NonZeroFelt) -> Self {
        self.div_rem(n).1
    }

    /// Parse a hex-encoded number into `Felt`.
    pub fn from_hex(hex_string: &str) -> Result<Self, FromStrError> {
        FieldElement::from_hex(hex_string)
            .map(Self)
            .map_err(|_| FromStrError)
    }

    /// Parse a decimal-encoded number into `Felt`.
    pub fn from_dec_str(dec_string: &str) -> Result<Self, FromStrError> {
        if dec_string.starts_with('-') {
            UnsignedInteger::from_dec_str(dec_string.strip_prefix('-').unwrap())
                .map(|x| Self(FieldElement::from(&x)).neg())
                .map_err(|_| FromStrError)
        } else {
            UnsignedInteger::from_dec_str(dec_string)
                .map(|x| Self(FieldElement::from(&x)))
                .map_err(|_| FromStrError)
        }
    }

    /// Returns the internal representation of a felt and reverses it to match
    /// starknet-rs mont representation
    pub fn to_raw_reversed(&self) -> [u64; 4] {
        let mut res = self.0.to_raw().limbs;
        res.reverse();
        res
    }

    /// Returns the internal representation of a felt
    pub fn to_raw(&self) -> [u64; 4] {
        self.0.to_raw().limbs
    }
    /// Convert `self`'s representative into an array of `u64` digits,
    /// least significant digits first.
    pub fn to_le_digits(&self) -> [u64; 4] {
        let mut limbs = self.0.representative().limbs;
        limbs.reverse();
        limbs
    }

    /// Convert `self`'s representative into an array of `u64` digits,
    /// most significant digits first.
    pub fn to_be_digits(&self) -> [u64; 4] {
        self.0.representative().limbs
    }

    /// Get the inner FieldElement (for internal use)
    pub fn inner(&self) -> &FieldElement<Stark252PrimeField> {
        &self.0
    }

    /// Count the minimum number of bits needed to express `self`'s representative.
    pub fn bits(&self) -> usize {
        self.0.representative().bits_le()
    }

    pub fn to_biguint(&self) -> BigUint {
        let big_digits = self
            .to_le_digits()
            .into_iter()
            .flat_map(|limb| [limb as u32, (limb >> 32) as u32])
            .collect();
        BigUint::new(big_digits)
    }
}

/// Defaults to [Felt::ZERO].
impl Default for Felt {
    fn default() -> Self {
        Self(FieldElement::<Stark252PrimeField>::zero())
    }
}

impl AsRef<Felt> for Felt {
    fn as_ref(&self) -> &Felt {
        self
    }
}

impl From<NonZeroFelt> for Felt {
    fn from(value: NonZeroFelt) -> Self {
        Self(value.0)
    }
}

impl From<&NonZeroFelt> for Felt {
    fn from(value: &NonZeroFelt) -> Self {
        Self(value.0)
    }
}

impl AsRef<NonZeroFelt> for NonZeroFelt {
    fn as_ref(&self) -> &NonZeroFelt {
        self
    }
}

impl TryFrom<Felt> for NonZeroFelt {
    type Error = FeltIsZeroError;

    fn try_from(value: Felt) -> Result<Self, Self::Error> {
        if value == Felt::ZERO {
            Err(FeltIsZeroError)
        } else {
            Ok(Self(value.0))
        }
    }
}

impl TryFrom<&Felt> for NonZeroFelt {
    type Error = FeltIsZeroError;

    fn try_from(value: &Felt) -> Result<Self, Self::Error> {
        if *value == Felt::ZERO {
            Err(FeltIsZeroError)
        } else {
            Ok(Self(value.0))
        }
    }
}

impl FromStr for Felt {
    type Err = FromStrError;

    /// Converts a hex (0x-prefixed) or decimal string to a [Felt].
    /// e.g., '0x123abc' or '1337'.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0x") {
            Felt::from_hex(s)
        } else {
            Felt::from_dec_str(s)
        }
    }
}

impl Add<&Felt> for u64 {
    type Output = Option<u64>;

    fn add(self, rhs: &Felt) -> Option<u64> {
        const PRIME_DIGITS_BE_HI: [u64; 3] =
            [0x0800000000000011, 0x0000000000000000, 0x0000000000000000];
        const PRIME_MINUS_U64_MAX_DIGITS_BE_HI: [u64; 3] =
            [0x0800000000000010, 0xffffffffffffffff, 0xffffffffffffffff];

        // Match with the 64 bits digits in big-endian order to
        // characterize how the sum will behave.
        match rhs.to_be_digits() {
            // All digits are `0`, so the sum is simply `self`.
            [0, 0, 0, 0] => Some(self),
            // A single digit means this is effectively the sum of two `u64` numbers.
            [0, 0, 0, low] => self.checked_add(low),
            // Now we need to compare the 3 most significant digits.
            // There are two relevant cases from now on, either `rhs` behaves like a
            // substraction of a `u64` or the result of the sum falls out of range.

            // The 3 MSB only match the prime for Felt::max_value(), which is -1
            // in the signed field, so this is equivalent to substracting 1 to `self`.
            [hi @ .., _] if hi == PRIME_DIGITS_BE_HI => self.checked_sub(1),

            // For the remaining values between `[-u64::MAX..0]` (where `{0, -1}` have
            // already been covered) the MSB matches that of `PRIME - u64::MAX`.
            // Because we're in the negative number case, we count down. Because `0`
            // and `-1` correspond to different MSBs, `0` and `1` in the LSB are less
            // than `-u64::MAX`, the smallest value we can add to (read, substract its
            // magnitude from) a `u64` number, meaning we exclude them from the valid
            // case.
            // For the remaining range, we take the absolute value module-2 while
            // correcting by substracting `1` (note we actually substract `2` because
            // the absolute value itself requires substracting `1`.
            [hi @ .., low] if hi == PRIME_MINUS_U64_MAX_DIGITS_BE_HI && low >= 2 => {
                (self).checked_sub(u64::MAX - (low - 2))
            }
            // Any other case will result in an addition that is out of bounds, so
            // the addition fails, returning `None`.
            _ => None,
        }
    }
}

mod arithmetic {
    use core::{
        iter,
        ops::{self, Neg},
    };

    use super::*;

    /// Field addition. Never overflows/underflows.
    impl ops::AddAssign<Felt> for Felt {
        fn add_assign(&mut self, rhs: Felt) {
            self.0 += rhs.0
        }
    }

    /// Field addition. Never overflows/underflows.
    impl ops::AddAssign<&Felt> for Felt {
        fn add_assign(&mut self, rhs: &Felt) {
            self.0 += rhs.0
        }
    }

    /// Field addition. Never overflows/underflows.
    impl ops::Add<Felt> for Felt {
        type Output = Felt;

        fn add(self, rhs: Felt) -> Self::Output {
            Self(self.0 + rhs.0)
        }
    }

    /// Field addition. Never overflows/underflows.
    impl ops::Add<&Felt> for Felt {
        type Output = Felt;

        fn add(self, rhs: &Felt) -> Self::Output {
            Self(self.0 + rhs.0)
        }
    }

    /// Field addition. Never overflows/underflows.
    impl ops::Add<Felt> for &Felt {
        type Output = Felt;

        fn add(self, rhs: Felt) -> Self::Output {
            Felt(self.0 + rhs.0)
        }
    }

    /// Field addition. Never overflows/underflows.
    impl ops::Add<&Felt> for &Felt {
        type Output = Felt;

        fn add(self, rhs: &Felt) -> Self::Output {
            Felt(self.0 + rhs.0)
        }
    }

    /// Field addition. Never overflows/underflows.
    impl ops::Add<u64> for Felt {
        type Output = Felt;

        fn add(self, rhs: u64) -> Self::Output {
            self + Felt::from(rhs)
        }
    }

    /// Field addition. Never overflows/underflows.
    impl ops::Add<u64> for &Felt {
        type Output = Felt;

        fn add(self, rhs: u64) -> Self::Output {
            self + Felt::from(rhs)
        }
    }

    /// Field subtraction. Never overflows/underflows.
    impl ops::SubAssign<Felt> for Felt {
        fn sub_assign(&mut self, rhs: Felt) {
            self.0 = self.0 - rhs.0
        }
    }

    /// Field subtraction. Never overflows/underflows.
    impl ops::SubAssign<&Felt> for Felt {
        fn sub_assign(&mut self, rhs: &Felt) {
            self.0 = self.0 - rhs.0
        }
    }

    /// Field subtraction. Never overflows/underflows.
    impl ops::Sub<Felt> for Felt {
        type Output = Felt;

        fn sub(self, rhs: Felt) -> Self::Output {
            Self(self.0 - rhs.0)
        }
    }

    /// Field subtraction. Never overflows/underflows.
    impl ops::Sub<&Felt> for Felt {
        type Output = Felt;

        fn sub(self, rhs: &Felt) -> Self::Output {
            Self(self.0 - rhs.0)
        }
    }

    /// Field subtraction. Never overflows/underflows.
    impl ops::Sub<Felt> for &Felt {
        type Output = Felt;

        fn sub(self, rhs: Felt) -> Self::Output {
            Felt(self.0 - rhs.0)
        }
    }

    /// Field subtraction. Never overflows/underflows.
    impl ops::Sub<&Felt> for &Felt {
        type Output = Felt;

        fn sub(self, rhs: &Felt) -> Self::Output {
            Felt(self.0 - rhs.0)
        }
    }

    /// Field subtraction. Never overflows/underflows.
    #[allow(clippy::suspicious_arithmetic_impl)]
    impl ops::Sub<Felt> for u64 {
        type Output = Option<u64>;
        fn sub(self, rhs: Felt) -> Self::Output {
            self + &rhs.neg()
        }
    }

    /// Field subtraction. Never overflows/underflows.
    #[allow(clippy::suspicious_arithmetic_impl)]
    impl ops::Sub<&Felt> for u64 {
        type Output = Option<u64>;
        fn sub(self, rhs: &Felt) -> Self::Output {
            self + &rhs.neg()
        }
    }

    /// Field subtraction. Never overflows/underflows.
    impl ops::Sub<u64> for Felt {
        type Output = Felt;
        fn sub(self, rhs: u64) -> Self::Output {
            self - Self::from(rhs)
        }
    }

    /// Field subtraction. Never overflows/underflows.
    impl ops::Sub<u64> for &Felt {
        type Output = Felt;
        fn sub(self, rhs: u64) -> Self::Output {
            self - Felt::from(rhs)
        }
    }

    /// Field multiplication. Never overflows/underflows.
    impl ops::MulAssign<Felt> for Felt {
        fn mul_assign(&mut self, rhs: Felt) {
            self.0 = self.0 * rhs.0
        }
    }

    /// Field multiplication. Never overflows/underflows.
    impl ops::MulAssign<&Felt> for Felt {
        fn mul_assign(&mut self, rhs: &Felt) {
            self.0 = self.0 * rhs.0
        }
    }

    /// Field multiplication. Never overflows/underflows.
    impl ops::Mul<Felt> for Felt {
        type Output = Felt;

        fn mul(self, rhs: Felt) -> Self::Output {
            Self(self.0 * rhs.0)
        }
    }

    /// Field multiplication. Never overflows/underflows.
    impl ops::Mul<&Felt> for Felt {
        type Output = Felt;

        fn mul(self, rhs: &Felt) -> Self::Output {
            Self(self.0 * rhs.0)
        }
    }

    /// Field multiplication. Never overflows/underflows.
    impl ops::Mul<Felt> for &Felt {
        type Output = Felt;

        fn mul(self, rhs: Felt) -> Self::Output {
            Felt(self.0 * rhs.0)
        }
    }

    /// Field multiplication. Never overflows/underflows.
    impl ops::Mul<&Felt> for &Felt {
        type Output = Felt;

        fn mul(self, rhs: &Felt) -> Self::Output {
            Felt(self.0 * rhs.0)
        }
    }

    // [ops::Div] not implemented by design to prevent misuse. Use [Felt::floor_div] or
    // [Felt::field_div] instead.

    impl ops::Neg for Felt {
        type Output = Felt;

        fn neg(self) -> Self::Output {
            Self(self.0.neg())
        }
    }

    impl ops::Neg for &Felt {
        type Output = Felt;

        fn neg(self) -> Self::Output {
            Felt(self.0.neg())
        }
    }

    impl iter::Sum for Felt {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            let mut base = Self::ZERO;
            iter.for_each(|addend| base += addend);
            base
        }
    }

    impl<'a> iter::Sum<&'a Felt> for Felt {
        fn sum<I: Iterator<Item = &'a Felt>>(iter: I) -> Self {
            let mut base = Self::ZERO;
            iter.for_each(|addend| base += addend);
            base
        }
    }
}

mod formatting {

    use core::fmt;

    use super::*;

    /// Represents [Felt] in decimal by default.
    impl fmt::Display for Felt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if *self == Felt::ZERO {
                return write!(f, "0");
            }

            let mut buf = [0u8; 4 * 20];
            let mut i = buf.len() - 1;
            let mut current = self.0.representative();
            let ten = UnsignedInteger::from(10_u16);

            loop {
                let (quotient, remainder) = current.div_rem(&ten);
                let digit = remainder.limbs[3] as u8;
                buf[i] = digit + b'0';
                current = quotient;
                if current == UnsignedInteger::from(0_u16) {
                    break;
                }
                i -= 1;
            }

            // sequence of `'0'..'9'` chars is guaranteed to be a valid UTF8 string
            let s = core::str::from_utf8(&buf[i..]).unwrap();
            fmt::Display::fmt(s, f)
        }
    }

    impl fmt::Debug for Felt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
}

#[macro_export]
macro_rules! felt_nonzero {
    ($expr:expr) => {
        $crate::NonZeroFelt::from_felt_unchecked($expr)
    };
}

#[macro_export]
macro_rules! felt_try_nonzero {
    ($expr:expr) => {
        $crate::NonZeroFelt::try_from($expr)
    };
}
