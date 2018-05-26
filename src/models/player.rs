#[derive(Queryable)]
pub struct Player {
    pub id: i32,
    pub game_id: i32,
    pub pieces: String,
    pub slack_id: String,
    pub points: i32
}
