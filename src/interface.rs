use bruh_moment::Bruh;
use clap::Parser;
use inquire::{validator::ExactLengthValidator, MultiSelect, Select};
use itertools::Itertools;
use libenigma::{
    common::Character,
    enigma::Enigma,
    plugboard::{Plug, Plugboard, Plugs},
    reflector::Reflectors,
    rotor::{RotorConfig, Rotors},
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use strum::IntoEnumIterator;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    /// Path to config file. If config file exists, that config will be used. Otherwise a config file will be generated
    #[arg(short, long)]
    pub config: Option<String>,

    /// Path to output ciphertext
    #[arg(short, long)]
    pub output: Option<String>,

    /// Path to plaintext input file.
    #[arg(short, long)]
    pub input: Option<String>,

    /// (Optional) Plaintext to encode. Mutually exclusive with -i command line argument
    pub plaintext: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    reflector: Reflectors,
    rotors: [(Rotors, char); 3],
    plugs: Vec<(char, char)>,
}

impl TryFrom<Config> for Enigma {
    type Error = Bruh;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        let rotor_config: RotorConfig = RotorConfig::try_from(value.rotors)?;
        let plugs: Vec<Plug> = value
            .plugs
            .iter()
            .map(|(l, r)| {
                (
                    Character::try_from(*l).unwrap(),
                    Character::try_from(*r).unwrap(),
                )
            })
            .map(|p| Plug::try_from(p))
            .try_collect()?;
        let plugs: Plugs = Plugs::try_from(plugs)?;
        let plugboard = Plugboard::try_from(plugs)?;

        Ok(Enigma::new(rotor_config, plugboard, value.reflector))
    }
}

impl Config {
    pub fn new() -> Config {
        let rotors = Config::get_rotors();
        let reflector = Config::get_reflector();
        let plugs = Config::get_plugs();

        Config {
            reflector: reflector,
            rotors: rotors,
            plugs: plugs,
        }
    }

    fn get_reflector() -> Reflectors {
        println!("Reflector Configuration:");
        let options: Vec<Reflectors> = Reflectors::iter().collect();

        let ans = Select::new("Select Reflector", options).prompt();

        ans.unwrap()
    }

    fn get_rotors() -> [(Rotors, char); 3] {
        println!("Rotor Configuration:");
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

    fn get_plugs() -> Vec<(char, char)> {
        println!("Plugboard Configuration:");
        let num_plugs = {
            let selection: Vec<usize> = (0..=10).into_iter().collect();

            Select::new("How many plugs do you want to use?", selection)
                .prompt()
                .unwrap()
        };

        let mut plugs: Vec<(char, char)> = Vec::new();
        let mut selected_chars: HashSet<char> = HashSet::new();
        let options: Vec<char> = ('A'..='Z').into_iter().collect_vec();
        let validator: ExactLengthValidator = ExactLengthValidator::new(2);

        for _i in 0..num_plugs {
            let ans = MultiSelect::new(
                "Select 2 characters to use for plug",
                options
                    .clone()
                    .into_iter()
                    .filter(|c| !selected_chars.contains(c))
                    .collect_vec(),
            )
            .with_validator(validator.clone())
            .prompt()
            .unwrap();

            for c in ans.iter() {
                selected_chars.insert(*c);
            }

            let plug = (ans[0], ans[1]);
            plugs.push(plug)
        }

        plugs
    }
}
