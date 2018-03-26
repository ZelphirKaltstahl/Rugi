// use #[should_panic(expected = "<panic message>")] to catch panics in tests

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_not_works() {
        assert!(1+2 == 2, "rust cannot calculate ... or is it me?");
    }

    #[test]
    fn should_work() {
        assert_ne!(1, 2);
    }
}
