#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }

    #[test]
    fn check_two() {
        assert!(1+1 == 2);
    }

    #[test]
    #[should_panic]
    fn check_three() {
        assert!(3 == 4);
    }
}