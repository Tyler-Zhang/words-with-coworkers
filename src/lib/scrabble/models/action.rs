use super::Word;

pub struct Action {
    pub word: Word,
    pub log: Vec<String>
}

impl Action {
    pub fn new(word: Word) -> Self {
        Self {
            word,
            log: Vec::new()
        }
    }
}
