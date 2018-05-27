use ::schema::games;

#[changeset_options(treat_none_as_null = "true")]
#[derive(Queryable, Identifiable, AsChangeset, Clone)]
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
