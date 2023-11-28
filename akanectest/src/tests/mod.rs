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
fn id_i() {
    unsafe {
        assert_eq!(ffi::idI(1), 1);
        assert_eq!(ffi::idI(2), 2);
    }
}

#[test]
fn id_f() {
    unsafe {
        assert_eq!(ffi::idF(1.0), 1.0);
        assert_eq!(ffi::idF(2.5), 2.5);
    }
}

#[test]
fn nested_id_one() {
    unsafe {
        assert_eq!(ffi::nestedIdOne(), 1);
    }
}

#[test]
fn add_i() {
    unsafe {
        assert_eq!(ffi::addI(1, 2), 3);
        assert_eq!(ffi::addI(2, 3), 5);
        assert_eq!(ffi::addI(3, 5), 8);
    }
}

#[test]
fn sub_i() {
    unsafe {
        assert_eq!(ffi::subI(1, 2), -1);
        assert_eq!(ffi::subI(5, 3), 2);
        assert_eq!(ffi::subI(4, 4), 0);
    }
}

#[test]
fn mul_i() {
    unsafe {
        assert_eq!(ffi::mulI(1, 2), 2);
        assert_eq!(ffi::mulI(2, 3), 6);
        assert_eq!(ffi::mulI(3, 5), 15);
    }
}

#[test]
fn div_i() {
    unsafe {
        assert_eq!(ffi::divI(1, 2), 0);
        assert_eq!(ffi::divI(5, 3), 1);
        assert_eq!(ffi::divI(8, 2), 4);
    }
}

#[test]
fn add_mul_i() {
    unsafe {
        assert_eq!(ffi::addMulI(1, 2, 3, 4), 14);
        assert_eq!(ffi::addMulI(2, 3, 4, 5), 26);
        assert_eq!(ffi::addMulI(2, 4, 3, 5), 23);
    }
}

#[test]
fn one_point_five() {
    unsafe {
        assert_eq!(ffi::onePointFive(), 1.5);
    }
}

#[test]
fn add_f() {
    unsafe {
        assert_eq!(ffi::addF(1.5, 2.5), 4.0);
        assert_eq!(ffi::addF(2.5, 3.5), 6.0);
        assert_eq!(ffi::addF(3.5, 5.0), 8.5);
    }
}

#[test]
fn sub_f() {
    unsafe {
        assert_eq!(ffi::subF(1.5, 2.5), -1.0);
        assert_eq!(ffi::subF(5.5, 3.5), 2.0);
        assert_eq!(ffi::subF(4.5, 4.0), 0.5);
    }
}

#[test]
fn mul_f() {
    unsafe {
        assert_eq!(ffi::mulF(1.5, 2.5), 3.75);
        assert_eq!(ffi::mulF(2.5, 3.5), 8.75);
        assert_eq!(ffi::mulF(3.5, 5.0), 17.5);
    }
}

#[test]
fn div_f() {
    unsafe {
        assert_eq!(ffi::divF(1.0, 2.0), 0.5);
        assert_eq!(ffi::divF(5.0, 2.5), 2.0);
        assert_eq!(ffi::divF(7.5, 2.5), 3.0);
    }
}

#[test]
fn add_mul_f() {
    unsafe {
        assert_eq!(ffi::addMulF(1.5, 2.5, 3.5, 4.5), 19.5);
        assert_eq!(ffi::addMulF(2.0, 3.5, 4.0, 5.5), 29.0);
        assert_eq!(ffi::addMulF(2.0, 4.0, 3.5, 5.5), 27.25);
    }
}

#[test]
fn const2_i_f() {
    unsafe {
        assert_eq!(ffi::const2IF(1, 2.0), 2.0);
        assert_eq!(ffi::const2IF(2, 1.0), 1.0);
    }
}
