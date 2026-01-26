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

