#[allow(dead_code)]
mod multi_operations {
    use crate::addition::add_two;
    use crate::subtraction::subtract_two;
    use crate::multiplication::multiply_two;
    use crate::division::divide_two;
    use crate::modulo::modulo_two;

    pub fn calculate_basics(first: i32, second: i32) -> (i32, i32, i32, f32, i32){
        let sum = add_two(first, second);
        let difference = subtract_two(first, second);
        let product = multiply_two(first, second);
        let quotient = divide_two(first, second);
        let remainder = modulo_two(first, second);

        (sum, difference, product, quotient, remainder)
    }

    pub fn multi_answer_printer(tuple_answers : (i32, i32, i32, f32, i32)) {
        
    }
}


#[cfg(test)]
mod tests {
    use crate::master_operator::multi_operations::calculate_basics;
    #[test]
    fn calculate_basics_can_return_all_correct_values() {
       let tuple_answers = calculate_basics(4, 2);
        assert_eq!(tuple_answers, (6, 2, 8, 2.0, 0));
    }
}