#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

mod soundex;
use soundex::soundex;

fn main() {
    let word = String::from("Ashcroft");
    println!("{}", soundex(&word));
}
