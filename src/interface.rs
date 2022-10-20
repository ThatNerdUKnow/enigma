use inquire::Select;
use libenigma::reflector::Reflectors;
use strum::IntoEnumIterator;

pub fn getReflector() -> Reflectors {
    let options: Vec<Reflectors> = Reflectors::iter().collect();

    let ans = Select::new("Select a rotor", options).prompt();

    ans.unwrap()
}
