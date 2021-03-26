#[allow(dead_code)]
pub fn multiply_two(first: i32, second: i32) -> i32{
    let product = first*second;
    product
}

#[cfg(test)]
mod tests {
    use crate::multiplication::multiply_two;
    #[test]
    fn division_method_correctly_multiplies_parameters1() {
       let product = multiply_two(6, 2);
        assert_eq!(product, 12);
    }

    #[test]
    fn division_method_correctly_multiplies_parameters2() {
        let product = multiply_two(24, 12);
        assert_eq!(product, 288)
    }

}
