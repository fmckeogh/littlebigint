use {num_bigint::BigUint as NumBigUint, proptest::prelude::*, smolbigint::BigUint};

// Used due to API differences between the libraries; in `num_bigint` it makes sense to always
// trim leading zeros, in `smolbigint` allocations cannot be made if a number needs to be bigger
fn trim_leading_zeros(slice: &mut [u8]) -> &mut [u8] {
    let index = slice.len() - slice.iter().rev().filter(|x| x == &&0).count();
    slice.split_at_mut(index).0
}
fn assert_eq_trimmed(a: &mut [u8], b: &mut [u8]) {
    assert_eq!(trim_leading_zeros(a), trim_leading_zeros(b))
}

#[test]
fn basic_addition() {
    let mut a_array = [4];
    let a = BigUint::from_slice(&mut a_array);

    let mut b_array = [2];
    let b = BigUint::from_slice(&mut b_array);

    let mut c_array = [2];
    let c = BigUint::from_slice(&mut c_array);

    assert_eq!(a, b + c);
}

#[test]
fn basic_carry_addition() {
    let mut a_array = [1, 1];
    let a = BigUint::from_slice(&mut a_array);

    let mut b_array = [255, 0];
    let b = BigUint::from_slice(&mut b_array);

    let mut c_array = [2];
    let c = BigUint::from_slice(&mut c_array);

    assert_eq!(a, b + c);
}

proptest! {
    #[test]
    fn small_addition(a: u8, b: u8) {
        let mut a_array = [a, 0];
        let a_bigint = BigUint::from_slice(&mut a_array);

        let mut b_array = [b];
        let b_bigint = BigUint::from_slice(&mut b_array);

        assert_eq!((a as u16 + b as u16).to_le_bytes(), (a_bigint + b_bigint).to_slice());
    }

    #[test]
    fn big_addition(mut a: Vec<u8>, mut b: Vec<u8>) {
        // Ignore zero-length inputs
        if a.len() == 0 || b.len() == 0 {
            return Ok(());
        }

        // Prevents panicking due to overflow, need to find a more ergonomic solution
        if b.len() >= a.len() {
            return Ok(());
        }

        let mut correct = (NumBigUint::from_bytes_le(&a) + NumBigUint::from_bytes_le(&b)).to_bytes_le();

        let a = BigUint::from_slice(&mut a);
        let b = BigUint::from_slice(&mut b);

        assert_eq_trimmed(correct.as_mut_slice(), (a + b).to_slice());
    }
}
