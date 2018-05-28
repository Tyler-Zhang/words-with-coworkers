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
        'J' | 'X' => 1,
        'Q' | 'Z' => 1,
        _ => panic!(format!("Trying to get string for invalid char {}", c))
    }
}

pub fn get_starting_spot(board: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (r, row) in board.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if (*col == '+') {
                return Some((r, c));
            }
        }
    }
    return None;
}


pub fn get_char_from_vec(vec: &Vec<Vec<char>>, x: i32, y: i32) -> Option<char> {
    if y < 0 || y as usize>= vec.len() {
        return None;
    }

    let row = &vec[y as usize];

    if x < 0 || x as usize > row.len() {
        return None;
    } else {
        return Some(row[x as usize]);
    }
}
