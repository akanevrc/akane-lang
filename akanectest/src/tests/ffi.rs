use std::ffi::{
    c_double,
    c_longlong,
};

#[link(name = "akanectest")]
extern "C" {
    pub fn zero() -> c_longlong;
    pub fn one() -> c_longlong;
    pub fn id(x: c_longlong) -> c_longlong;
    pub fn nestedIdOne() -> c_longlong;
    pub fn addValues(x: c_longlong, y: c_longlong) -> c_longlong;
    pub fn subValues(x: c_longlong, y: c_longlong) -> c_longlong;
    pub fn mulValues(x: c_longlong, y: c_longlong) -> c_longlong;
    pub fn divValues(x: c_longlong, y: c_longlong) -> c_longlong;
    pub fn addMulValues(x: c_longlong, y: c_longlong, z: c_longlong, w: c_longlong) -> c_longlong;
    pub fn pipelineAddValues(x: c_longlong, y: c_longlong) -> c_longlong;
    pub fn onePointFive() -> c_double;
}
