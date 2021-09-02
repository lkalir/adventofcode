use advent_of_code_data_rs::*;
use libc::c_char;
use num_traits::FromPrimitive;

#[no_mangle]
pub extern "C" fn get_inputs(day: u8, year: u16) -> *const c_char {
    if let (Some(d), Some(y)) = (FromPrimitive::from_u8(day), FromPrimitive::from_u16(year)) {
        get_input(y, d).as_ptr() as *const c_char
    } else {
        std::ptr::null()
    }
}
