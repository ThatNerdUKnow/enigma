use std::{cell::RefCell, collections::HashSet, hash::Hash, rc::Rc};

use inquire::{type_aliases::Filter, Select};
use itertools::Itertools;
use libenigma::{reflector::Reflectors, rotor::Rotors};
use strum::IntoEnumIterator;

pub fn getReflector() -> Reflectors {
    let options: Vec<Reflectors> = Reflectors::iter().collect();

    let ans = Select::new("Select Reflector", options).prompt();

    ans.unwrap()
}

pub fn getRotors() -> [(Rotors, char); 3] {
    let mut selectedrotors: HashSet<Rotors> = HashSet::new();
    let options: Vec<Rotors> = Rotors::iter().collect();
    let position_options: Vec<char> = ('A'..='Z').collect_vec();

    let mut ans: Vec<(Rotors, char)> = Vec::new();
    for i in 0..3 {
        // Get Rotor from user
        let r: Rotors = *Select::new(
            &format!("Select rotor for position {}", i + 1),
            options
                .iter()
                .filter(|r| !selectedrotors.contains(r))
                .collect_vec(),
        )
        .prompt()
        .unwrap();

        selectedrotors.insert(r);
        // Get char from user

        let p: char = Select::new(
            &format!("Select position for rotor '{r}'"),
            position_options.clone(),
        )
        .prompt()
        .unwrap();

        ans.push((r, p));
    }
    ans.as_slice().try_into().unwrap()
}
