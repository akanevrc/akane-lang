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
