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

impl Tile {
    fn get_multiplier(&self) -> (i32, i32) {
        match self {
            Tile::DoubleWord    => (2, 1),
            Tile::Starting      => (2, 1),
            Tile::TripleWord    => (3, 1),
            Tile::DoubleLetter  => (1, 2),
            Tile::TripleLetter  => (1, 3),
            _                   => (1, 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tile;

    #[test]
    fn from() {
        assert_eq!(Tile::from('@'), Tile::DoubleLetter);
        assert_eq!(Tile::from('#'), Tile::TripleLetter);
        assert_eq!(Tile::from('+'), Tile::Starting);
        assert_eq!(Tile::from('A'), Tile::Letter('A'));
        assert_eq!(Tile::from('B'), Tile::Letter('B'));
        assert_eq!(Tile::from('C'), Tile::Letter('C'));
    }

    #[test]
    fn into() {
        assert_eq!(Into::<char>::into(Tile::DoubleLetter), '@');
        assert_eq!(Into::<char>::into(Tile::TripleLetter), '#');
        assert_eq!(Into::<char>::into(Tile::Starting), '+');
        assert_eq!(Into::<char>::into(Tile::Letter('A')), 'A');
        assert_eq!(Into::<char>::into(Tile::Letter('B')), 'B');
        assert_eq!(Into::<char>::into(Tile::Letter('C')), 'C');
    }

    #[test]
    fn get_multiplier() {
        assert_eq!(Tile::DoubleLetter.get_multiplier(), (1, 2));
        assert_eq!(Tile::DoubleWord.get_multiplier(), (2, 1));
        assert_eq!(Tile::Letter('A').get_multiplier(), (1, 1));
    }
}
