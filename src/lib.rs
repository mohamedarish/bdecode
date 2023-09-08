use std::{error, result};

pub mod decode;
pub mod decoder;

pub type Error = Box<dyn error::Error>;
pub type Result<T> = result::Result<T, Error>;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::add;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
