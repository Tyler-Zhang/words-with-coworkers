use ::schema::players;
use super::Game;

#[derive(Queryable, Identifiable, AsChangeset, Associations, PartialEq, Clone, Debug)]
#[table_name="players"]
#[belongs_to(Game)]
pub struct Player {
    pub id: i32,
    pub game_id: i32,
    pub pieces: String,
    pub slack_id: String,
    pub points: i32,
    pub team_id: String
}

#[derive(Insertable, Debug)]
#[table_name="players"]
pub struct NewPlayer<'a> {
    pub game_id: i32,
    pub pieces: &'a str,
    pub slack_id: &'a str,
    pub points: i32,
    pub team_id: &'a str
}
