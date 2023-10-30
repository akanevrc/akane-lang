use std::os::raw::c_long;

#[link(name = "akanectest")]
extern "C" {
    pub fn sum(a: c_long, b: c_long) -> c_long;
}
