use {
    crate::assert_eq_trimmed, littlebigint::BigUint, num_bigint::BigUint as NumBigUint,
    proptest::prelude::*,
};

#[test]
fn basic_multiplication() {
    let mut a_array = [42];
    let a = BigUint::from_slice(&mut a_array);

    let mut b_array = [6];
    let b = BigUint::from_slice(&mut b_array);

    let mut c_array = [7];
    let c = BigUint::from_slice(&mut c_array);

    let mut buf = [0u8];

    assert_eq!(a, b.mul(&c, &mut buf));
}

#[test]
fn basic_carry_multiplication() {
    // 412897704603
    let mut a_array = [155, 250, 158, 34, 96];
    let a = BigUint::from_slice(&mut a_array);

    // 6887253
    let mut b_array = [85, 23, 105, 0, 0];
    let b = BigUint::from_slice(&mut b_array);

    // 59951
    let mut c_array = [47, 234];
    let c = BigUint::from_slice(&mut c_array);

    let mut buf = [0u8; 6];

    assert_eq!(a, b.mul(&c, &mut buf));
}

proptest! {
    #[test]
    fn small_multiplication(a: u8, b: u8) {
        let mut a_array = [a, 0];
        let a_bigint = BigUint::from_slice(&mut a_array);

        let mut b_array = [b];
        let b_bigint = BigUint::from_slice(&mut b_array);

        let mut buf = [0, 0];

        assert_eq!((a as u16 * b as u16).to_le_bytes(), a_bigint.mul(&b_bigint, &mut buf).into_slice());
    }

    #[test]
    fn big_multiplication(mut a: Vec<u8>, mut b: Vec<u8>) {
        // Ignore zero-length inputs and prevent panicking due to overflow
        if a.len() < b.len() || a.len() == 0 {
            return Ok(());
        }

        let mut correct = (NumBigUint::from_bytes_le(&a) * NumBigUint::from_bytes_le(&b)).to_bytes_le();

        for _ in 0..(b.len() + 1) {
            a.push(0);
        }
        let len = a.len() + b.len();
        let a = BigUint::from_slice(&mut a);
        let b = BigUint::from_slice(&mut b);

        let mut buf = vec![0u8; len];

        assert_eq_trimmed(correct.as_mut_slice(), (a.mul(&b, &mut buf)).into_slice());
    }
}
/*
#[test]
fn temp() {
    let mut a_array = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ];
    let a = BigUint::from_slice(&mut a_array);

    let mut b_array = vec![0, 0];
    let b = BigUint::from_slice(&mut b_array);

    let mut buf = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    dbg!(a.mul(&b, &mut buf));

    assert_eq!
}
*/
