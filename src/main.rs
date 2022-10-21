use interface::{getReflector, getRotors};

mod interface;
fn main() {
    let reflector = getReflector();
    let rotors = getRotors();

    println!("{reflector}");
    println!("{rotors:?}");
}
