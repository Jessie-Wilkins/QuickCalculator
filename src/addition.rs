fn addTwo(first: i32, second: i32) -> i32{
    let sum = first+second;
    sum
}

#[cfg(test)]
mod tests {
    use crate::addition::addTwo;
    #[test]
    fn addition_method_correctly_adds_parameters1() {
       let sum = addTwo(2, 2);
        assert_eq!(sum, 4);
    }
}
