use {
    crate::assert_eq_trimmed, littlebigint::BigUint, num_bigint::BigUint as NumBigUint,
    num_integer::Integer, proptest::prelude::*,
};

#[test]
fn basic_div_rem() {
    let mut a_array = [9];
    let a = BigUint::from_slice(&mut a_array);
    let mut b_array = [4];
    let b = BigUint::from_slice(&mut b_array);

    let mut c_array = [2];
    let c = BigUint::from_slice(&mut c_array);
    let mut d_array = [1];
    let d = BigUint::from_slice(&mut d_array);

    assert_eq!(a.div_rem(b), (c, d));
}

proptest! {
    #[test]
    fn div_rem(mut a: Vec<u8>, mut b: Vec<u8>) {
        if a.is_empty() || b.is_empty() {
            return Ok(());
        }

        let (correct_div, correct_rem) = NumBigUint::from_bytes_le(&a).div_rem(&NumBigUint::from_bytes_le(&b));
        let (div, rem) = BigUint::from_slice(&mut a).div_rem(BigUint::from_slice(&mut b));


        assert_eq_trimmed(correct_div.to_bytes_le().as_mut_slice(), div.into_slice());
        assert_eq_trimmed(correct_rem.to_bytes_le().as_mut_slice(), rem.into_slice());
    }
}
