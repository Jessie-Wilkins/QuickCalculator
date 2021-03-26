#[allow(dead_code)]
pub fn modulo_two(first: i32, second: i32) -> i32{
    let remainder = first%second;
    remainder
}

#[cfg(test)]
mod tests {
    use crate::modulo::modulo_two;
    #[test]
    fn division_method_correctly_multiplies_parameters1() {
       let remainder = modulo_two(6, 2);
        assert_eq!(remainder, 0);
    }

    #[test]
    fn division_method_correctly_multiplies_parameters2() {
        let remainder = modulo_two(25, 12);
        assert_eq!(remainder, 1)
    }

}
