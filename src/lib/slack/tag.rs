pub struct Tag<'a>(pub &'a str);

impl<'a> ToString for Tag<'a> {
    fn to_string(&self) -> String {
        format!("<@{}>", self.0)
    }
}
