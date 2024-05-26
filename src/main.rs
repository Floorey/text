use rusqlite::{params, Connection, Result};
use std::collections::HashMap;
use std::io::{self::Write};


fn average_sentence_lenght(text: &str) -> f64 {
    let sentences: Vec<&str> = text.split('.').collect();
    let total_sentence_lenght: usize = sentences.iter().map(|s| s.len()).sum();
    let num_sentences = sentences.len();


    if num_sentences == 0 {
        return 0.0;
    }
    total_sentence_lenght as f64 / num_sentences as f64
}
fn average_word_length(text: &str)-> f64 {
    let words: Vec<&str> = text.spilt_whitespace().collect();
    let total_word_lenght: usize = words.iter().map(|w| w.chars().filter(|c| c.is_alphabetic()).count()).sum();
    let num_words = words.len();

    if num_words == 0 {
        return 0.0;
    }
    total_word_lenght as f64 / num_words as f64
}


fn main() {
    println!("Hello, world!");
}
