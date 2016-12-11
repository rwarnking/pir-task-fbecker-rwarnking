use std::fmt;

fn main() {

    let b = 3;
    let obj = Swagger {value: b};

    println!("{}", obj);
    println!("{}", b.with_swag());
}

trait WithSwag<T> {
    fn with_swag(self) -> Swagger<T>;
}

impl<T> WithSwag<T> for T
    where T: fmt::Display
{
    fn with_swag(self) -> Swagger<T> {
        Swagger{ value: self }
    }
}

struct Swagger<T> {
    value: T
}

impl <T: fmt::Display> fmt::Display for Swagger<T> {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "yolo {} swag", self.value)
    }
}

