use {
    crate::assert_eq_trimmed, littlebigint::BigUint, num_bigint::BigUint as NumBigUint,
    proptest::prelude::*,
};

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

        assert_eq!((a as u16 + b as u16).to_le_bytes(), (a_bigint + b_bigint).into_slice());
    }

    #[test]
    fn big_addition(mut a: Vec<u8>, mut b: Vec<u8>) {
        // Ignore zero-length inputs
        if a.is_empty() || b.is_empty() {
            return Ok(());
        }

        // Prevents panicking due to overflow, need to find a more ergonomic solution
        if b.len() >= a.len() {
            return Ok(());
        }

        let mut correct = (NumBigUint::from_bytes_le(&a) + NumBigUint::from_bytes_le(&b)).to_bytes_le();

        let a = BigUint::from_slice(&mut a);
        let b = BigUint::from_slice(&mut b);

        assert_eq_trimmed(correct.as_mut_slice(), (a + b).into_slice());
    }
}
