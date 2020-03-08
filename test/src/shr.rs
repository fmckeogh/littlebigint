use {
    crate::assert_eq_trimmed, littlebigint::BigUint, num_bigint::BigUint as NumBigUint,
    proptest::prelude::*,
};

#[test]
fn basic_shiftright() {
    let mut a_array = [1];
    let a = BigUint::from_slice(&mut a_array);

    let mut b_array = [2];
    let b = BigUint::from_slice(&mut b_array);

    assert_eq!(a, b >> 1);
}

#[test]
fn basic_carry_shiftright() {
    let mut a_array = [123, 0];
    let a = BigUint::from_slice(&mut a_array);

    let mut b_array = [96, 15];
    let b = BigUint::from_slice(&mut b_array);

    assert_eq!(a, b >> 5);
}

#[test]
fn basic_carry_large_shiftright() {
    let mut a_array = [
        213, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
    ];
    let a = BigUint::from_slice(&mut a_array);

    let mut b_array = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 80, 13,
    ];
    let b = BigUint::from_slice(&mut b_array);

    assert_eq!(a, b >> 492);
}

proptest! {
    #[test]
    fn small_shiftright(a: u8, b: usize) {
        if b > 8 {
            return Ok(());
        }

        let mut a_array = [0, a];
        let a_bigint = BigUint::from_slice(&mut a_array);

        assert_eq!(((a as u16) >> b).to_le_bytes(), (a_bigint >> b).into_slice());
    }

    #[test]
    fn big_shiftright(mut a: Vec<u8>, mut b: usize) {
        if a.is_empty() || b > 100_000_000 {
            return Ok(());
        }

        let correct = NumBigUint::from_bytes_le(&a) >> b;

        let a = BigUint::from_slice(&mut a);

        assert_eq_trimmed(correct.to_bytes_le().as_mut_slice(), (a >> b).into_slice());
    }
}
