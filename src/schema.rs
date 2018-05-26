table! {
    games (id) {
        id -> Int4,
        board -> Bpchar,
        turn_count -> Int4,
        pieces -> Varchar,
        channel_id -> Varchar,
        player_turn_id -> Int4,
    }
}

table! {
    players (id) {
        id -> Int4,
        game_id -> Int4,
        pieces -> Varchar,
        slack_id -> Varchar,
        points -> Int4,
    }
}

joinable!(players -> games (game_id));

allow_tables_to_appear_in_same_query!(
    games,
    players,
);
