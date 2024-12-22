pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_failuers(){
        panic!("fail");
    }

    #[test]
    fn it_works1(){
        assert!(true);
    }

    #[test]
    fn it_failuers1(){
        assert!(false);
    }
}
