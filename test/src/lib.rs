#![cfg(test)]

mod add;
mod cmp;
mod div_rem;
mod mul;
mod shl;
mod shr;
mod sub;
mod zero;

// Used due to API differences between the libraries; in `num_bigint` it makes sense to always
// trim leading zeros, in `littlebigint` allocations cannot be made if a number needs to be bigger
fn trim_leading_zeros(slice: &mut [u8]) -> &mut [u8] {
    let index = slice.len() - slice.iter().rev().take_while(|x| x == &&0).count();
    slice.split_at_mut(index).0
}

pub fn assert_eq_trimmed(a: &mut [u8], b: &mut [u8]) {
    assert_eq!(trim_leading_zeros(a), trim_leading_zeros(b))
}
