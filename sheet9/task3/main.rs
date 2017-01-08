use std::collections::HashMap;

macro_rules! hash_map {
    ($( $key:expr => $value:expr ),*) => {
        {
            let mut m = HashMap::new();
            $( m.insert($key, $value); )*
            m
        }
    };
}

fn main() {
    let one = hash_map!("lala" => 3, "ladawd" => 5);
    println!("(str, int):");
    for (key, value) in one {
        println!("\t{:?} => {:?}", key, value);
    }

    let two = hash_map!(423 => "c", 32000 => "o");
    println!("(int, char):");
    for (key, value) in two {
        println!("\t{:?} => {:?}", key, value);
    }

    let three: HashMap<i64, Vec<i32>> 
      = hash_map!(-3000000000000 => vec![1, 2], -1 => vec![4, -1, 6], -79 => vec![]);
    println!("(int, Vector):");
    for (key, value) in three {
        println!("\t{:?} => {:?}", key, value);
    }
}
