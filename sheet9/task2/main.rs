use std::fmt::{Debug, Display, UpperHex, LowerHex, Binary, Octal, UpperExp, LowerExp};

fn main() {
    impl_all();

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

struct Swagger<T>(pub T);

macro_rules! impl_swagger_print {
    ($format:expr, $trait_name:ident) => {
        impl<T: std::fmt::$trait_name> $trait_name for Swagger<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                write!(f, $format, self.0)
            }
        }
    };
}

fn impl_all() {
    impl_swagger_print!("yolo {:?} swag", Debug);
    impl_swagger_print!("yolo {} swag", Display);
    impl_swagger_print!("yolo {:X} swag", UpperHex);
    impl_swagger_print!("yolo {:x} swag", LowerHex);
    impl_swagger_print!("yolo {:b} swag", Binary);
    impl_swagger_print!("yolo {:o} swag", Octal);
    impl_swagger_print!("yolo {:e} swag", LowerExp);
    impl_swagger_print!("yolo {:E} swag", UpperExp);
}
