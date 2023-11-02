mod ffi;

#[test]
fn zero() {
    unsafe {
        assert_eq!(ffi::zero(), 0);
    }
}

#[test]
fn one() {
    unsafe {
        assert_eq!(ffi::one(), 1);
    }
}

#[test]
fn id() {
    unsafe {
        assert_eq!(ffi::id(1), 1);
        assert_eq!(ffi::id(2), 2);
    }
}

#[test]
fn nested_id_one() {
    unsafe {
        assert_eq!(ffi::nestedIdOne(), 1);
    }
}

#[test]
fn add_values() {
    unsafe {
        assert_eq!(ffi::addValues(1, 2), 3);
        assert_eq!(ffi::addValues(2, 3), 5);
        assert_eq!(ffi::addValues(3, 5), 8);
    }
}

#[test]
fn sub_values() {
    unsafe {
        assert_eq!(ffi::subValues(1, 2), -1);
        assert_eq!(ffi::subValues(5, 3), 2);
        assert_eq!(ffi::subValues(4, 4), 0);
    }
}

#[test]
fn mul_values() {
    unsafe {
        assert_eq!(ffi::mulValues(1, 2), 2);
        assert_eq!(ffi::mulValues(2, 3), 6);
        assert_eq!(ffi::mulValues(3, 5), 15);
    }
}

#[test]
fn div_values() {
    unsafe {
        assert_eq!(ffi::divValues(1, 2), 0);
        assert_eq!(ffi::divValues(5, 3), 1);
        assert_eq!(ffi::divValues(8, 2), 4);
    }
}

#[test]
fn add_mul_values() {
    unsafe {
        assert_eq!(ffi::addMulValues(1, 2, 3, 4), 14);
        assert_eq!(ffi::addMulValues(2, 3, 4, 5), 26);
        assert_eq!(ffi::addMulValues(2, 4, 3, 5), 23);
    }
}
