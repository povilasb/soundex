use std::collections::HashMap;

// TODO: use rustfmt
// TODO: use more informative matchers

pub fn soundex(word: &String) -> String {
    let map: HashMap<_, _> = [
            ('b', '1'), ('f', '1'), ('p', '1'), ('v' , '1'),
            ('c', '2'), ('g', '2'), ('j', '2'), ('k', '2'), ('q', '2'),
            ('s', '2'), ('x', '2'), ('z', '2'),
            ('d', '3'), ('t', '3'),
            ('l', '4'), ('m', '5'), ('n', '5'), ('r', '6'),
        ].iter().cloned().collect();
    let suffix: String = word.chars().skip(1).collect();
    let first_letter: String = word.chars().take(1).collect();
    return (first_letter + &rm_vowels(
        &unique_str(
            &rm_hw(&suffix).chars().map(|c| tr_char(c, &map)).collect()
        )
    )).chars().take(4).collect();
}

// TODO: return iterator instead of string.
// Now I couldn't make the function return an iterator...
fn remove_letters_(word: &String, rm_us: &[char]) -> String {
    return word.chars().filter(|x| !rm_us.contains(x)).collect()
}

fn rm_hw(word: &String) -> String {
    return remove_letters_(&word, &['h', 'H', 'w', 'W']);
}

fn rm_vowels(word: &String) -> String {
    return remove_letters_(&word, &['a', 'e', 'i', 'o', 'u', 'y', 'h',
        'A', 'E', 'I', 'O', 'U', 'Y', 'H']);
}

fn tr_char(letter: char, map: &HashMap<char, char>) -> char {
    match map.get(&letter) {
        Some(letter) => return *letter,
        None => return letter
    }
}

fn unique_str(word: &String) -> String {
    let mut uniq = "".to_string();
    let mut last_char = '\0';
    for c in word.chars() {
        if c != last_char {
            uniq.push(c);
        }
        last_char = c;
    }
    return uniq;
}

#[cfg(test)]
mod tests {
    pub use super::*;

    describe! rm_hw {
        it "removes h and w in any case" {
            assert!(rm_hw(&"haHbwcWd".to_string()) == "abcd");
        }
    }

    describe! rm_vowels {
        it "removes them in any case" {
            assert!(rm_vowels(&"aAbcdiI".to_string()) == "bcd");
        }
    }

    describe! tr_char {
        it "translates given letter using given map" {
            let map: HashMap<_, _> = [('a', '1'), ('b', '2')]
                .iter().cloned().collect();

            let new_val = tr_char('b', &map);

            assert!(new_val == '2');
        }

        it "returns the same character if it does not exist in the dictionary" {
            let map: HashMap<_, _> = [('a', '1'), ('b', '2')]
                .iter().cloned().collect();

            let new_val = tr_char('c', &map);

            assert!(new_val == 'c');
        }
    }

    describe! unique_str {
        it "returns string with no repeating adjacent characters" {
            let new_str = unique_str(&"aabccdee".to_string());

            assert!(new_str == "abcde");
        }
    }

    describe! soundex {
        it "returns max 4 symbols" {
            assert!(soundex(&"Abcdl".to_string()) == "A123");
        }

        it "appends zeroes when string is too short" {
            // TODO
            //assert!(soundex("Abuie".to_string()) == "A100");
        }

        it "encodes word using soundex algorithm" {
            assert!(soundex(&"Ashcraft".to_string()) == "A261");
            assert!(soundex(&"Ashcroft".to_string()) == "A261");
            assert!(soundex(&"Tymczak".to_string()) == "T522");
            // TODO
            //assert!(soundex("Pfister".to_string()) == "P263");
        }
    }
}
