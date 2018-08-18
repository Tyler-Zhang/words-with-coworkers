use super::super::models::{Board, Word, Tile, Point};

pub fn extend_word(board: &Board, word: &mut Word) {
    let mut letter_chars: Vec<char> = word.letters.clone();

    let mut backwards_extend_length: i32 = 0;

    // Extend backwards first
    board.iterate_while(word.start - word.direction, -word.direction, |tile: &Tile| -> bool {
        if let &Tile::Letter(c) = tile {
            letter_chars.insert(0, c);
            backwards_extend_length += 1;
            return true;
        }

        false
    });

    // Extend forwards
    board.iterate_while(word.get_end() + word.direction, word.direction, |tile: &Tile| -> bool {
        if let &Tile::Letter(c) = tile {
            letter_chars.push(c);
            return true;
        }

        false
    });

    word.letters = letter_chars;
    word.start = word.start - (word.direction * backwards_extend_length);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extend_word() {
        let board = Board::from("\
        .....\
        .....\
        ..B..\
        ..C..\
        .....\
        ");

        let mut word = Word::new(format!("EF"), (2, 0), true);

        extend_word(&board, &mut word);

        assert_eq!(word.letters.iter().collect::<String>(), "EFBC");
        assert_eq!(word.start, Point::new(2, 0));
        assert_eq!(word.get_end(), Point::new(2, 3));
    }
}
