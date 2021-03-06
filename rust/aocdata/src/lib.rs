use advent_of_code_data_rs::*;
use libc::{c_char, c_int};
use num_traits::FromPrimitive;

#[no_mangle]
pub extern "C" fn get_inputs(day: c_int, year: c_int) -> *const c_char {
    if let (Some(d), Some(y)) = (FromPrimitive::from_i32(day), FromPrimitive::from_i32(year)) {
        get_input(y, d).map_or_else(std::ptr::null, |p| p.as_ptr() as *const c_char)
    } else {
        std::ptr::null()
    }
}
