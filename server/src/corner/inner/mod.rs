pub mod test;
#[cfg(test)]
mod test {

    #[test]
    fn test() {
        assert_eq!(test_succ(), "suc")
    }
}
