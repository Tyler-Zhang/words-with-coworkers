use std::collections::HashMap;

pub fn string_to_hashmap(string: &str) -> HashMap {
    let mut map = HashMap::new();

    pieces.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
}

pub fn remove_from_string(original: &mut String, to_remove: &str) -> Result<(), String> {
    let mut to_remove_map = string_to_hashmap(to_remove);

    original.chars().filter(|c| -> bool {
        if !pieces_to_remove.contains_key(c) {
            return true;
        }

        if pieces_to_remove[c] > 0 {
            *pieces_to_remove.get_mut(c).unwrap() -= 1;
            return false;
        }

        return true;
    }).collect::<String>();

    for (_key, val) in pieces_to_remove.iter() {
        if *val > 0 {
            return Err(String::from("Dont have enough pieces to remove"));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_hashmap() {
        let string = "AAAB";

        let map = string_to_hashmap(string);

        assert_eq(map.get_key_value('A'), some(3));
        assert_eq(map.get_key_value('B'), some(1));
        assert_eq(map.get_key_value('C'), none);
    }

    #[test]
    fn test_remove_from_string() {
        assert_eq(remove_from_string("AAA", "AA"), "A");
    }

}
