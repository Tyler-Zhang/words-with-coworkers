table! {
    games (id) {
        id -> Int4,
        board -> Bpchar,
        turn_count -> Int4,
        pieces -> Varchar,
        channel_id -> Varchar,
        player_turn_id -> Nullable<Int4>,
        team_id -> Varchar,
    }
}

table! {
    players (id) {
        id -> Int4,
        game_id -> Int4,
        pieces -> Varchar,
        slack_id -> Varchar,
        points -> Int4,
        team_id -> Varchar,
    }
}

table! {
    teams (id) {
        id -> Varchar,
        team_domain -> Varchar,
        access_token -> Varchar,
        bot_user_access_token -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    games,
    players,
    teams,
);
