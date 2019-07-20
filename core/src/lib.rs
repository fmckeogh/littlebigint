#![no_std]

use core::{cmp::PartialEq, ops::AddAssign};

#[derive(Debug)]
pub struct BigUint<'a>(&'a mut [u8]);

impl<'a> BigUint<'a> {
    pub fn from_slice(slice: &'a mut [u8]) -> Self {
        Self(slice)
    }

    pub fn to_slice(self) -> &'a [u8] {
        self.0
    }
}

impl<'a> PartialEq for BigUint<'a> {
    fn eq(&self, rhs: &BigUint<'a>) -> bool {
        self.0 == rhs.0
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
