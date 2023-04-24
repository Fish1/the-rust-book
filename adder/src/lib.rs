#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn it_no_works() {
        let result = 2 + 5;
        assert_eq!(result, 4);
    }
    #[test]
    fn panic_test() {
        panic!("Make this test fail!");
    }
}
