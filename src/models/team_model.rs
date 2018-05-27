use ::schema::teams;

#[derive(Queryable, Clone, Identifiable, AsChangeset, Associations)]
pub struct Team {
    pub id: String,
    pub team_domain: String,
    pub access_token: String,
    pub bot_user_access_token: String
}

#[derive(Insertable)]
#[table_name="teams"]
pub struct NewTeam<'a> {
    pub id: &'a str,
    pub team_domain: &'a str,
    pub access_token: &'a str,
    pub bot_user_access_token: &'a str
}
