use interface::{getReflector, getRotors};

use crate::interface::get_plugs;

mod interface;
fn main() {
    let reflector = getReflector();
    let rotors = getRotors();
    let plugs = get_plugs();

    println!("{reflector}");
    println!("{rotors:?}");
    println!("{plugs:?}");
}
