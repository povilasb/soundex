#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]
#[cfg(test)] #[macro_use] extern crate hamcrest;

mod soundex;
use soundex::soundex;

fn main() {
    let word = String::from("Ashcroft");
    println!("{}", soundex(&word));
}
