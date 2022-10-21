use std::{cell::RefCell, collections::HashSet, hash::Hash, rc::Rc};

use inquire::{type_aliases::Filter, Select};
use itertools::Itertools;
use libenigma::{reflector::Reflectors, rotor::Rotors};
use strum::IntoEnumIterator;

pub fn getReflector() -> Reflectors {
    let options: Vec<Reflectors> = Reflectors::iter().collect();

    let ans = Select::new("Select a rotor", options).prompt();

    ans.unwrap()
}

pub fn getRotors() -> () {
    let mut selectedrotors: HashSet<Rotors> = HashSet::new();
    let options: Vec<Rotors> = Rotors::iter().collect();

    for i in 0..3 {
        // Get Rotor from user
        let r: Rotors = *Select::new(
            "Select a rotor",
            options
                .iter()
                .filter(|r| !selectedrotors.contains(r))
                .collect_vec(),
        )
        .prompt()
        .unwrap();

        selectedrotors.insert(r);
        // Get char from user
    }
    ()
}
