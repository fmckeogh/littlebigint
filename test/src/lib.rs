#[cfg(test)]
mod tests {
    use {proptest::prelude::*, smolbigint::BigUint};

    #[test]
    fn basic_addition() {
        let mut a_array = [4];
        let a = BigUint::from_slice(&mut a_array);

        let mut b_array = [2];
        let mut b = BigUint::from_slice(&mut b_array);

        let mut c_array = [2];
        let c = BigUint::from_slice(&mut c_array);

        b += c;

        assert_eq!(a, b);
    }

    #[test]
    fn basic_carry_addition() {
        let mut a_array = [1, 1];
        let a = BigUint::from_slice(&mut a_array);

        let mut b_array = [255, 0];
        let mut b = BigUint::from_slice(&mut b_array);

        let mut c_array = [2];
        let c = BigUint::from_slice(&mut c_array);

        b += c;

        assert_eq!(a, b);
    }

    proptest! {
        #[test]
        fn small_addition(a: u8, b: u8) {
            let mut a_array = [a, 0];
            let mut a_bigint = BigUint::from_slice(&mut a_array);

            let mut b_array = [b];
            let b_bigint = BigUint::from_slice(&mut b_array);

            a_bigint += b_bigint;

            assert_eq!((a as u16 + b as u16).to_le_bytes(), a_bigint.to_slice());
        }
    }
}
