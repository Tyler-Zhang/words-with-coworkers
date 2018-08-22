use std::collections::HashSet;
use std::io::{BufReader, BufRead};
use std::fs::File;

pub struct ScrabbleDictionary {
    pub words: HashSet<String>
}

impl ScrabbleDictionary {
    pub fn new(path: &str) -> ScrabbleDictionary {
        println!("Building hashset");
        let mut words = HashSet::new();

        let f = File::open(path).expect("File not found");
        let file = BufReader::new(&f);

        for (_num, line) in file.lines().enumerate() {
            words.insert(line.unwrap());
        }

        return ScrabbleDictionary { words: words }
    }

    pub fn is_word_valid(&self, word: &str) -> bool {
        self.words.contains(word)
    }
}
