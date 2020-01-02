// extern crate we're testing, same as any other code would do.
extern crate fastrs;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works2() {
        assert_eq!(2 + 3, 5);
    }
    #[test]
    fn test_add() {
        assert_eq!(fastrs::add(3, 2), 5);
    }
}
