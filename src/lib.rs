#[macro_use]
extern crate lazy_static;

pub mod buffer;
pub mod syllable;
pub mod byte;


use syllable::*;
use byte::Byte;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
