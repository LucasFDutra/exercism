use std::collections::HashSet;

pub fn check_word(word: &str, candidate: &str) -> bool {
    let mut word_chars: Vec<char> = word.to_lowercase().chars().collect();
    let mut candidate_chars: Vec<char> = candidate.to_lowercase().chars().collect();

    // pra ser anagrama, as palavras devem ter o mesmo tamanho
    if word_chars.len() != candidate_chars.len() {
        return false;
    }

    // uma palavra não é anagrama dela mesma
    if word_chars == candidate_chars {
        return false;
    }

    // se os vetores ordenados forem iguais, são anagramas
    word_chars.sort();
    candidate_chars.sort();
    if word_chars != candidate_chars {
        return false;
    }
    return true;
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();

    for &candidate in possible_anagrams.iter() {
        if check_word(word, candidate) {
            anagrams.insert(candidate);
        }
    }

    return anagrams;
}
