#[allow(dead_code)]


pub mod multi_operations {
    pub use crate::addition::add_two;
    pub use crate::subtraction::subtract_two;
    pub use crate::multiplication::multiply_two;
    pub use crate::division::divide_two;
    pub use crate::modulo::modulo_two;

    pub fn calculate_basics(first: i32, second: i32) -> (i32, i32, i32, f32, i32){
        let sum = add_two(first, second);
        let difference = subtract_two(first, second);
        let product = multiply_two(first, second);
        let quotient = divide_two(first, second);
        let remainder = modulo_two(first, second);

        (sum, difference, product, quotient, remainder)
    }

    pub fn calculate_basics_switched(first: i32, second: i32) -> (i32, i32, i32, i32, f32, f32, i32, i32) {
        let sum = add_two(first, second);
        let difference1 = subtract_two(first, second);
        let difference2 = subtract_two(second, first);
        let product = multiply_two(first, second);
        let quotient1 = divide_two(first, second);
        let quotient2 = divide_two(second, first);
        let remainder = modulo_two(first, second);

        (sum, difference1, difference2, product, quotient1, quotient2, remainder)
    }

    pub fn multi_answer_printer(tuple_answers : (i32, i32, i32, f32, i32)) -> String {
        let mut answer_string = "Sum: ".to_string()+&tuple_answers.0.to_string()+&"\n".to_string();
        answer_string += &("Difference: ".to_string()+&tuple_answers.1.to_string()+&"\n".to_string());
        answer_string += &("Product: ".to_string()+&tuple_answers.2.to_string()+&"\n".to_string());
        answer_string += &("Quotient: ".to_string()+&tuple_answers.3.to_string()+&"\n".to_string());
        answer_string += &("Remainder: ".to_string()+&tuple_answers.4.to_string());
        answer_string
    }

    pub fn convert_strings_to_ints(first: &str, second: &str) -> [i32; 2] {
        
        [first.to_string().parse::<i32>().unwrap(), second.to_string().parse::<i32>().unwrap()]
    }

    pub fn is_input_a_number(input : &str) -> bool {
        input.chars().all(char::is_numeric)
    }

    pub fn multi_input_basic_calculation_processor(first: &str, second: &str) -> String {
        if is_input_a_number(first) && is_input_a_number(second) {
            let int_array = convert_strings_to_ints(first, second);
            let answer_tuple = calculate_basics(int_array[0], int_array[1]);
            multi_answer_printer(answer_tuple)
        }
        else {
            "The inputs given were not integers. Please input integers.".to_string()
        }
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

    #[test]
    fn multi_user_strings_are_converted_to_numbers() {
        let integer_array = convert_strings_to_ints("4", "2");
        assert_eq!(integer_array, [4, 2]);
    }

    #[test]
    fn multi_user_input_validation_returns_false_when_input_is_not_a_number() {
        let is_number = is_input_a_number("t");
        assert_eq!(is_number, false);
    }

    #[test]
    fn multi_user_input_processing_returns_string_answers() {
        let formatted_answer = multi_input_basic_calculation_processor("5", "2");
        assert_eq!(formatted_answer, "Sum: 7\nDifference: 3\nProduct: 10\nQuotient: 2.5\nRemainder: 1")
    }

    #[test]
    fn calculate_basics_switched_can_return_all_correct_values_for_switched_and_unswitched_answers() {
        let tuple_answers = calculate_basics_switched(4, 2);
        assert_eq!(tuple_answers, (6, 2, -2, 8, 2.0, 0.5, 0, 2));
    }
}