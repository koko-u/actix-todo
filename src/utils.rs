pub trait AsOption: Into<String> {
    fn into_option(self) -> Option<String>;
}
impl<'a> AsOption for &'a str {
    fn into_option(self) -> Option<String> {
        let s: String = self.into();
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }
}
