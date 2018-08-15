#[derive(PartialEq, Debug)]
pub enum Tile {
    Empty,
    DoubleWord,
    TripleWord,
    DoubleLetter,
    TripleLetter,
    Starting,
    Letter(char)
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '2' => Tile::DoubleWord,
            '3' => Tile::TripleWord,
            '@' => Tile::DoubleLetter,
            '#' => Tile::TripleLetter,
            '+' => Tile::Starting,
             _  => Tile::Letter(c)
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Tile::Empty         => '.',
            Tile::DoubleWord    => '2',
            Tile::TripleWord    => '3',
            Tile::DoubleLetter  => '@',
            Tile::TripleLetter  => '#',
            Tile::Starting      => '+',
            Tile::Letter(c)     => c
        }
    }
}
