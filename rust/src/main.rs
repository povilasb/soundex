use std::collections::HashMap;

// TODO: use rustfmt
// TODO: use more informative matchers


// TODO: return iterator instead of string.
// Now I couldn't make the function return an iterator...
fn remove_letters_(word: String, rm_us: &[char]) -> String {
    return word.chars().filter(|x| !rm_us.contains(x)).collect()
}

fn rm_hw(word: String) -> String {
    return remove_letters_(word, &['h', 'H', 'w', 'W']);
}

fn rm_vowels(word: String) -> String {
    return remove_letters_(word, &['a', 'e', 'i', 'o', 'u', 'y', 'h',
        'A', 'E', 'I', 'O', 'U', 'Y', 'H']);
}

fn tr_char(letter: char, map: &HashMap<char, char>) -> char {
    match map.get(&letter) {
        Some(letter) => return *letter,
        None => return letter
    }
}

fn unique_str(word: String) -> String {
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

fn soundex(word: String) -> String {
    let map: HashMap<_, _> = [
            ('b', '1'), ('f', '1'), ('p', '1'), ('v' , '1'),
            ('c', '2'), ('g', '2'), ('j', '2'), ('k', '2'), ('q', '2'),
            ('s', '2'), ('x', '2'), ('z', '2'),
            ('d', '3'), ('t', '3'),
            ('l', '4'), ('m', '5'), ('n', '5'), ('r', '6'),
        ].iter().cloned().collect();
    let suffix = word.chars().skip(1).collect();
    let first_letter: String = word.chars().take(1).collect();
    return (first_letter + &rm_vowels(
        unique_str(
            rm_hw(suffix).chars().map(|c| tr_char(c, &map)).collect()
        )
    )).chars().take(4).collect();
}

fn main() {
    let word = String::from("Ashcroft");
    println!("{}", soundex(word));
}

#[test]
fn rm_hw_removes_h_and_w_in_any_case() {
    assert!(rm_hw("haHbwcWd".to_string()) == "abcd")
}

#[test]
fn rm_vowels_removes_them_in_any_case() {
    assert!(rm_vowels("aAbcdiI".to_string()) == "bcd")
}

#[test]
fn tr_char_translates_given_letter_using_given_map() {
    let map: HashMap<_, _> = [('a', '1'), ('b', '2')]
        .iter().cloned().collect();

    let new_val = tr_char('b', &map);

    assert!(new_val == '2')
}

#[test]
fn tr_char_returns_the_same_character_if_it_does_not_exist_in_the_dictionary() {
    let map: HashMap<_, _> = [('a', '1'), ('b', '2')]
        .iter().cloned().collect();

    let new_val = tr_char('c', &map);

    assert!(new_val == 'c')
}

#[test]
fn unique_str_returns_string_with_no_repeating_adjacent_characters() {
    let new_str = unique_str("aabccdee".to_string());

    assert!(new_str == "abcde");
}

#[test]
fn soundex_returns_max_4_symbols() {
    assert!(soundex("Abcdl".to_string()) == "A123");
}

#[test]
fn soundex_appends_zeroes_when_string_is_too_short() {
    // TODO
    //assert!(soundex("Abuie".to_string()) == "A100");
}

#[test]
fn soundex_encodes_word_using_soundex_algorithm() {
    assert!(soundex("Ashcraft".to_string()) == "A261");
    assert!(soundex("Ashcroft".to_string()) == "A261");
    assert!(soundex("Tymczak".to_string()) == "T522");
    // TODO
    //assert!(soundex("Pfister".to_string()) == "P263");
}
