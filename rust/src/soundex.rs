use std::collections::HashMap;

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
    return str_ensure_with_size(4, &(first_letter + &rm_vowels(
        &unique_str(
            &rm_hw(&suffix).chars().map(|c| tr_char(c, &map)).collect()
        )
    )))
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

fn str_ensure_with_size(n: usize, string: &String) -> String {
    if string.len() > n {
        string.chars().take(n).collect()
    } else {
        format!("{}{}", string, "0".to_string().repeat(n - string.len()))
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;
    use hamcrest::prelude::*;

    describe! rm_hw {
        it "removes h and w in any case" {
            assert_that!(&rm_hw(&"haHbwcWd".to_string()), is(equal_to("abcd")));
        }
    }

    describe! rm_vowels {
        it "removes them in any case" {
            assert_that!(&rm_vowels(&"aAbcdiI".to_string()), is(equal_to("bcd")));
        }
    }

    describe! tr_char {
        it "translates given letter using given map" {
            let map: HashMap<_, _> = [('a', '1'), ('b', '2')]
                .iter().cloned().collect();

            let new_val = tr_char('b', &map);

            assert_that!(new_val, is(equal_to('2')));
        }

        it "returns the same character if it does not exist in the dictionary" {
            let map: HashMap<_, _> = [('a', '1'), ('b', '2')]
                .iter().cloned().collect();

            let new_val = tr_char('c', &map);

            assert_that!(new_val, is(equal_to('c')));
        }
    }

    describe! unique_str {
        it "returns string with no repeating adjacent characters" {
            let new_str = unique_str(&"aabccdee".to_string());

            assert_that!(&new_str, is(equal_to("abcde")));
        }
    }

    describe! soundex {
        it "returns max 4 symbols" {
            assert_that!(&soundex(&"Abcdl".to_string()), is(equal_to("A123")));
        }

        it "appends zeroes when string is too short" {
            assert_that!(&soundex(&"Abuie".to_string()), is(equal_to("A100")));
        }

        it "encodes word using soundex algorithm" {
            assert_that!(&soundex(&"Ashcraft".to_string()), is(equal_to("A261")));
            assert_that!(&soundex(&"Ashcroft".to_string()), is(equal_to("A261")));
            assert_that!(&soundex(&"Tymczak".to_string()), is(equal_to("T522")));
            // TODO
            //assert!(soundex("Pfister".to_string()) == "P263");
        }
    }

    describe! str_ensure_with_size {
        describe! when_string_is_shorter_than_desired_size {
            it "returns string with 0oes appended to ensure proper size" {
                assert_that!(str_ensure_with_size(4, &"ab".to_string()),
                             is(equal_to("ab00".to_string())));
            }
        }

        describe! when_string_is_exactly_the_same_size_as_desired {
            it "returns the same string" {
                assert_that!(str_ensure_with_size(4, &"abcd".to_string()),
                             is(equal_to("abcd".to_string())));
            }
        }

        describe! when_string_is_longer_than_desired_size {
            it "returns trimmed string" {
                assert_that!(str_ensure_with_size(4, &"abcdef".to_string()),
                             is(equal_to("abcd".to_string())));
            }
        }
    }
}
