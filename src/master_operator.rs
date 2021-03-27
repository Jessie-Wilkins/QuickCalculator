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

    pub fn multi_answer_printer(tuple_answers : (i32, i32, i32, f32, i32)) -> String {
        let mut answer_string = "Sum: ".to_string()+&tuple_answers.0.to_string()+&"\n".to_string();
        answer_string += &("Difference: ".to_string()+&tuple_answers.1.to_string()+&"\n".to_string());
        answer_string += &("Product: ".to_string()+&tuple_answers.2.to_string()+&"\n".to_string());
        answer_string += &("Quotient: ".to_string()+&tuple_answers.3.to_string()+&"\n".to_string());
        answer_string += &("Remainder: ".to_string()+&tuple_answers.4.to_string());
        answer_string
    }
}


#[cfg(test)]
mod tests {
    use crate::master_operator::multi_operations::*;

    #[test]
    fn calculate_basics_can_return_all_correct_values() {
       let tuple_answers = calculate_basics(4, 2);
        assert_eq!(tuple_answers, (6, 2, 8, 2.0, 0));
    }

    #[test]
    fn multi_answer_printer_prints_all_answers() {
        let formatted_answer = multi_answer_printer(calculate_basics(4, 2));
        assert_eq!(formatted_answer, "Sum: 6\nDifference: 2\nProduct: 8\nQuotient: 2\nRemainder: 0")
    }

    #[test]
    fn multi_answer_printer_prints_all_answers_including_ones_with_decimals() {
        let formatted_answer = multi_answer_printer(calculate_basics(5, 2));
        assert_eq!(formatted_answer, "Sum: 7\nDifference: 3\nProduct: 10\nQuotient: 2.5\nRemainder: 1")
    }
}