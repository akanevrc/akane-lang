use std::ffi::{
    c_double,
    c_longlong,
};

#[link(name = "akanectest")]
extern "C" {
    pub fn zero() -> c_longlong;
    pub fn one() -> c_longlong;
    pub fn idI(x: c_longlong) -> c_longlong;
    pub fn idF(x: c_double) -> c_double;
    pub fn nestedIdOne() -> c_longlong;
    pub fn addI(x: c_longlong, y: c_longlong) -> c_longlong;
    pub fn subI(x: c_longlong, y: c_longlong) -> c_longlong;
    pub fn mulI(x: c_longlong, y: c_longlong) -> c_longlong;
    pub fn divI(x: c_longlong, y: c_longlong) -> c_longlong;
    pub fn addMulI(x: c_longlong, y: c_longlong, z: c_longlong, w: c_longlong) -> c_longlong;
    pub fn onePointFive() -> c_double;
    pub fn addF(x: c_double, y: c_double) -> c_double;
    pub fn subF(x: c_double, y: c_double) -> c_double;
    pub fn mulF(x: c_double, y: c_double) -> c_double;
    pub fn divF(x: c_double, y: c_double) -> c_double;
    pub fn addMulF(x: c_double, y: c_double, z: c_double, w: c_double) -> c_double;
}
