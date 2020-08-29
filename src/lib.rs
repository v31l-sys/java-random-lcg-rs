#[macro_use]
extern crate lazy_static;

pub mod lcg;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcg_test() {
        assert_eq!(lcg::FORWARD1.multiplier, 0x5DEECE66D);
        assert_eq!(lcg::FORWARD4.multiplier, 0x32EB772C5F11);
    }
}
