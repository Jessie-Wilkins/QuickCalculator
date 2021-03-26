#[allow(dead_code)]
fn divide_two(first: i32, second: i32) -> f32{
    let quotient : f32 = first as f32 / second as f32;
    quotient
}

#[cfg(test)]
mod tests {
    use crate::division::divide_two;
    #[test]
    fn division_method_correctly_divides_parameters1() {
       let quotient = divide_two(6, 2);
        assert_eq!(quotient, 3.0);
    }

    #[test]
    fn division_method_correctly_divides_parameters2() {
        let quotient = divide_two(24, 12);
        assert_eq!(quotient, 2.0)
    }

    #[test]
    fn division_method_correctly_divides_parameters_that_gives_non_whole_quotient() {
        let quotient = divide_two(5, 2);
        assert_eq!(quotient, 2.5)
    }
}
