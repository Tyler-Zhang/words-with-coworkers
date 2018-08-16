use ::schema::games;

#[changeset_options(treat_none_as_null = "true")]
#[derive(Queryable, Identifiable, AsChangeset, Clone, Debug)]
pub struct Game {
    pub id: i32,
    pub board: String,
    pub board_width: i32,
    pub board_height: i32,
    pub turn_count: i32,
    pub pieces: String,
    pub channel_id: String,
    pub player_turn_id: Option<i32>,
    pub team_id: String
}

#[derive(Insertable)]
#[table_name="games"]
pub struct NewGame<'a> {
    pub board: &'a str,
    pub board_width: i32,
    pub board_height: i32,
    pub turn_count: i32,
    pub pieces: &'a str,
    pub channel_id: &'a str,
    pub player_turn_id: Option<i32>,
    pub team_id: &'a str
}

impl Game {
    pub fn board_to_vec (&self) -> Vec<Vec<char>> {
    let mut main_vec = Vec::new();

    for i in 0..(self.board_height as usize) {
        main_vec.push(
            self.board[(i * self.board_width as usize)..(i + 1) * self.board_width as usize]
                .to_owned()
                .into_bytes()
                .into_iter()
                .map(|c| c as char)
                .collect()
        )
    }

    main_vec
    }

    pub fn set_board_from_vec (&mut self, vec: Vec<Vec<char>>) {
        assert_eq!(vec.len(), self.board_height as usize);
        assert_eq!(vec[0].len(), self.board_height as usize);
        
        let mut new_board = String::new();

        for row in vec {
            for c in row {
                new_board.push(c);
            }
        }

        self.board = new_board;
    }
}
