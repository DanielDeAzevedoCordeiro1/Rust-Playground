pub struct Basic;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum StatusCode {
    Ok = 0,
    DivisionByZero = 1,
    Overflow = 2,
    Underflow = 3,
}
impl Basic {
    #[unsafe(no_mangle)]
    pub extern "C" fn add_u64(number_one: u64, number_two: u64, output: *mut u64) -> StatusCode {
        match number_one.checked_add(number_two) {
            Some(value) => {
                unsafe { *output = value }
                StatusCode::Ok
            }
            None => StatusCode::Overflow,
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn subtract_u64(
        number_one: u64,
        number_two: u64,
        output: *mut u64,
    ) -> StatusCode {
        match number_one.checked_sub(number_two) {
            Some(value) => {
                unsafe { *output = value }
                StatusCode::Ok
            }
            None => StatusCode::Underflow,
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn multiply_u64(
        number_one: u64,
        number_two: u64,
        output: *mut u64,
    ) -> StatusCode {
        match number_one.checked_mul(number_two) {
            Some(value) => {
                unsafe { *output = value }
                StatusCode::Ok
            }
            None => StatusCode::Overflow,
        }
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn divide_u64(number_one: u64, number_two: u64, output: *mut u64) -> StatusCode {
        match number_one.checked_div(number_two) {
            Some(value) => {
                unsafe { *output = value }
                StatusCode::Ok
            }
            None => StatusCode::DivisionByZero,
        }
    }
}
