use ast::OmlValue;

pub mod ast;

fn main() {
    match OmlValue::from_str("") {
        Ok(val) => println!("Success: {:?}", val),
        Err(err) => println!("Error: {}", err),
    }
}
