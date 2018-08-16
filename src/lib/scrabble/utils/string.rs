use std::collections::HashMap;

pub fn string_to_hashmap(string: &str) -> HashMap<char, i32> {
    let mut map = HashMap::new();

    string.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);

    map
}

pub fn remove_from_string(original: &str, to_remove: &str) -> Result<String, String> {
    let mut to_remove_map = string_to_hashmap(to_remove);

    let new_string = original.chars().filter(|c| -> bool {
        if !to_remove_map.contains_key(c) {
            return true;
        }

        if to_remove_map[c] > 0 {
            *to_remove_map.get_mut(c).unwrap() -= 1;
            return false;
        }

        return true;
    }).collect::<String>();

    for (_key, val) in to_remove_map.iter() {
        if *val > 0 {
            return Err(String::from("Dont have enough pieces to remove"));
        }
    }

    Ok(new_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_hashmap() {
        let string = "AAAB";

        let map = string_to_hashmap(string);

        assert_eq!(map.get(& 'A'), Some(&3));
        assert_eq!(map.get(& 'B'), Some(&1));
        assert_eq!(map.contains_key(& 'C'), false);
    }

    #[test]
    fn test_remove_from_string() {
        assert_eq!(remove_from_string("ABC", "ABC").unwrap(), "");
        assert_eq!(remove_from_string("ABCD", "BC").unwrap(), "AD");
    }

    #[test]
    fn test_bad_remove_from_string() {
        assert_eq!(remove_from_string("AB", "E").is_err(), true);
    }

}
