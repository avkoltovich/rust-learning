pub fn get_pig_latin_string(string: &str) {
    let mut word = String::from(string);
    let first_letter = word.remove(0);
    let final_word: String;

    if is_vowel(&first_letter) {
        final_word = format!("{}{}-{}", first_letter, word, "hay");
    } else {
        final_word = format!("{}-{}{}", word, first_letter, "ay");
    }

    println!("Слово в pig latin кодировке: {}", final_word);
}

pub fn is_vowel(c: &char) -> bool {
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'y'];

    let is_vowel = vowels.contains(c);
    is_vowel
}