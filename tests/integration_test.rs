extern crate scrabble;
mod common;

use ::scrabble::{Result, Point, Direction, start_game, play_word};
use common::stub_current_player_hand;


#[test]
fn full_game_test() -> Result<()>{
    let mut game = start_game(2);

    stub_current_player_hand(&mut game, "ACTORSEOYK");

    println!("{}", game);
    game = play_word(
        game,
        Point::new(7, 7),
        Direction::right(),
        "ACTOR"
    )?;

    assert_eq!(game.turn, 1);
    assert_eq!(game.has_word_been_played, true);
    assert_eq!(game.players[0].score, 8);

    stub_current_player_hand(&mut game, "BOARSANALS");
    game = play_word(
        game,
        Point::new(10, 6),
        Direction::down(),
        "BOARS"
    )?;

    assert_eq!(game.turn, 2);
    assert_eq!(game.players[1].score, 14);

    game = play_word(
        game,
        Point::new(7, 5),
        Direction::down(),
        "SEA"
    )?;
    assert_eq!(game.players[0].score, 11);

    // Tests that it counts the C above ANAL
    game = play_word(
        game,
        Point::new(8,8),
        Direction::down(),
        "ANAL"
    )?;
    assert_eq!(game.players[1].score, 22);

    game = play_word(
        game,
        Point::new(7,5),
        Direction::right(),
        "SOY"
    )?;
    assert_eq!(game.players[0].score, 25);

    game = play_word(
        game,
        Point::new(12,7),
        Direction::right(),
        "S"
    )?;
    assert_eq!(game.players[1].score, 30);

    Ok(())
}

#[test]
fn no_cover_starting() {
    let mut game = start_game(2);

    stub_current_player_hand(&mut game, "ACTORSEOYK");

    let result = play_word(
        game,
        Point::new(7, 6),
        Direction::right(),
        "ACTOR"
    );

    assert_eq!(result.is_err(), true);
}
