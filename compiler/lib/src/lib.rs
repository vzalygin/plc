pub mod parser;
pub mod translator;
pub mod common;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2+2;
        assert_eq!(result, 4);
    }
}
