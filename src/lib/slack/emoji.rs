pub enum ScrabbleBoardTile {
    Board,
    DoubleWord,
    TripleWord,
    DoubleLetter,
    TripleLetter,
    Start
}

impl ScrabbleBoardTile {
    pub fn as_str(&self) -> &'static str {
        match self {
            &ScrabbleBoardTile::Board => "board",
            &ScrabbleBoardTile::DoubleWord => "double_word",
            &ScrabbleBoardTile::TripleWord => "triple_word",
            &ScrabbleBoardTile::DoubleLetter => "double_letter",
            &ScrabbleBoardTile::TripleLetter => "triple_letter"
        }
    }
}

impl ToString for ScrabbleBoardTile {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

pub enum Emoji {
    Number(i32),
    ScrabbleLetter(char),
    ScrabbleBoardTile(ScrabbleBoardTile),
    WhiteSquare
}

impl ToString for Emoji {
    fn to_string(&self) -> String {
        match self {
            Emoji::Number(num) => format!(":{}:", number_to_word(num)),
            Emoji::ScrabbleLetter(c) => {
                assert!(c >= & 'A' && c <= & 'Z');
                format!(":scrabble-{}:", c)
            },
            Emoji::ScrabbleBoardTile(scrabble_tile) => format!(":scrabble_{}:", scrabble_tile.as_str()),
            Emoji::WhiteSquare => format!(":white_square:")
        }
    }
}

fn number_to_word (num: &i32) -> &'static str {
    match num {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => panic!(format!("{} too high", num))
    }
}

