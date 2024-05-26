use rusqlite::{params, Connection, Result};
use std::collections::HashMap;
use std::io::{self, Write};

fn average_sentence_length(text: &str) -> f64 {
    let sentences: Vec<&str> = text.split('.').collect();
    let total_sentence_length: usize = sentences.iter().map(|s| s.len()).sum();
    let num_sentences = sentences.len();

    if num_sentences == 0 {
        return 0.0;
    }
    total_sentence_length as f64 / num_sentences as f64
}

fn average_word_length(text: &str) -> f64 {
    let words: Vec<&str> = text.split_whitespace().collect();
    let total_word_length: usize = words.iter().map(|w| w.chars().filter(|c| c.is_alphabetic()).count()).sum();
    let num_words = words.len();

    if num_words == 0 {
        return 0.0;
    }
    total_word_length as f64 / num_words as f64
}

fn count_letters(text: &str) -> HashMap<char, i32> {
    let mut letter_count = HashMap::new();
    for c in text.chars() {
        if c.is_alphabetic() {
            let counter = letter_count.entry(c.to_ascii_lowercase()).or_insert(0);
            *counter += 1;
        }
    }
    letter_count
}

fn find_most_frequent_letters(text: &str, num_letters: usize) -> String {
    let letter_count = count_letters(text);
    let mut letter_vec: Vec<(&char, &i32)> = letter_count.iter().collect();
    letter_vec.sort_by(|a, b| b.1.cmp(a.1));
    letter_vec.iter().take(num_letters).map(|(&c, _)| c).collect()
}

fn analyze_text(text: &str) -> (usize, usize, Vec<(String, i32)>, f64) {
    let num_sentences = text.matches('.').count();
    let words: Vec<&str> = text.split_whitespace().collect();
    let num_words = words.len();

    let mut word_count = HashMap::new();
    for word in &words {
        let word = word.trim_matches(|c: char| !c.is_alphabetic()).to_ascii_lowercase();
        let counter = word_count.entry(word).or_insert(0);
        *counter += 1;
    }

    let mut most_common_words: Vec<(&String, &i32)> = word_count.iter().collect();
    most_common_words.sort_by(|a, b| b.1.cmp(a.1));
    let top_5_words: Vec<(String, i32)> = most_common_words.into_iter().take(5).map(|(w, c)| (w.clone(), *c)).collect();

    let total_word_length: usize = word_count.keys().map(|w| w.len()).sum();
    let average_word_length = total_word_length as f64 / num_words as f64;

    println!("Analysis Results:");
    println!("Number of sentences: {}", num_sentences);
    println!("Number of words: {}", num_words);
    println!("Most common words:");
    for (word, count) in &top_5_words {
        println!("{} (occurs {} times)", word, count);
    }
    println!("Average word length: {:.2}", average_word_length);

    (num_sentences, num_words, top_5_words, average_word_length)
}

fn count_bytes(text: &str) -> usize {
    text.len()
}

fn count_vowels_and_consonants(text: &str) -> (i32, i32) {
    let mut num_vowels = 0;
    let mut num_consonants = 0;
    for c in text.chars() {
        if c.is_alphabetic() {
            let c = c.to_ascii_lowercase();
            if "aeiou".contains(c) {
                num_vowels += 1;
            } else {
                num_consonants += 1;
            }
        }
    }
    (num_vowels, num_consonants)
}

fn create_database(db_name: &str) -> Result<()> {
    let conn = Connection::open(db_name)?;
    println!("Opened database successfully");
    Ok(())
}

fn create_text_table(db_name: &str, table_name: &str) -> Result<()> {
    let conn = Connection::open(db_name)?;
    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id INTEGER PRIMARY KEY,
                content TEXT NOT NULL,
                num_sentences INTEGER,
                num_words INTEGER,
                most_common_words TEXT,
                average_word_length REAL
            )",
            table_name
        ),
        params![],
    )?;
    println!("Table created successfully");
    Ok(())
}

fn store_analysis_results(
    db_name: &str,
    table_name: &str,
    text: &str,
    num_sentences: usize,
    num_words: usize,
    most_common_words: Vec<(String, i32)>,
    average_word_length: f64,
) -> Result<()> {
    let conn = Connection::open(db_name)?;
    let most_common_words_str = most_common_words
        .iter()
        .map(|(word, count)| format!("{} (occurs {} times)", word, count))
        .collect::<Vec<String>>()
        .join(", ");

    conn.execute(
        &format!(
            "INSERT INTO {} (content, num_sentences, num_words, most_common_words, average_word_length) VALUES (?1, ?2, ?3, ?4, ?5)",
            table_name
        ),
        params![
            text,
            num_sentences as i64,
            num_words as i64,
            most_common_words_str,
            average_word_length
        ],
    )?;
    println!("Analysis results stored successfully");
    Ok(())
}

fn main() -> Result<()> {
    let mut text = String::new();
    println!("Please enter a text:");
    io::stdin().read_line(&mut text).expect("Failed to read line");

    let (num_sentences, num_words, top_5_words, avg_word_len) = analyze_text(&text);

    let most_frequent_letters = find_most_frequent_letters(&text, 5);
    println!("Most frequent letter(s): {}", most_frequent_letters);

    let num_bytes = count_bytes(&text);
    println!("Number of bytes in the text: {}", num_bytes);

    let (num_vowels, num_consonants) = count_vowels_and_consonants(&text);
    println!("Number of vowels in the text: {}", num_vowels);
    println!("Number of consonants in the text: {}", num_consonants);

    let avg_sentence_len = average_sentence_length(&text);
    println!("Average sentence length: {:.2} characters per sentence.", avg_sentence_len);
    println!("Average word length: {:.2} characters per word.", avg_word_len);

    // Create Database
    let db_name = "text_analysis.db";
    create_database(db_name)?;

    let table_name = "TextData";
    create_text_table(db_name, table_name)?;

    store_analysis_results(
        db_name,
        table_name,
        &text,
        num_sentences,
        num_words,
        top_5_words,
        avg_word_len,
    )?;

    Ok(())
}
