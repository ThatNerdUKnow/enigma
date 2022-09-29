mod common;

use common::Character;


fn main() {
   ('A'..='z').into_iter().for_each(|c|
    match Character::try_from(c){
        Ok(output) => println!("{}:{:?}",c,output),
        Err(message) => println!("{},{}",c,message),
    }
)
}
