use std::os::raw::c_long;

#[link(name = "akanectest")]
extern "C" {
    pub fn zero() -> c_long;
    pub fn one() -> c_long;
    pub fn id(x: c_long) -> c_long;
    pub fn nestedIdOne() -> c_long;
    pub fn addValues(x: c_long, y: c_long) -> c_long;
    pub fn subValues(x: c_long, y: c_long) -> c_long;
    pub fn mulValues(x: c_long, y: c_long) -> c_long;
    pub fn divValues(x: c_long, y: c_long) -> c_long;
}
