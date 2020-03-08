use {
    core::cmp::Ordering::{Equal, Greater, Less},
    littlebigint::BigUint,
    num_bigint::BigUint as NumBigUint,
    proptest::prelude::*,
};

#[test]
fn basic_equal() {
    let mut a_array = [7, 0, 0];
    let a = BigUint::from_slice(&mut a_array);

    let mut b_array = [7, 0];
    let b = BigUint::from_slice(&mut b_array);

    assert_eq!(a.cmp(&b), Equal);
}

#[test]
fn basic_greater() {
    let mut a_array = [4, 5];
    let a = BigUint::from_slice(&mut a_array);

    let mut b_array = [19];
    let b = BigUint::from_slice(&mut b_array);

    assert_eq!(a.cmp(&b), Greater);
}

#[test]
fn basic_less() {
    let mut a_array = [73];
    let a = BigUint::from_slice(&mut a_array);

    let mut b_array = [13, 53];
    let b = BigUint::from_slice(&mut b_array);

    assert_eq!(a.cmp(&b), Less);
}

proptest! {
    #[test]
    fn ordering(mut a: Vec<u8>, mut b: Vec<u8>) {
        if a.is_empty() || b.is_empty() {
            return Ok(());
        }
        assert_eq!(NumBigUint::from_bytes_le(&a).cmp(&NumBigUint::from_bytes_le(&b)), BigUint::from_slice(&mut a).cmp(&BigUint::from_slice(&mut b)))
    }
}
