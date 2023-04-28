//! # My Crate
//! this is documentation for the thing this comment is in

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

/// Adds one to the number given
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = chapter14::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(a: i32) -> i32 {
    a + 1
}

pub mod kinds {
    pub enum PrimaryColor {
        Red, Yello, Blue
    }

    pub enum SecondaryColor {
        Orange, Green, Purple
    }
}

pub mod utils {
    use crate::kinds::*;

    pub fn mix(color1: PrimaryColor, color2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange 
    }
}

#[cfg(test)]
mod test {

    use super::*;
    
    #[test]
    fn add_one_test() {
        let x = 3;
        let y = add_one(x);
        assert_eq!(y, x + 1);
    }
}

