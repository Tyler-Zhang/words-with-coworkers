use scrabble::{Game, Tile};

pub fn stub_current_player_hand(game: &mut Game, hand: &str) {
    let current_player = game.get_current_player();

    assert_eq!(hand.len(), current_player.hand.len());

    current_player.hand = hand.chars().map(|c| Tile::from(c)).collect();
}
