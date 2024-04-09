// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.

trait NewColor
where
    Self: Sized,
{
    fn new(red: u8, green: u8, blue: u8) -> Self;
}

trait DefaultColor: NewColor {
    fn default_red() -> Self {
        Self::new(255, 0, 0)
    }

    fn default_green() -> Self {
        Self::new(0, 255, 0)
    }

    fn default_blue() -> Self {
        Self::new(0, 0, 255)
    }

    fn default_black() -> Self {
        Self::new(0, 0, 0)
    }

    fn default_white() -> Self {
        Self::new(255, 255, 255)
    }
}

struct ColorClassicStruct {
    // Something goes here
    red: u8,
    green: u8,
    blue: u8,
}

impl NewColor for ColorClassicStruct {
    fn new(red: u8, green: u8, blue: u8) -> Self {
        ColorClassicStruct { red, green, blue }
    }
}

impl DefaultColor for ColorClassicStruct {}

/*  Something goes here */
struct ColorTupleStruct(u8, u8, u8);

impl NewColor for ColorTupleStruct {
    fn new(red: u8, green: u8, blue: u8) -> Self {
        ColorTupleStruct(red, green, blue)
    }
}

impl DefaultColor for ColorTupleStruct {}

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // Instantiate a classic c struct!
        let green = ColorClassicStruct::default_green();

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // Instantiate a tuple struct!
        let green = ColorTupleStruct::default_green();

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
