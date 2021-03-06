use std::fmt::{Debug, Display, UpperHex, LowerHex, Binary, Octal, UpperExp, LowerExp};

macro_rules! impl_swagger_print {
    ($format:expr, $trait_name:ident) => {
        impl<T: std::fmt::$trait_name> $trait_name for Swagger<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                write!(f, concat!("yolo ", $format, " swag"), self.0)
            }
        }
    };
}

fn main() {
    // Testing
    println!("Display (0.25): {}", Swagger(0.25));
    println!("Debug (0.25): {:?}", Swagger(0.25));
    println!("LowerHex (156): {:x}", Swagger(156));
    println!("UpperHex (156): {:X}", Swagger(156));
    println!("LowerExp (0.25): {:e}", Swagger(0.25));
    println!("UpperExp (0.25): {:E}", Swagger(0.25));
    println!("Binary (156): {:b}", Swagger(156));
    println!("Octal (156): {:o}", Swagger(156));
}

impl_swagger_print!("{}", Display);
impl_swagger_print!("{:?}", Debug);
impl_swagger_print!("{:X}", UpperHex);
impl_swagger_print!("{:x}", LowerHex);
impl_swagger_print!("{:b}", Binary);
impl_swagger_print!("{:o}", Octal);
impl_swagger_print!("{:e}", LowerExp);
impl_swagger_print!("{:E}", UpperExp);

struct Swagger<T>(pub T);
