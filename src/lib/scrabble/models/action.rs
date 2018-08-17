pub struct Action<'a> {
    pub word: &'a str,
    pub start: (u32, u32),
    pub direction: (u32, u32),
    pub direction_down: bool,
    pub log: Vec<String>
}

impl<'a> Action<'a> {
    pub fn new(word: &'a str, start: (u32, u32), direction_down: bool) -> Action<'a> {
        Action {
            word,
            start,
            direction: if direction_down { (0, 1) } else { (1, 0) },
            direction_down,
            log: Vec::new()
        }
    }
}
