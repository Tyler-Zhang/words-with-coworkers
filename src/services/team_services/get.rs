use diesel::PgConnection;
use diesel::prelude::*;
use ::models::Team;

pub fn get(conn: &PgConnection, team_id_query: &str) -> Option<Team> {
    use ::schema::teams::dsl::*;

    let matched_teams = teams.filter(id.eq(team_id_query))
        .limit(1)
        .load::<Team>(conn)
        .expect("Error loading teams");
    
    if matched_teams.is_empty() {
        None
    } else {
        Some(matched_teams[0].clone())
    }
}
