use super::checkers::is_char_letter;

pub fn extend_word_both_dir(board: &Vec<Vec<char>>, start: (i32, i32), horizontal: bool, c: char) -> (String, (i32, i32)) {
    let (delta_x, delta_y) = if horizontal { (1, 0) } else { (0, 1) };

    let backwards = extend_word(board, start, (delta_x * -1, delta_y * -1));
    let forwards = extend_word(board, start, (delta_x, delta_y));

    let mut full_word = backwards.0.chars().rev().collect::<String>();
    full_word.push(c);
    full_word.push_str(&forwards.0);

    (full_word, backwards.1)
}

/**
 * Returns the word that the extended and where the word starts
 */
pub fn extend_word(board: &Vec<Vec<char>>, start: (i32, i32), direction: (i32, i32)) -> (String, (i32, i32)) {
    let (mut x, mut y) = start;

    let mut word = String::new();

    loop {
        let next_letter = get_char_from_vec(board, x + direction.0, y + direction.1);

        if next_letter.is_none() || !is_char_letter(next_letter.unwrap()) {
            break;
        }

        x += direction.0;
        y += direction.1;
        word.push(next_letter.unwrap());
    }

    (word, (x, y))
}

pub fn get_multiplier (c: char) -> (i32, i32) {
    match c {
        '2' => (2, 1),
        '3' => (3, 1),
        '@' => (1, 2),
        '#' => (1, 3),
        _ => (1, 1)
    }
}

pub fn get_char_score (c: char) -> i32 {
    match c {
        'E' | 'A' | 'I' | 'O' | 'N' | 'R' | 'T' | 'L' | 'S' | 'U' => 1,
        'D' | 'G'  => 2,
        'B' | 'C' | 'M' | 'P' => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'K' => 5,
        'J' | 'X' => 8,
        'Q' | 'Z' => 10,
        _ => panic!(format!("Trying to get string for invalid char {}", c))
    }
}

pub fn get_starting_spot(board: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (r, row) in board.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col == '+' {
                return Some((r, c));
            }
        }
    }
    return None;
}


pub fn get_char_from_vec(vec: &Vec<Vec<char>>, x: i32, y: i32) -> Option<char> {
    if y < 0 || y as usize >= vec.len() {
        return None;
    }

    let row = &vec[y as usize];

    if x < 0 || x as usize >= row.len() {
        return None;
    } else {
        return Some(row[x as usize]);
    }
}
