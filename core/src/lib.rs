#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

use core::{
    cmp::{self, PartialEq},
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Debug)]
pub struct BigUint<'a>(&'a mut [u8]);

impl<'a, 'b> fmt::Display for BigUint<'a> {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        // implement once div_rem is ready
        unimplemented!()
    }
}

impl<'a, 'b, 'c> BigUint<'a> {
    pub fn from_slice(slice: &'a mut [u8]) -> Self {
        Self(slice)
    }

    pub fn into_slice(self) -> &'a mut [u8] {
        self.0
    }

    pub fn mul(mut self, val: &BigUint<'b>, buf: &'c mut [u8]) -> Self {
        self.mul_assign(val, buf);
        self
    }

    pub fn mul_assign(&mut self, val: &BigUint<'b>, buf: &'c mut [u8]) {
        // Zero buf just in case
        for byte in buf.iter_mut() {
            *byte = 0;
        }

        // buf += self * val
        mac3(buf, self.0, val.0);

        // Zero self.0
        for byte in self.0.iter_mut() {
            *byte = 0;
        }

        // Copy buf into self.0
        let index = buf.len() - buf.iter().rev().take_while(|x| x == &&0).count();
        for (i, byte) in buf.iter_mut().enumerate().take(index) {
            self.0[i] = *byte;
        }
    }
}

impl<'a> PartialEq for BigUint<'a> {
    fn eq(&self, rhs: &BigUint<'a>) -> bool {
        self.0 == rhs.0
    }
}

impl<'a, 'b> Add<BigUint<'b>> for BigUint<'a> {
    type Output = BigUint<'a>;

    fn add(mut self, rhs: BigUint<'b>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<'a, 'b> AddAssign<BigUint<'b>> for BigUint<'a> {
    fn add_assign(&mut self, rhs: BigUint<'b>) {
        if self.0.len() < rhs.0.len() {
            panic!(
                "Length of BigUint on RHS ({}) is greater than LHS ({}) of AddAssign",
                rhs.0.len(),
                self.0.len()
            );
        }

        let (a_lo, a_hi) = self.0.split_at_mut(rhs.0.len());

        let mut carry = 0u16;
        for (a, b) in a_lo.iter_mut().zip(rhs.0) {
            *a = adc(*a, *b, &mut carry);
        }

        if carry != 0 {
            for a in a_hi {
                *a = adc(*a, 0, &mut carry);
                if carry == 0 {
                    break;
                }
            }
            if carry != 0 {
                panic!("Overflowed during AddAssign by {}", carry);
            }
        }
    }
}

impl<'a, 'b> Sub<BigUint<'b>> for BigUint<'a> {
    type Output = BigUint<'a>;

    fn sub(mut self, rhs: BigUint<'b>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<'a, 'b> SubAssign<BigUint<'b>> for BigUint<'a> {
    fn sub_assign(&mut self, rhs: BigUint<'b>) {
        let mut borrow = 0;

        let len = cmp::min(self.0.len(), rhs.0.len());
        let (a_lo, a_hi) = self.0.split_at_mut(len);
        let (b_lo, b_hi) = rhs.0.split_at(len);

        for (a, b) in a_lo.iter_mut().zip(b_lo) {
            *a = sbb(*a, *b, &mut borrow);
        }

        if borrow != 0 {
            for a in a_hi {
                *a = sbb(*a, 0, &mut borrow);
                if borrow == 0 {
                    break;
                }
            }
        }

        // note: we're _required_ to fail on underflow
        assert!(
            borrow == 0 && b_hi.iter().all(|x| *x == 0),
            "Underflowed during SubAssign"
        );
    }
}

// Add with carry
fn adc(a: u8, b: u8, acc: &mut u16) -> u8 {
    *acc += u16::from(a);
    *acc += u16::from(b);

    let lo = *acc as u8;
    *acc >>= 8;
    lo
}

// Subtract with borrow
fn sbb(a: u8, b: u8, acc: &mut i32) -> u8 {
    *acc += i32::from(a);
    *acc -= i32::from(b);
    let lo = *acc as u8;
    *acc >>= 8;
    lo
}

// Multiply-Accumulate with carry
fn mac(a: u8, b: u8, c: u8, acc: &mut u16) -> u8 {
    *acc += u16::from(a);
    *acc += u16::from(b) * u16::from(c);

    let lo = *acc as u8;
    *acc >>= 8;
    lo
}

// acc += b * c
fn mac_digit(acc: &mut [u8], b: &[u8], c: u8) {
    if c == 0 {
        return;
    }

    let mut carry = 0;
    let (a_lo, a_hi) = acc.split_at_mut(b.len());

    for (a, &b) in a_lo.iter_mut().zip(b) {
        *a = mac(*a, b, c, &mut carry);
    }

    let mut a = a_hi.iter_mut();
    while carry != 0 {
        let a = a.next().expect("carry overflow during multiplication!");
        *a = adc(*a, 0, &mut carry);
    }
}

// acc += b * c
fn mac3(acc: &mut [u8], b: &[u8], c: &[u8]) {
    let (x, y) = if b.len() < c.len() { (b, c) } else { (c, b) };
    for (i, xi) in x.iter().enumerate() {
        mac_digit(&mut acc[i..], y, *xi);
    }
}

#[cfg(test)]
mod tests {
    use {super::*, num_bigint::BigUint as NumBigUint, proptest::prelude::*, std::prelude::v1::*};

    fn trim_leading_zeros(slice: &mut [u8]) -> &mut [u8] {
        let index = slice.len() - slice.iter().rev().take_while(|x| x == &&0).count();
        slice.split_at_mut(index).0
    }

    proptest! {
        #[test]
        fn mac_digit_prop(a: Vec<u8>, b: Vec<u8>, c: u8) {
            if a.len() < b.len() || a.len() == 0 {
                return Ok(());
            }

            let correct = {
                let mut acc = NumBigUint::from_bytes_le(&(a.clone()));
                acc += NumBigUint::from_bytes_le(&b) * c;
                acc.to_bytes_le()
            };

            let mut a = a.clone();

            // avoid overflows
            a.push(0);

            mac_digit(&mut a, &b, c);

            assert_eq!(correct, trim_leading_zeros(&mut a));
        }

        #[test]
        fn mac3_prop(mut a: Vec<u8>, b: Vec<u8>, c: Vec<u8>) {
            if a.len() < (b.len() + c.len()) {
                return Ok(());
            }

            let correct = {
                let mut acc = NumBigUint::from_bytes_le(&a);
                acc += NumBigUint::from_bytes_le(&b) * NumBigUint::from_bytes_le(&c);
                acc.to_bytes_le()
            };

            // avoid overflows
            a.push(0);

            mac3(&mut a, &b, &c);

            assert_eq!(correct, trim_leading_zeros(&mut a))
        }
    }
}
