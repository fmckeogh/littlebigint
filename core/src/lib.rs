#![no_std]

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

impl<'a> BigUint<'a> {
    pub fn from_slice(slice: &'a mut [u8]) -> Self {
        Self(slice)
    }

    pub fn to_slice(self) -> &'a mut [u8] {
        self.0
    }
}

impl<'a> PartialEq for BigUint<'a> {
    fn eq(&self, rhs: &BigUint<'a>) -> bool {
        self.0 == rhs.0
    }
}

impl<'a> Add for BigUint<'a> {
    type Output = BigUint<'a>;

    fn add(mut self, rhs: BigUint<'a>) -> Self::Output {
        self += rhs;
        self
    }
}

impl<'a> AddAssign for BigUint<'a> {
    fn add_assign(&mut self, rhs: BigUint<'a>) {
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

// Add with carry
fn adc(a: u8, b: u8, acc: &mut u16) -> u8 {
    *acc += u16::from(a);
    *acc += u16::from(b);

    let lo = *acc as u8;
    *acc >>= 8;
    lo
}

impl<'a> Sub for BigUint<'a> {
    type Output = BigUint<'a>;

    fn sub(mut self, rhs: BigUint<'a>) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<'a> SubAssign for BigUint<'a> {
    fn sub_assign(&mut self, rhs: BigUint<'a>) {
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

// Subtract with borrow
fn sbb(a: u8, b: u8, acc: &mut i32) -> u8 {
    *acc += i32::from(a);
    *acc -= i32::from(b);
    let lo = *acc as u8;
    *acc >>= 8;
    lo
}
