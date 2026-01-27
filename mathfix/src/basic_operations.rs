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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn add_u64_ok() {
        let mut result: u64 = 0;
        let operation = Basic::add_u64(2, 23, &mut result);
        assert_eq!(25, result);
        assert_eq!(StatusCode::Ok, operation);
    }

    #[test]
    fn add_overflow() {
        let mut result: u64 = 0;
        let operation = Basic::add_u64(u64::MAX, 1, &mut result);
        assert_eq!(StatusCode::Overflow, operation);
    }

    #[test]
    fn sub_u64_ok() {
        let mut result: u64 = 0;
        let operation: StatusCode = Basic::subtract_u64(255, 2, &mut result);
        assert_eq!(253, result);
        assert_eq!(StatusCode::Ok, operation);
    }

    #[test]
    fn mul_u64_ok() {
        let mut result: u64 = 0;
        let operation: StatusCode = Basic::multiply_u64(255, 2, &mut result);
        assert_eq!(510, result);
        assert_eq!(StatusCode::Ok, operation);
    }

    #[test]
    fn mult_overflow() {
        let mut result: u64 = 0;
        let operation: StatusCode = Basic::multiply_u64(u64::MAX, 2, &mut result);
        assert_eq!(StatusCode::Overflow, operation);
    }

    #[test]
    fn div_ok() {
        let mut result: u64 = 0;
        let operation: StatusCode = Basic::divide_u64(255, 2, &mut result);
        assert_eq!(127, result);
        assert_eq!(StatusCode::Ok, operation);
    }

    #[test]
    fn div_zero() {
        let mut result: u64 = 0;
        let operation: StatusCode = Basic::divide_u64(1, 0, &mut result);
        assert_eq!(StatusCode::DivisionByZero, operation);
    }
}
