#[macro_use]
extern crate nom;

mod parse;


#[derive(Debug, PartialEq, Eq)]
pub enum Format {
    AsciiV1,
    BinaryLittleEndianV1,
    BinaryBigEndianV1,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
