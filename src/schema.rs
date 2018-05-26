table! {
    games (id) {
        id -> Int4,
        board -> Bpchar,
        board_width -> Int4,
        board_height -> Int4,
        turn_count -> Int4,
        pieces -> Varchar,
        channel_id -> Varchar,
        player_turn_id -> Nullable<Int4>,
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

allow_tables_to_appear_in_same_query!(
    games,
    players,
);
