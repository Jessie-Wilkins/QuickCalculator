#[allow(dead_code)]
fn subtract_two(first: i32, second: i32) -> i32{
    first-second

}

#[cfg(test)]
mod tests {
    use crate::subtraction::subtract_two;
    #[test]
    fn subtraction_method_correctly_subtracts_parameters1() {
       let difference = subtract_two(6, 2);
        assert_eq!(difference, 4);
    }

    #[test]
    fn subtraction_method_correctly_subtracts_parameters2() {
        let difference = subtract_two(24, 12);
        assert_eq!(difference, 12)
    }
}
