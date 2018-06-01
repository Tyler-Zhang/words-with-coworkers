use regex::Regex;

pub fn extract_user_ids (text: &str) -> Vec<&str>{
    // Matches text like <@UAX2B1P71|tztylerzhang>
    let re = Regex::new(r"<@(?P<id>\w+)\|\S+>").unwrap();

    re.captures_iter(text).map(|c| c.name("id").unwrap().as_str()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_bare_id() {
        let text = "<@UAX2B1P71|tztylerzhang>";
        assert_eq!(extract_user_ids(text), vec!["UAX2B1P71"]);
    }

    #[test]
    fn extracts_multiple_ids() {
        let text = "<@UAX2B1P71|tztylerzhang> sakfljs <@UASF123|popoi>";
        assert_eq!(extract_user_ids(text), vec!["UAX2B1P71", "UASF123"]);        
    }

    #[test]
    fn extracts_no_ids() {
        let text = "hey there <> @ <@> how's it <skladjf|> going?";
        assert_eq!(extract_user_ids(text), Vec::<&str>::new());
    }

    #[test]
    fn extracts_with_symbols() {
        let text = "hey there my man <@UAX2B1P71|tztylerz,an.g!>";
        assert_eq!(extract_user_ids(text), vec!["UAX2B1P71"]);        
    }
}
