#![allow(non_snake_case)]

mod master_operator;
mod addition;
mod subtraction;
mod division;
mod multiplication;
mod modulo;

use crate::master_operator::multi_operations::multi_input_basic_calculation_processor;

fn main() {
    println!("{}", multi_input_basic_calculation_processor("5", "2"));
}
