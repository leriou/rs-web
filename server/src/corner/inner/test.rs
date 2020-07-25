pub fn test_succ() {
    "succ"
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        assert_eq!(test_succ(), "suc")
    }
}
