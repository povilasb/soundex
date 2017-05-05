from functools import reduce
from typing import Iterable
from itertools import filterfalse


CONSONANTS_DIGITS = {
    'b': '1',
    'f': '1',
    'p': '1',
    'v': '1',
    'c': '2',
    'g': '2',
    'j': '2',
    'k': '2',
    'q': '2',
    's': '2',
    'x': '2',
    'z': '2',
    'd': '3',
    't': '3',
    'l': '4',
    'm': '5',
    'n': '5',
    'r': '6',
}


def main():
    original_word = 'Ashcraft'
    print(soundex_word(original_word))


def consonants_to_digits(word) -> Iterable[str]:
    return map(
        lambda letter:
            CONSONANTS_DIGITS[letter] if letter in CONSONANTS_DIGITS else letter,
        word,
    )


def filter_out(word, exclude_letters: Iterable[str]):
    return filterfalse(lambda letter: letter in exclude_letters, word)


def merge_adjacent(intermediate) -> str:
    intermediate = list(intermediate)
    last_symbol = intermediate[1]
    result = intermediate[0] + last_symbol

    for symbol in intermediate[1:]:
        if symbol != last_symbol:
            result += symbol
        last_symbol = symbol

    return result


def iterable_to_str(iterable):
    return reduce(lambda result, letter: result + letter, iterable, '')


def remove_vowels(word):
    return iterable_to_str(
        filter(lambda letter: ord(letter) in range(0x30, 0x40), word)
    )


def min_size_str(string, min_size):
    if len(string) > min_size:
        return string[:min_size]
    else:
        return string + 0 * (min_size - len(string))


def soundex_word(word):
    return min_size_str(
        word[0] + remove_vowels(
            merge_adjacent(
                filter_out(consonants_to_digits(word), 'hw')
            )
        ),
        4,
    )


if __name__ == '__main__':
    main()
