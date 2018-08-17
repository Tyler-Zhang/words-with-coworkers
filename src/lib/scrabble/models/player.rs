use super::super::utils::string;

pub struct Player {
    pub pieces: String,
    pub score: u32
}

impl Player {
    pub fn new(pieces: String) -> Self {
        Player{ pieces, score: 0 }
    }

    pub fn remove_pieces(&mut self, pieces: &str) -> Result<(), String> {
        self.pieces = string::remove_from_string(&self.pieces, pieces)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_pieces() {
        let mut player = Player::new(format!("ABCDEFGH"));
        assert_eq!(player.remove_pieces("ABDG").is_ok(), true);
        assert_eq!(player.pieces, "CEFH");
    }

    #[test]
    fn test_failed_remove_pieces() {
        let mut player = Player::new(format!("ABCDEFGH"));
        assert_eq!(player.remove_pieces("ABCIJK").is_err(), true);
        assert_eq!(player.pieces, "ABCDEFGH");
    }
}
