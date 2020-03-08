use littlebigint::BigUint;

#[test]
fn basic_zero_true() {
    let mut a_array = [0];
    let a = BigUint::from_slice(&mut a_array);

    assert!(a.is_zero());

    let mut a_array = [0; 1024];
    let a = BigUint::from_slice(&mut a_array);

    assert!(a.is_zero());
}

#[test]
fn basic_zero_false() {
    let mut a_array = [2];
    let a = BigUint::from_slice(&mut a_array);

    assert!(!a.is_zero());

    let mut a_array = [255, 0, 0, 0, 0, 0, 0, 0];
    let a = BigUint::from_slice(&mut a_array);

    assert!(!a.is_zero());

    let mut a_array = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3];
    let a = BigUint::from_slice(&mut a_array);

    assert!(!a.is_zero());
}
