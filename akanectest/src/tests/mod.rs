mod ffi;

#[test]
fn sum() {
    unsafe {
        assert_eq!(ffi::sum(2, 3), 5);
    }
}
