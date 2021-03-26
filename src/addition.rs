#[allow(dead_code)]
pub fn add_two(first: i32, second: i32) -> i32{
    first+second
}

#[cfg(test)]
mod tests {
    use crate::addition::add_two;
    #[test]
    fn addition_method_correctly_adds_parameters1() {
       let sum = add_two(2, 2);
        assert_eq!(sum, 4);
    }

    #[test]
    fn addition_method_correctly_adds_parameters2() {
        let sum = add_two(5, 7);
        assert_eq!(sum, 12)
    }
}
