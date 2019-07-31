use {
    crate::assert_eq_trimmed, num_bigint::BigUint as NumBigUint, proptest::prelude::*,
    smolbigint::BigUint,
};

#[test]
fn basic_subtraction() {
    let mut a_array = [0];
    let a = BigUint::from_slice(&mut a_array);

    let mut b_array = [2];
    let b = BigUint::from_slice(&mut b_array);

    let mut c_array = [2];
    let c = BigUint::from_slice(&mut c_array);

    assert_eq!(a, b - c);
}

#[test]
fn basic_carry_subtraction() {
    let mut a_array = [254, 0];
    let a = BigUint::from_slice(&mut a_array);

    let mut b_array = [5, 1];
    let b = BigUint::from_slice(&mut b_array);

    let mut c_array = [7];
    let c = BigUint::from_slice(&mut c_array);

    assert_eq!(a, b - c);
}

proptest! {
    #[test]
    fn small_subtraction(a: u8, b: u8) {
        if b > a {
            return Ok(());
        }

        let mut a_array = [a, 0];
        let a_bigint = BigUint::from_slice(&mut a_array);

        let mut b_array = [b];
        let b_bigint = BigUint::from_slice(&mut b_array);

        assert_eq!((a as u16 - b as u16).to_le_bytes(), (a_bigint - b_bigint).into_slice());
    }

    #[test]
    fn big_subtraction(mut a: Vec<u8>, mut b: Vec<u8>) {
        if a.len() == 0 || b.len() == 0 {
            return Ok(());
        }

        if NumBigUint::from_bytes_le(&b) > NumBigUint::from_bytes_le(&a) {
            return Ok(());
        }

        let mut correct = (NumBigUint::from_bytes_le(&a) - NumBigUint::from_bytes_le(&b)).to_bytes_le();

        let a = BigUint::from_slice(&mut a);
        let b = BigUint::from_slice(&mut b);

        assert_eq_trimmed(correct.as_mut_slice(), (a - b).into_slice());
    }
}
