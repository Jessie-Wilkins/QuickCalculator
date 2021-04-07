#![allow(non_snake_case)]

mod master_operator;
mod addition;
mod subtraction;
mod division;
mod multiplication;
mod modulo;

use crate::master_operator::multi_operations::multi_input_basic_calculation_processor;
use std::env;


fn main() {

    let args: Vec<String> = env::args().collect();

    let first_number = &args[1];
    let second_number = &args[2];


    println!("{}", multi_input_basic_calculation_processor(first_number, second_number));
}
