mod soundex;
use soundex::soundex;

fn main() {
    let word = String::from("Ashcroft");
    println!("{}", soundex(&word));
}
