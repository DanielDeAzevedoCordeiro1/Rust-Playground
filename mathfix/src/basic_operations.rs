pub struct Basic;

#[derive(Debug)]
pub enum BasicErrors {
    DivisionByZero,
    Overflow,
    Underflow
}

impl Basic {

    pub fn add_usize(number_one: usize, number_two: usize) -> Result<usize, BasicErrors> {
        number_one
            .checked_add(number_two)
            .ok_or(BasicErrors::Overflow)
    }

    pub fn subtract_usize(number_one: usize, number_two: usize) -> Result<usize, BasicErrors>{
        number_one
            .checked_sub(number_two)
            .ok_or(BasicErrors::Underflow)
    }

    pub fn multiply_usize(number_one: usize, number_two: usize) -> Result<usize, BasicErrors>{
        number_one
            .checked_mul(number_two)
            .ok_or(BasicErrors::Overflow)
    }

    pub fn divide_usize(number_one: usize, number_two: usize) -> Result<usize, BasicErrors>{
        number_one
            .checked_div(number_two)
            .ok_or(BasicErrors::DivisionByZero)
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn add_ok() {
        let result: Result<usize, BasicErrors> = Basic::add_usize(2,23);
        match result {
            Ok(value) => assert_eq!(25, value),
            Err(_) => panic!("Erro ao somar usize")
        }
    }

    #[test]
    fn add_overflow() {
        let result: Result<usize, BasicErrors> = Basic::add_usize(usize::MAX,1);
        match result {
            Err(BasicErrors::Overflow) => {  }
            _ => panic!("Erro desconhecido")
        }
    }

    #[test]
    fn sub_ok() {
        let result: Result<usize, BasicErrors> = Basic::subtract_usize(255,2);
        match result {
            Ok(value) => assert_eq!(253, value),
            Err(_) => panic!("Erro ao subtrair usize")
        }
    }

    #[test]
    fn sub_underflow() {
        let result: Result<usize, BasicErrors> = Basic::subtract_usize(0,1);
        match result {
            Err(BasicErrors::Underflow) => {  }
            _ => panic!("Erro desconhecido")
        }
    }

    #[test]
    fn mul_ok() {
        let result: Result<usize, BasicErrors> = Basic::multiply_usize(255,2);
        match result {
            Ok(value) => assert_eq!(510, value),
            Err(_) => panic!("Erro ao multiplicar usize")
        }
    }

    #[test]
    fn mult_overflow() {
        let result: Result<usize, BasicErrors> = Basic::multiply_usize(usize::MAX,2);
        match result {
            Err(BasicErrors::Overflow) => {  }
            _ => panic!("Erro desconhecido")
        }
    }

    #[test]
    fn div_ok() {
        let result: Result<usize, BasicErrors> = Basic::divide_usize(255,2);
        match result {
            Ok(value) => assert_eq!(127, value),
            Err(_) => panic!("Erro ao dividir usize")
        }
    }

    #[test]
    fn div_zero() {
        let result: Result<usize, BasicErrors> = Basic::divide_usize(1,0);
        match result {
            Err(BasicErrors::DivisionByZero) => {  }
            _ => panic!("Erro desconhecido")
        }
    }
}
