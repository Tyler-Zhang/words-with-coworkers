use std::cmp::Ordering;
use super::super::utils;

#[derive(PartialEq, Eq, Debug)]
pub struct Letters {
    pub string: String
}

impl PartialOrd for Letters {
    fn partial_cmp(&self, other: &Letters) -> Option<Ordering> {
        let mut own_letter_map = utils::string::string_to_hashmap(&self.string);

        for c in other.string.chars() {
            if !own_letter_map.contains_key(&c) || own_letter_map[&c] <= 0{
                return Some(Ordering::Less);
            }

            *own_letter_map.get_mut(&c).unwrap() -= 1;
        }

        for (_key, val) in own_letter_map.iter() {
            if *val > 0 {
                return Some(Ordering::Greater);
            }
        }

        Some(Ordering::Equal)
    }
}

impl Letters {
    pub fn new(string: String) -> Self {
        Self { string }
    }

    pub fn remove_from(&mut self, letters: &Letters) -> Result<(), String> {
        if *self < *letters {
            return Err(format!("Not enough letters to remove"));
        }

        self.string = utils::string::remove_from_string(&self.string, &letters.string).unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_partial_cmp() {
        let letters_1 = Letters::new(format!("ASFASFS"));
        let letters_2 = Letters::new(format!("FAA"));

        assert_eq!(letters_1 > letters_2, true);
        assert_eq!(letters_1 < letters_2, false);
    }

    #[test]
    pub fn test_remove_from() {
        let mut letters = Letters::new(format!("ABCDEF"));

        assert_eq!(letters.remove_from(&Letters::new(format!("BCF"))).is_ok(), true);
        assert_eq!(
            letters,
            Letters::new(format!("ADE"))
        );
    }
}
